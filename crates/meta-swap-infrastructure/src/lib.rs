#![forbid(unsafe_code)]
//! PostgreSQL adapters. Financial writes use explicit database transactions.

use meta_swap_application::{
    IntentPersistenceError, IntentRepository, PersistIntentRequest, PersistedIntent,
};
use sqlx::{Acquire, PgPool, Postgres, Transaction};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub struct PostgresIntentRepository {
    pool: PgPool,
}

impl PostgresIntentRepository {
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// # Errors
    ///
    /// Returns a database error when the migration cannot be applied.
    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        MIGRATOR.run(&self.pool).await
    }
}

impl IntentRepository for PostgresIntentRepository {
    async fn persist_intent(
        &mut self,
        request: PersistIntentRequest,
    ) -> Result<PersistedIntent, IntentPersistenceError> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| IntentPersistenceError::StorageFailure)?;
        let inserted = sqlx::query(
            "INSERT INTO intent_idempotency (swap_id, idempotency_key, intent_id) \
             VALUES ($1::uuid, $2, $3::uuid) ON CONFLICT DO NOTHING",
        )
        .bind(&request.swap_id)
        .bind(&request.idempotency_key)
        .bind(request.intent.id())
        .execute(&mut *transaction)
        .await
        .map_err(|_| IntentPersistenceError::StorageFailure)?;
        if inserted.rows_affected() == 0 {
            return Err(IntentPersistenceError::DuplicateIdempotencyKey);
        }

        let updated = sqlx::query(
            "UPDATE swap_sessions SET state = 'awaiting_wallet_approval', version = version + 1 \
             WHERE id = $1::uuid AND version = $2 AND state = 'building_intent'",
        )
        .bind(&request.swap_id)
        .bind(
            i64::try_from(request.expected_swap_version)
                .map_err(|_| IntentPersistenceError::VersionConflict)?,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| IntentPersistenceError::StorageFailure)?;
        if updated.rows_affected() == 0 {
            transaction
                .rollback()
                .await
                .map_err(|_| IntentPersistenceError::StorageFailure)?;
            return Err(IntentPersistenceError::VersionConflict);
        }

        persist_records(&mut transaction, &request).await?;
        transaction
            .commit()
            .await
            .map_err(|_| IntentPersistenceError::StorageFailure)?;
        Ok(PersistedIntent {
            intent_id: request.intent.id().to_owned(),
            swap_version: request.expected_swap_version + 1,
        })
    }
}

async fn persist_records(
    transaction: &mut Transaction<'_, Postgres>,
    request: &PersistIntentRequest,
) -> Result<(), IntentPersistenceError> {
    let intent = &request.intent;
    sqlx::query(
        "INSERT INTO transaction_intents (id, swap_id, chain_id, chain_family, network_id, \
         input_asset_address, input_atomic, output_asset_address, minimum_received_atomic, policy_version) \
         VALUES ($1::uuid, $2::uuid, $3, $4, $5, $6, $7::numeric, $8, $9::numeric, $10)",
    )
    .bind(intent.id())
    .bind(&request.swap_id)
    .bind(intent.chain_context().chain_id().as_str())
    .bind(format!("{:?}", intent.chain_context().family()))
    .bind(intent.chain_context().network_id().as_str())
    .bind(intent.input().asset_id().canonical_address())
    .bind(intent.input().atomic().to_string())
    .bind(intent.output_asset().canonical_address())
    .bind(intent.minimum_received().atomic().to_string())
    .bind(intent.policy_version().as_str())
    .execute(&mut **transaction)
    .await
    .map_err(|_| IntentPersistenceError::DuplicateIntentId)?;
    sqlx::query(
        "INSERT INTO audit_events (id, entity_id, actor_id, action, correlation_id) \
         VALUES ($1::uuid, $2::uuid, $3, 'intent_created', $4)",
    )
    .bind(&request.audit_event.event_id)
    .bind(&request.swap_id)
    .bind(&request.audit_event.actor_id)
    .bind(&request.audit_event.correlation_id)
    .execute(&mut **transaction)
    .await
    .map_err(|_| IntentPersistenceError::StorageFailure)?;
    sqlx::query(
        "INSERT INTO outbox_events (id, aggregate_id, event_type, correlation_id, delivery_key) \
         VALUES ($1::uuid, $2::uuid, 'intent_created', $3, $4)",
    )
    .bind(&request.outbox_event.event_id)
    .bind(&request.swap_id)
    .bind(&request.outbox_event.correlation_id)
    .bind(&request.outbox_event.delivery_key)
    .execute(&mut **transaction)
    .await
    .map_err(|_| IntentPersistenceError::StorageFailure)?;
    Ok(())
}
