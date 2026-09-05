# Architecture

## Recommendation

Build a Rust modular monolith with separate HTTP/API and worker roles. PostgreSQL owns durable state, the ledger, audit history, and the transactional outbox. Redis is optional performance and coordination infrastructure only. The bot is the product UI; wallet approval happens in native wallets via a compatibility-tested TON Connect/headless-session path.

```text
Telegram webhook -> Presentation / Conversation service -> Application use cases
                                                   |-> PostgreSQL
                                                   |-> Redis (cache, rate limits, leases)
                                                   |-> Outbox workers -> Telegram API
                                                   |-> Quote provider adapter
                                                   |-> TON Connect adapter
                                                   `-> TON chain verifier
```

## Module boundaries

| Layer | Owns | Must not own |
|---|---|---|
| Domain | typed amounts, assets, swap invariants, state transitions, fee/referral formulas | HTTP, SQL, Telegram, wallet transport |
| Application | use cases, idempotency orchestration, transaction boundaries, ports | provider-specific DTOs or rendering |
| Infrastructure | PostgreSQL, Redis, Telegram, TON, provider implementations | financial policy decisions |
| Presentation | commands, callbacks, localization, keyboards, message rendering | chain verification and ledger logic |
| Workers | outbox delivery, expiry, verification, reconciliation leases | in-memory-only financial state |

## Runtime and composition

- Rust current stable edition, Tokio runtime, Axum/Tower HTTP stack, structured `tracing`.
- Constructor injection wires ports to adapters. Traits are limited to external boundaries: repositories, clock, quote provider, chain reader, wallet transport, notification sender.
- API replicas are stateless. PostgreSQL optimistic locking protects user and swap concurrency. Workers claim jobs with durable leases/row locking.
- Graceful shutdown stops intake, cancels workers, releases leases by expiry, and drains safe in-flight work. A user signing action is never replayed on shutdown.

## Deferred integration boundary

The core exposes `ChainAdapter`, `WalletHandoffPort`, `QuoteProvider`, and `ChainVerifier` ports. TON production implementations are blocked on Phase 0.1 testnet evidence. Omniston or any DEX is an adapter, never a domain dependency.
