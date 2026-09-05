---
name: ton-wallet-chain-verification
description: Govern TON wallet-session and chain-verification work without bypassing the Phase 0.1 release gate. Use for TON adapter interfaces, session policy, evidence, or verifier design/review.
---

# ton-wallet-chain-verification

## Purpose
Govern TON wallet-session and chain-verification work without bypassing the Phase 0.1 release gate. Use for TON adapter interfaces, session policy, evidence, or verifier design/review.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/TON_PHASE0_GATE.md`, `docs/RELEASE_GATES.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/MULTICHAIN.md`, `docs/SECURITY.md`, `docs/SWAP_STATE_MACHINE.md`, and ADRs 002, 004, and 005.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Keep TON-specific BOC, cell, Jetton, router payload, and finality logic inside the TON adapter/verifier boundary.
- Require explicit network, allowlists, immutable intent comparison, one-time proof binding, expiry/revoke policy, and independent masterchain-final evidence.
- Until Phase 0.1 evidence is recorded, limit work to interfaces, fixtures, policy, and tests.

## Prohibited actions
- Do not implement production TON Connect transport, wallet/provider integration, live liquidity routing, raw `ton://` primary transport, signing authority, or success from wallet/message evidence.

## Expected output
- Deliver evidence requirements, allowed interface/test-only scope, verifier comparison criteria, and all pending/ambiguous cases.

## Required testing and review
- Require contract fixtures and malformed/late-response tests; defer testnet E2E implementation until gate evidence exists. Test duplicate/restart referral triggering only after verified fee evidence.

## Escalate and stop
- Stop and report unverified protocol behavior, an unapproved endpoint/router/token/wallet, absent gate evidence, or inability to independently establish finality.
