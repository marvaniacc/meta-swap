use meta_swap_application::{
    ExecutionLedgerEntry, ExecutionLedgerEntryKind, IntentAuditEvent, IntentOutboxEvent,
    IntentPersistenceError, IntentRepository, PersistIntentRequest,
    PersistVerifiedExecutionRequest, RealizedFee, VerifiedExecutionPersistenceError,
    VerifiedExecutionRepository,
};
use meta_swap_domain::{
    AssetAmount, AssetId, ChainContext, ChainFamily, ChainId, FinalityLevel, NetworkId,
    PolicyVersion, SwapIntent, VerifiedExecution,
};
use meta_swap_infrastructure::PostgresIntentRepository;
use sqlx::{PgPool, Row};

const SWAP_ID: &str = "00000000-0000-0000-0000-000000000001";
const INTENT_ID: &str = "00000000-0000-0000-0000-000000000002";

#[tokio::test]
async fn persists_intent_audit_and_outbox_atomically_and_rejects_duplicate_delivery() {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        eprintln!("skipping PostgreSQL integration test: DATABASE_URL is not set");
        return;
    };
    let pool = PgPool::connect(&database_url).await.unwrap();
    let mut repository = PostgresIntentRepository::new(pool.clone());
    repository.migrate().await.unwrap();
    sqlx::query("TRUNCATE swap_sessions CASCADE")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO swap_sessions (id, state, version) VALUES ($1::uuid, 'building_intent', 4)",
    )
    .bind(SWAP_ID)
    .execute(&pool)
    .await
    .unwrap();

    let request = request();
    let persisted = repository.persist_intent(request.clone()).await.unwrap();
    assert_eq!(persisted.intent_id, INTENT_ID);
    assert_eq!(persisted.swap_version, 5);
    assert_eq!(count(&pool, "transaction_intents").await, 1);
    assert_eq!(count(&pool, "audit_events").await, 1);
    assert_eq!(count(&pool, "outbox_events").await, 1);

    assert_eq!(
        repository.persist_intent(request).await,
        Err(IntentPersistenceError::DuplicateIdempotencyKey)
    );
    assert_eq!(count(&pool, "transaction_intents").await, 1);
    assert_eq!(count(&pool, "audit_events").await, 1);
    assert_eq!(count(&pool, "outbox_events").await, 1);
}

#[tokio::test]
async fn persists_finalized_execution_financial_events_atomically_and_rejects_redelivery() {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        eprintln!("skipping PostgreSQL integration test: DATABASE_URL is not set");
        return;
    };
    let pool = PgPool::connect(&database_url).await.unwrap();
    let mut repository = PostgresIntentRepository::new(pool.clone());
    repository.migrate().await.unwrap();
    sqlx::query("TRUNCATE swap_sessions CASCADE")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO swap_sessions (id, state, version) VALUES ($1::uuid, 'financial_verification_pending', 7)")
        .bind(SWAP_ID).execute(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO transaction_intents (id, swap_id, chain_id, chain_family, network_id, input_asset_address, input_atomic, output_asset_address, minimum_received_atomic, policy_version) \
         VALUES ($1::uuid, $2::uuid, 'ton-mainnet', 'Ton', 'mainnet', 'native', 1000, 'jetton:output', 900, 'policy-1')",
    ).bind(INTENT_ID).bind(SWAP_ID).execute(&pool).await.unwrap();

    let request = verified_request();
    let persisted = repository
        .persist_verified_execution(request.clone())
        .await
        .unwrap();
    assert_eq!(persisted.swap_version, 8);
    assert_eq!(count(&pool, "swap_executions").await, 1);
    assert_eq!(count(&pool, "fee_events").await, 1);
    assert_eq!(count(&pool, "ledger_entries").await, 3);
    assert_eq!(count(&pool, "audit_events").await, 1);
    assert_eq!(count(&pool, "outbox_events").await, 1);
    assert_eq!(
        repository.persist_verified_execution(request).await,
        Err(VerifiedExecutionPersistenceError::DuplicateIdempotencyKey)
    );
    assert_eq!(count(&pool, "swap_executions").await, 1);
    assert_eq!(count(&pool, "ledger_entries").await, 3);
}

fn request() -> PersistIntentRequest {
    let chain_id = ChainId::new("ton-mainnet").unwrap();
    let input_asset = AssetId::new(chain_id.clone(), "native").unwrap();
    let output_asset = AssetId::new(chain_id.clone(), "jetton:output").unwrap();
    PersistIntentRequest {
        swap_id: SWAP_ID.into(),
        expected_swap_version: 4,
        idempotency_key: "telegram-update-1".into(),
        intent: SwapIntent::new(
            INTENT_ID,
            ChainContext::new(
                chain_id,
                ChainFamily::Ton,
                NetworkId::new("mainnet").unwrap(),
            ),
            AssetAmount::new(input_asset, 1_000),
            output_asset.clone(),
            AssetAmount::new(output_asset, 900),
            PolicyVersion::new("policy-1").unwrap(),
        )
        .unwrap(),
        audit_event: IntentAuditEvent {
            event_id: "00000000-0000-0000-0000-000000000003".into(),
            actor_id: "telegram-user-1".into(),
            correlation_id: "intent-creation-1".into(),
        },
        outbox_event: IntentOutboxEvent {
            event_id: "00000000-0000-0000-0000-000000000004".into(),
            correlation_id: "intent-creation-1".into(),
            delivery_key: "outbox-intent-creation-1".into(),
        },
    }
}

fn verified_request() -> PersistVerifiedExecutionRequest {
    let chain_id = ChainId::new("ton-mainnet").unwrap();
    let input_asset = AssetId::new(chain_id.clone(), "native").unwrap();
    let output_asset = AssetId::new(chain_id.clone(), "jetton:output").unwrap();
    let intent = SwapIntent::new(
        INTENT_ID,
        ChainContext::new(
            chain_id,
            ChainFamily::Ton,
            NetworkId::new("mainnet").unwrap(),
        ),
        AssetAmount::new(input_asset.clone(), 1_000),
        output_asset.clone(),
        AssetAmount::new(output_asset.clone(), 900),
        PolicyVersion::new("policy-1").unwrap(),
    )
    .unwrap();
    let execution = VerifiedExecution::new(
        &intent,
        "final-evidence-1",
        FinalityLevel::Finalized,
        AssetAmount::new(input_asset.clone(), 1_000),
        AssetAmount::new(output_asset.clone(), 950),
    )
    .unwrap();
    PersistVerifiedExecutionRequest {
        swap_id: SWAP_ID.into(),
        expected_swap_version: 7,
        idempotency_key: "verification-1".into(),
        intent_id: INTENT_ID.into(),
        execution_id: "00000000-0000-0000-0000-000000000010".into(),
        verified_execution: execution,
        policy_version: PolicyVersion::new("policy-1").unwrap(),
        audit_event: IntentAuditEvent {
            event_id: "00000000-0000-0000-0000-000000000011".into(),
            actor_id: "verifier".into(),
            correlation_id: "verification-1".into(),
        },
        outbox_event: IntentOutboxEvent {
            event_id: "00000000-0000-0000-0000-000000000012".into(),
            correlation_id: "verification-1".into(),
            delivery_key: "execution-verified-1".into(),
        },
        ledger_entries: vec![
            ExecutionLedgerEntry {
                entry_id: "00000000-0000-0000-0000-000000000013".into(),
                kind: ExecutionLedgerEntryKind::InputDebited,
                amount: AssetAmount::new(input_asset, 1_000),
            },
            ExecutionLedgerEntry {
                entry_id: "00000000-0000-0000-0000-000000000014".into(),
                kind: ExecutionLedgerEntryKind::OutputCredited,
                amount: AssetAmount::new(output_asset.clone(), 950),
            },
            ExecutionLedgerEntry {
                entry_id: "00000000-0000-0000-0000-000000000015".into(),
                kind: ExecutionLedgerEntryKind::ProductFeeRealized,
                amount: AssetAmount::new(output_asset.clone(), 5),
            },
        ],
        realized_fee: Some(RealizedFee {
            fee_event_id: "00000000-0000-0000-0000-000000000016".into(),
            amount: AssetAmount::new(output_asset, 5),
        }),
    }
}

async fn count(pool: &PgPool, table: &str) -> i64 {
    let query = format!("SELECT COUNT(*) AS count FROM {table}");
    sqlx::query(&query)
        .fetch_one(pool)
        .await
        .unwrap()
        .get("count")
}
