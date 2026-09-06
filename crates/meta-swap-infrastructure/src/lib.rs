#![forbid(unsafe_code)]
//! `PostgreSQL` adapters. Financial writes use explicit database transactions.

use meta_swap_application::{
    ExecutionLedgerEntryKind, IntentPersistenceError, IntentRepository, PersistIntentRequest,
    PersistVerifiedExecutionRequest, PersistedIntent, PersistedVerifiedExecution,
    VerifiedExecutionPersistenceError, VerifiedExecutionRepository,
};
use sqlx::{PgPool, Postgres, Transaction};

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
    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
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

impl VerifiedExecutionRepository for PostgresIntentRepository {
    async fn persist_verified_execution(
        &mut self,
        request: PersistVerifiedExecutionRequest,
    ) -> Result<PersistedVerifiedExecution, VerifiedExecutionPersistenceError> {
        validate_ledger_entries(&request)?;
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        let claimed = sqlx::query(
            "INSERT INTO verification_idempotency (swap_id, idempotency_key, execution_id) \
             VALUES ($1::uuid, $2, $3::uuid) ON CONFLICT DO NOTHING",
        )
        .bind(&request.swap_id)
        .bind(&request.idempotency_key)
        .bind(&request.execution_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        if claimed.rows_affected() == 0 {
            return Err(VerifiedExecutionPersistenceError::DuplicateIdempotencyKey);
        }

        let updated = sqlx::query(
            "UPDATE swap_sessions SET state = 'swap_succeeded', version = version + 1 \
             WHERE id = $1::uuid AND version = $2 AND state = 'financial_verification_pending'",
        )
        .bind(&request.swap_id)
        .bind(
            i64::try_from(request.expected_swap_version)
                .map_err(|_| VerifiedExecutionPersistenceError::VersionConflict)?,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        if updated.rows_affected() == 0 {
            transaction
                .rollback()
                .await
                .map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
            return Err(VerifiedExecutionPersistenceError::VersionConflict);
        }

        let execution = &request.verified_execution;
        sqlx::query(
            "INSERT INTO swap_executions (id, swap_id, intent_id, evidence_reference, input_asset_address, \
             input_atomic, output_asset_address, output_atomic, policy_version) \
             VALUES ($1::uuid, $2::uuid, $3::uuid, $4, $5, $6::numeric, $7, $8::numeric, $9)",
        )
        .bind(&request.execution_id).bind(&request.swap_id).bind(&request.intent_id)
        .bind(execution.evidence_reference()).bind(execution.actual_input().asset_id().canonical_address())
        .bind(execution.actual_input().atomic().to_string()).bind(execution.actual_output().asset_id().canonical_address())
        .bind(execution.actual_output().atomic().to_string()).bind(request.policy_version.as_str())
        .execute(&mut *transaction).await.map_err(|_| VerifiedExecutionPersistenceError::DuplicateExecution)?;

        if let Some(fee) = &request.realized_fee {
            sqlx::query(
                "INSERT INTO fee_events (id, execution_id, asset_address, atomic, policy_version) \
                 VALUES ($1::uuid, $2::uuid, $3, $4::numeric, $5)",
            )
            .bind(&fee.fee_event_id)
            .bind(&request.execution_id)
            .bind(fee.amount.asset_id().canonical_address())
            .bind(fee.amount.atomic().to_string())
            .bind(request.policy_version.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        }
        for entry in &request.ledger_entries {
            sqlx::query(
                "INSERT INTO ledger_entries (id, swap_id, execution_id, event_type, asset_address, atomic, \
                 correlation_id, evidence_reference, policy_version) \
                 VALUES ($1::uuid, $2::uuid, $3::uuid, $4, $5, $6::numeric, $7, $8, $9)",
            )
            .bind(&entry.entry_id).bind(&request.swap_id).bind(&request.execution_id).bind(entry.kind.as_str())
            .bind(entry.amount.asset_id().canonical_address()).bind(entry.amount.atomic().to_string())
            .bind(&request.audit_event.correlation_id).bind(execution.evidence_reference()).bind(request.policy_version.as_str())
            .execute(&mut *transaction).await.map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        }
        sqlx::query(
            "INSERT INTO audit_events (id, entity_id, actor_id, action, correlation_id) VALUES ($1::uuid, $2::uuid, $3, 'execution_verified', $4)",
        )
        .bind(&request.audit_event.event_id).bind(&request.swap_id).bind(&request.audit_event.actor_id).bind(&request.audit_event.correlation_id)
        .execute(&mut *transaction).await.map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        sqlx::query(
            "INSERT INTO outbox_events (id, aggregate_id, event_type, correlation_id, delivery_key) VALUES ($1::uuid, $2::uuid, 'execution_verified', $3, $4)",
        )
        .bind(&request.outbox_event.event_id).bind(&request.swap_id).bind(&request.outbox_event.correlation_id).bind(&request.outbox_event.delivery_key)
        .execute(&mut *transaction).await.map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        transaction
            .commit()
            .await
            .map_err(|_| VerifiedExecutionPersistenceError::StorageFailure)?;
        Ok(PersistedVerifiedExecution {
            execution_id: request.execution_id,
            swap_version: request.expected_swap_version + 1,
        })
    }
}

fn validate_ledger_entries(
    request: &PersistVerifiedExecutionRequest,
) -> Result<(), VerifiedExecutionPersistenceError> {
    let input = request
        .ledger_entries
        .iter()
        .filter(|entry| entry.kind == ExecutionLedgerEntryKind::InputDebited)
        .collect::<Vec<_>>();
    let output = request
        .ledger_entries
        .iter()
        .filter(|entry| entry.kind == ExecutionLedgerEntryKind::OutputCredited)
        .collect::<Vec<_>>();
    let fee = request
        .ledger_entries
        .iter()
        .filter(|entry| entry.kind == ExecutionLedgerEntryKind::ProductFeeRealized)
        .collect::<Vec<_>>();
    if input.len() != 1
        || output.len() != 1
        || input[0].amount != *request.verified_execution.actual_input()
        || output[0].amount != *request.verified_execution.actual_output()
        || fee.len() != usize::from(request.realized_fee.is_some())
    {
        return Err(VerifiedExecutionPersistenceError::InvalidLedgerEntries);
    }
    if let Some(realized_fee) = &request.realized_fee {
        if fee[0].amount != realized_fee.amount || realized_fee.amount.atomic() == 0 {
            return Err(VerifiedExecutionPersistenceError::InvalidLedgerEntries);
        }
    }
    Ok(())
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
