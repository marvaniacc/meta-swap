---
name: security-review-financial-bot
description: Perform a security review of Meta Swap financial-bot changes across trust boundaries, secrets, authorization, financial finality, and recovery. Use before merging sensitive workflow changes.
---

# security-review-financial-bot

## Purpose
Perform a security review of Meta Swap financial-bot changes across trust boundaries, secrets, authorization, financial finality, and recovery. Use before merging sensitive workflow changes.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/SECURITY.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/ARCHITECTURE.md`, `docs/TELEGRAM_UX_I18N.md`, `docs/TON_PHASE0_GATE.md`, `docs/TEST_STRATEGY.md`, and ADRs 002–006.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Enumerate untrusted inputs and assets; trace authentication, authorization, idempotency, audit, encryption/redaction, and finality decisions.
- Review Telegram replay/tampering, session binding, payload substitution, quote staleness, provider/indexer disagreement, Redis loss, admin privilege, logging, and payout controls.
- Classify findings by impact and required evidence; require a test or documented compensating control for each accepted risk.

## Prohibited actions
- Do not approve custody behavior, secret logging, trust in external success acknowledgements, unapproved integrations, silent reconciliation, or security claims without evidence.

## Expected output
- Deliver scope, threat findings, affected invariant, exploit preconditions, remediation, test evidence, residual risk, and release-gate impact.

## Required testing and review
- Run relevant negative/security tests and a credential-pattern scan; require duplicate/restart and finality tests for financial workflows.

## Escalate and stop
- Stop and report critical/unknown signing-material exposure, authorization bypass, unverifiable finality, missing audit trail, or a gate-blocked integration.
