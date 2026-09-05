---
name: rust-financial-backend
description: Guide Rust changes to the Meta Swap financial backend, domain model, application use cases, persistence ports, and workers. Use when adding or reviewing Rust financial workflow code.
---

# rust-financial-backend

## Purpose
Guide Rust changes to the Meta Swap financial backend, domain model, application use cases, persistence ports, and workers. Use when adding or reviewing Rust financial workflow code.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/ARCHITECTURE.md`, `docs/DATA_MODEL.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/SWAP_STATE_MACHINE.md`, `docs/TEST_STRATEGY.md`, and ADRs 001, 002, and 004.
- Map each change to domain, application, infrastructure, presentation, or worker boundaries before coding.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Model amounts with asset-bound atomic integers; record policy versions and rounding.
- Enforce durable uniqueness, optimistic concurrency, transactional outbox writes, append-only ledger/audit records, and deterministic recovery.
- Keep external DTOs, SQL, Telegram rendering, and chain mechanics outside the domain layer.

## Prohibited actions
- Do not add dependencies, services, brokers, floating-point financial math, mutable finalized records, or fallback resubmission.

## Expected output
- Deliver a boundary map, invariant/constraint changes, migration and idempotency implications, and unresolved facts.

## Required testing and review
- Run formatting and linting for changed Rust; add unit/property/integration coverage plus duplicate-delivery and restart recovery tests for changed state or workers.

## Escalate and stop
- Stop and report any ambiguity about finality, persistence transaction boundaries, policy rounding, or a required architecture/financial-invariant change.
