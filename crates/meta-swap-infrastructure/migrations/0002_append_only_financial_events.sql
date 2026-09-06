CREATE TABLE ledger_entries (
    id UUID PRIMARY KEY,
    swap_id UUID NOT NULL REFERENCES swap_sessions (id),
    event_type TEXT NOT NULL,
    asset_chain_id TEXT NOT NULL,
    asset_address TEXT NOT NULL,
    atomic_amount NUMERIC(39, 0) NOT NULL CHECK (atomic_amount >= 0),
    correlation_id TEXT NOT NULL,
    effective_at TIMESTAMPTZ NOT NULL,
    chain_evidence_reference TEXT,
    policy_version TEXT NOT NULL,
    UNIQUE (event_type, correlation_id)
);

CREATE FUNCTION prevent_append_only_event_mutation()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE EXCEPTION 'append-only financial and audit events cannot be modified or deleted';
END;
$$;

CREATE TRIGGER audit_events_are_append_only
BEFORE UPDATE OR DELETE ON audit_events
FOR EACH ROW EXECUTE FUNCTION prevent_append_only_event_mutation();

CREATE TRIGGER ledger_entries_are_append_only
BEFORE UPDATE OR DELETE ON ledger_entries
FOR EACH ROW EXECUTE FUNCTION prevent_append_only_event_mutation();
