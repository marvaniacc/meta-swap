# Meta Swap

Meta Swap is a planned non-custodial Telegram swap bot. The project is beginning with a
Rust foundation that separates financial domain policy, application orchestration, and
Telegram presentation contracts.

## Workspace

- `meta-swap-domain`: chain-neutral identities, integer-only asset amounts, and the swap
  lifecycle. It has no database, Telegram, wallet, or provider dependency.
- `meta-swap-application`: use-case ports and optimistic-concurrency orchestration. A
  production repository must use PostgreSQL as the durable source of truth.
- `meta-swap-presentation`: Telegram-safe message and opaque callback contracts.
- `meta-swap-runtime`: validated process configuration that redacts database and webhook
  secrets from debug output.

No live TON wallet, quote-provider, DEX, or chain-verification integration is present.
Those adapters remain blocked by the Phase 0.1 evidence gate in `docs/TON_PHASE0_GATE.md`.
The workspace deliberately has no third-party dependencies yet; this avoids introducing an
unreviewed dependency before an infrastructure implementation needs one.

The initial PostgreSQL schema is in `db/migrations/`. It specifies durable idempotency,
optimistic aggregate revisions, immutable intent records, append-only audit/ledger records,
and transactional-outbox records. The runtime migration/repository adapter is deliberately
not included yet because the environment currently cannot fetch approved third-party Rust
dependencies; a no-op/in-memory replacement would violate the PostgreSQL source-of-truth
invariant.

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Read `AGENTS.md` and `docs/` before changing financial behavior or adding an integration.
