---
name: rust-test-and-recovery
description: Guide Rust testing for financial correctness, concurrency, duplicate delivery, worker recovery, and restart safety. Use when adding or reviewing tests for stateful or financial code.
---

# rust-test-and-recovery

## Purpose
Guide Rust testing for financial correctness, concurrency, duplicate delivery, worker recovery, and restart safety. Use when adding or reviewing tests for stateful or financial code.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/TEST_STRATEGY.md`, `docs/SWAP_STATE_MACHINE.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/DATA_MODEL.md`, `docs/ARCHITECTURE.md`, and ADRs 001, 004, and 006.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Select unit, property, PostgreSQL integration, boundary-contract, failure, and recovery tests based on the changed invariant.
- Use real PostgreSQL (and Redis where relevant) for durable uniqueness, concurrency, outbox, and recovery coverage. Kill/restart at non-terminal points and redeliver external events.
- Assert at-most-once financial event creation, immutable terminal records, explicit `pending`/`ambiguous` uncertainty, and no signing-action replay.

## Prohibited actions
- Do not rely solely on mocks for database guarantees, use timing-only assertions for concurrency, omit duplicate/restart cases for workers/state changes, or run automated mainnet financial transactions.

## Expected output
- Deliver a test matrix linking each invariant to fault injection, assertions, deterministic fixtures, and environment requirements.

## Required testing and review
- Run affected formatting/linting, unit/property/integration/migration tests, and security scans; report environment-limited tests rather than weakening assertions.

## Escalate and stop
- Stop and report an untestable invariant, missing durable boundary, nondeterministic finality source, or recovery behavior that could replay user actions.
