---
name: noncustodial-financial-correctness
description: Review and implement non-custodial financial invariants, immutable intent, finality, ledger, idempotency, reconciliation, and recovery. Use for any financial state or accounting change.
---

# noncustodial-financial-correctness

## Purpose
Review and implement non-custodial financial invariants, immutable intent, finality, ledger, idempotency, reconciliation, and recovery. Use for any financial state or accounting change.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/DATA_MODEL.md`, `docs/SWAP_STATE_MACHINE.md`, `docs/SECURITY.md`, `docs/TEST_STRATEGY.md`, and ADRs 002, 004, and 006.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Define the immutable intent, authoritative evidence, durable idempotency key, append-only audit/ledger event, and restart behavior before implementation.
- Derive success, realized fees, and downstream events only in the verifier’s finalized-evidence transaction. Reconcile gaps into durable mismatch/alert records.

## Prohibited actions
- Do not treat a quote, wallet response, provider response, BOC, broadcast acknowledgement, or message lookup as success; overwrite finalized financial data; create rewards from expected fees; or use Redis as the final guard.

## Expected output
- Deliver an invariant table, event/correlation design, failure outcomes (`pending` or `ambiguous` for unknown evidence), and recovery plan.

## Required testing and review
- Add unit/property, PostgreSQL constraint/idempotency, duplicate delivery, reconciliation, and process-restart tests for every changed invariant.

## Escalate and stop
- Stop and report missing finality evidence, non-deterministic fee/rounding policy, contradiction between evidence sources, or any custody exposure.
