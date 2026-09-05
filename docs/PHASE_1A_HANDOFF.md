# Codex handoff: Phase 1A foundation

## Mission

Implement the foundation of Meta Swap, a non-custodial, multi-chain-ready Telegram swap bot backend in Rust. Read `AGENTS.md` and every document in `docs/` before making changes.

## In scope

1. A small Rust workspace and dependency direction matching `ARCHITECTURE.md`.
2. Chain-neutral typed domain values and integer-only amount arithmetic.
3. PostgreSQL migrations and repositories for the data-model foundation.
4. Persisted swap-state transitions, optimistic concurrency, and durable idempotency.
5. Append-only ledger/audit, fee and referral-domain foundations.
6. Transactional outbox, worker leasing, configuration, tracing, health/readiness/liveness.
7. Telegram presentation abstractions, durable update/callback protection, and English-only localization keys.
8. Unit, property, PostgreSQL integration, idempotency, and restart/recovery test foundations.

## Explicitly out of scope

- TON Connect transport, bridge cryptography, wallet signing, or production session adapter.
- Omniston, STON.fi, or any live quote/provider integration.
- Mainnet transactions, arbitrary asset listing, cross-chain swap, or bridges.
- Real affiliate payout execution.
- Any flow that accepts or stores user wallet secrets.

## Completion standard

Do not call Phase 1A complete without tests proving durable idempotency, restart recovery, financial immutability, no floating-point amount behavior, and the separation of user-visible text from business logic. Report facts still blocked by Phase 0.1 rather than replacing them with assumptions.
