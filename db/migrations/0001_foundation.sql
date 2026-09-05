-- Phase 1A PostgreSQL foundation. Apply with the infrastructure migration runner once the
-- approved SQL driver dependency can be fetched in the build environment.
CREATE TABLE users (
    id UUID PRIMARY KEY,
    telegram_user_id BIGINT NOT NULL UNIQUE,
    preferred_locale TEXT NOT NULL DEFAULT 'en',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE telegram_updates (
    telegram_update_id BIGINT PRIMARY KEY,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE swap_sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    idempotency_key TEXT NOT NULL UNIQUE,
    state TEXT NOT NULL CHECK (state IN (
        'draft', 'awaiting_pair', 'awaiting_amount', 'quoting', 'quote_available',
        'awaiting_confirmation', 'building_intent', 'awaiting_wallet_approval',
        'wallet_response_received', 'chain_candidate_observed',
        'financial_verification_pending', 'swap_succeeded', 'swap_failed_onchain',
        'ambiguous', 'finalized', 'finalized_failed', 'cancelled'
    )),
    version BIGINT NOT NULL CHECK (version >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE transaction_intents (
    id UUID PRIMARY KEY,
    swap_id UUID NOT NULL REFERENCES swap_sessions(id),
    sequence INTEGER NOT NULL,
    payload_hash TEXT NOT NULL,
    policy_version TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (swap_id, sequence)
);

CREATE TABLE ledger_entries (
    id UUID PRIMARY KEY,
    correlation_id TEXT NOT NULL UNIQUE,
    event_type TEXT NOT NULL,
    asset_chain_id TEXT NOT NULL,
    asset_address TEXT NOT NULL,
    atomic_amount NUMERIC(78, 0) NOT NULL CHECK (atomic_amount >= 0),
    policy_version TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE audit_events (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id UUID NOT NULL,
    event_type TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (entity_type, entity_id, event_type, correlation_id)
);

CREATE TABLE outbox_events (
    id UUID PRIMARY KEY,
    delivery_key TEXT NOT NULL UNIQUE,
    event_type TEXT NOT NULL,
    aggregate_id UUID NOT NULL,
    payload TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    available_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    delivered_at TIMESTAMPTZ,
    lease_owner TEXT,
    lease_expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
