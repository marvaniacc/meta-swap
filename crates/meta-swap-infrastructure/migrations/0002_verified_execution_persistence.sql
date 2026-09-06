CREATE TABLE verification_idempotency (
    swap_id UUID NOT NULL REFERENCES swap_sessions (id),
    idempotency_key TEXT NOT NULL,
    execution_id UUID NOT NULL,
    PRIMARY KEY (swap_id, idempotency_key),
    UNIQUE (execution_id)
);

CREATE TABLE swap_executions (
    id UUID PRIMARY KEY,
    swap_id UUID NOT NULL UNIQUE REFERENCES swap_sessions (id),
    intent_id UUID NOT NULL REFERENCES transaction_intents (id),
    evidence_reference TEXT NOT NULL UNIQUE,
    input_asset_address TEXT NOT NULL,
    input_atomic NUMERIC(39, 0) NOT NULL CHECK (input_atomic > 0),
    output_asset_address TEXT NOT NULL,
    output_atomic NUMERIC(39, 0) NOT NULL CHECK (output_atomic >= 0),
    policy_version TEXT NOT NULL
);

CREATE TABLE fee_events (
    id UUID PRIMARY KEY,
    execution_id UUID NOT NULL UNIQUE REFERENCES swap_executions (id),
    asset_address TEXT NOT NULL,
    atomic NUMERIC(39, 0) NOT NULL CHECK (atomic > 0),
    policy_version TEXT NOT NULL
);

CREATE TABLE ledger_entries (
    id UUID PRIMARY KEY,
    swap_id UUID NOT NULL REFERENCES swap_sessions (id),
    execution_id UUID NOT NULL REFERENCES swap_executions (id),
    event_type TEXT NOT NULL,
    asset_address TEXT NOT NULL,
    atomic NUMERIC(39, 0) NOT NULL CHECK (atomic >= 0),
    correlation_id TEXT NOT NULL,
    evidence_reference TEXT NOT NULL,
    policy_version TEXT NOT NULL
);

CREATE FUNCTION prevent_append_only_event_mutation() RETURNS trigger AS $$
BEGIN
    RAISE EXCEPTION 'append-only financial records cannot be mutated';
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER audit_events_append_only BEFORE UPDATE OR DELETE ON audit_events
    FOR EACH ROW EXECUTE FUNCTION prevent_append_only_event_mutation();
CREATE TRIGGER ledger_entries_append_only BEFORE UPDATE OR DELETE ON ledger_entries
    FOR EACH ROW EXECUTE FUNCTION prevent_append_only_event_mutation();
