# Meta Swap engineering guidance

## Product invariants

- The product is non-custodial. Never request, accept, store, transmit, or log a user private key, seed phrase, mnemonic, or signing secret.
- Telegram Bot is the product UI. Do not introduce a Telegram Mini App without an approved ADR.
- TON is the only active execution chain in the MVP. Keep the core multi-chain-ready, but do not add cross-chain swaps or bridges without an approved ADR.
- Never use floating-point arithmetic for asset amounts, fees, rewards, or slippage. Use typed atomic integer amounts plus explicit rounding rules.
- PostgreSQL is the durable source of truth for financial and authorization state. Redis is only a cache, rate-limit store, or short-lived coordination aid.
- A wallet response, provider response, broadcast acknowledgement, BOC, or message lookup must never by itself finalize a swap.
- A swap is successful only after independently verified, final on-chain execution matches the immutable transaction intent.
- Every financial transition must be idempotent, auditable, restart-safe, and represented by append-only ledger/audit events.
- A referral reward may be created only from a verified, realized product-fee event.
- All user-facing copy must use stable localization keys. English is the MVP locale; business and domain layers must not contain rendered user text.

## Delivery rules

- Read `docs/` and relevant ADRs before changing the architecture or financial logic.
- Keep modules within domain, application, infrastructure, presentation, and worker boundaries described in `docs/ARCHITECTURE.md`.
- Do not add a dependency without documenting its purpose, maintenance posture, and security impact in the change summary or an ADR.
- Prefer a small modular monolith over new services. Do not add a message broker before the PostgreSQL outbox design is insufficient.
- Do not implement TON Connect production transport, wallet-provider integration, or a live liquidity provider until the Phase 0.1 release gate is satisfied.
- Treat unknown chain evidence as `pending` or `ambiguous`, never as success.
- Add or update unit, integration, idempotency, recovery, and security tests that cover every changed invariant.
- Never silence a financial failure with a fallback that changes the transaction or resubmits a user action.

## Required checks

- Format and lint all changed Rust code.
- Run affected unit, property, integration, and migration tests.
- Include restart and duplicate-delivery coverage for state-machine or worker changes.
- Report unresolved protocol facts rather than inventing provider or wallet behavior.
