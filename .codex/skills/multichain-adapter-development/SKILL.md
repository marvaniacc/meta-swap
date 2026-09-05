---
name: multichain-adapter-development
description: Guide chain-neutral core and isolated adapter development while preserving the TON-only MVP. Use when designing chain identity, asset values, adapter ports, or future-chain readiness.
---

# multichain-adapter-development

## Purpose
Guide chain-neutral core and isolated adapter development while preserving the TON-only MVP. Use when designing chain identity, asset values, adapter ports, or future-chain readiness.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/MULTICHAIN.md`, `docs/ARCHITECTURE.md`, `docs/FINANCIAL_CORRECTNESS.md`, `docs/TON_PHASE0_GATE.md`, `docs/TEST_STRATEGY.md`, and ADRs 001, 004, and 005.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Keep the shared core small and chain-neutral: chain/family/network/finality, canonical asset identity, atomic amounts, immutable intent, and verified execution.
- Confine BOCs/cells/Jettons and any future calldata/instructions/PSBTs to their respective adapters. Require adapter capabilities for normalization, validation, handoff, compilation, observation, finality, and verification.

## Prohibited actions
- Do not activate a non-TON chain, add an RPC URL as an adapter, introduce bridges/cross-chain routing, leak chain mechanics into the domain core, or identify assets by symbol.

## Expected output
- Deliver a port/capability matrix, chain-specific isolation map, finality assumptions, and activation evidence requirements.

## Required testing and review
- Add typed-identity, normalization, amount, adapter-contract, duplicate-observation, and finality/recovery tests relevant to the change.

## Escalate and stop
- Stop and report any request to activate a new chain without the required adapter, wallet matrix, testnet E2E, threat-model supplement, and launch approval.
