CREATE TABLE swap_sessions (
    id UUID PRIMARY KEY,
    state TEXT NOT NULL,
    version BIGINT NOT NULL CHECK (version >= 0)
);

CREATE TABLE transaction_intents (
    id UUID PRIMARY KEY,
    swap_id UUID NOT NULL REFERENCES swap_sessions (id),
    chain_id TEXT NOT NULL,
    chain_family TEXT NOT NULL,
    network_id TEXT NOT NULL,
    input_asset_address TEXT NOT NULL,
    input_atomic NUMERIC(39, 0) NOT NULL CHECK (input_atomic > 0),
    output_asset_address TEXT NOT NULL,
    minimum_received_atomic NUMERIC(39, 0) NOT NULL CHECK (minimum_received_atomic >= 0),
    policy_version TEXT NOT NULL
);

CREATE TABLE intent_idempotency (
    swap_id UUID NOT NULL REFERENCES swap_sessions (id),
    idempotency_key TEXT NOT NULL,
    intent_id UUID NOT NULL,
    PRIMARY KEY (swap_id, idempotency_key),
    UNIQUE (intent_id)
);

CREATE TABLE audit_events (
    id UUID PRIMARY KEY,
    entity_id UUID NOT NULL REFERENCES swap_sessions (id),
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    correlation_id TEXT NOT NULL
);

CREATE TABLE outbox_events (
    id UUID PRIMARY KEY,
    aggregate_id UUID NOT NULL REFERENCES swap_sessions (id),
    event_type TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    delivery_key TEXT NOT NULL UNIQUE
);
