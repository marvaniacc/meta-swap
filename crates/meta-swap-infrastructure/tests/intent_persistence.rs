use meta_swap_application::{
    IntentAuditEvent, IntentOutboxEvent, IntentPersistenceError, IntentRepository,
    PersistIntentRequest,
};
use meta_swap_domain::{
    AssetAmount, AssetId, ChainContext, ChainFamily, ChainId, NetworkId, PolicyVersion, SwapIntent,
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
    sqlx::query("TRUNCATE outbox_events, audit_events, intent_idempotency, transaction_intents, swap_sessions")
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

async fn count(pool: &PgPool, table: &str) -> i64 {
    let query = format!("SELECT COUNT(*) AS count FROM {table}");
    sqlx::query(&query)
        .fetch_one(pool)
        .await
        .unwrap()
        .get("count")
}
