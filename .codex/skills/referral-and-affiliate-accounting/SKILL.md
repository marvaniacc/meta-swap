---
name: referral-and-affiliate-accounting
description: Guide referral attribution, realized-fee rewards, vesting, controlled payout accounting, and related audit rules. Use for referral or affiliate financial changes.
---

# referral-and-affiliate-accounting

## Purpose
Guide referral attribution, realized-fee rewards, vesting, controlled payout accounting, and related audit rules. Use for referral or affiliate financial changes.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/REFERRALS.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/DATA_MODEL.md`, `docs/SECURITY.md`, `docs/TEST_STRATEGY.md`, and ADRs 004 and 006.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Bind one immutable, validated referral relationship; prevent self-referral. Snapshot versioned fee/share/rounding policy.
- Create exactly one reward from an eligible verified `fee_realized` event, then use append-only lifecycle events and controlled business-treasury payout batches with independent verification.
- Treat risk holds as lifecycle decisions, not retrospective attribution rewrites.

## Prohibited actions
- Do not reward quotes, submissions, expected fees, failed/ambiguous swaps, or client-supplied referrers; create an internal user balance; or mark a payout paid from broadcast acknowledgement.

## Expected output
- Deliver attribution validation, formula/rounding and policy version, idempotency correlations, lifecycle transitions, and operator approval/audit requirements.

## Required testing and review
- Add formula/property tests, database uniqueness tests, duplicate/restart reward tests, hold/release tests, and payout evidence/reconciliation tests as applicable.

## Escalate and stop
- Stop and report undefined fee evidence, rounding/vesting policy, eligibility ambiguity, payout authorization uncertainty, or any change that makes rewards custodial.
