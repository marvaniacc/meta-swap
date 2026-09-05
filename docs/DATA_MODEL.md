# Data model

## Database principles

- PostgreSQL stores all durable financial, authorization, idempotency, audit, and workflow facts.
- Use UUID primary keys, UTC timestamps, atomic integer amounts, foreign keys, unique constraints, and optimistic version columns where state mutates.
- Raw sensitive wallet/session material is encrypted at rest; logs contain only redacted identifiers and hashes.

## Core tables

| Table | Purpose and key constraints |
|---|---|
| `users` | Telegram identity; `telegram_user_id` unique; locale preference and lifecycle status |
| `chains` | chain registry, family, network, adapter/policy version, status |
| `assets` | chain-scoped canonical asset identity; unique `(chain_id, canonical_address_or_marker)` |
| `wallet_sessions` | encrypted per-user dApp session state; wallet/address/network binding, status and revision |
| `dialogue_states` | mutable UI draft keyed by user; never sole financial state |
| `telegram_updates` | durable update dedupe with unique Telegram update ID |
| `callback_actions` | opaque, expiring, one-time actions bound to user and aggregate revision |
| `swap_sessions` | swap aggregate, state, version, immutable identity; unique user/idempotency key |
| `quotes` | immutable provider quote snapshots, expiry, min output and policy versions |
| `transaction_intents` | immutable canonical approval request and payload hash; unique sequence per swap |
| `chain_transactions` | deduplicated chain observations by canonical chain identity |
| `swap_executions` | one final verified execution per swap; corrections use new evidence/events |
| `fee_events` | expected/realized/voided fees, correlated to swap and policy |
| `ledger_entries` | append-only financial events with correlation uniqueness |
| `referral_codes` | unique normalized invite codes |
| `referral_relationships` | immutable referrer/referred relation; unique referred user; self-referral forbidden |
| `referral_reward_events` | idempotent reward per realized fee/relationship, lifecycle status |
| `affiliate_payout_accounts` | verified payout destinations with cooldown/revocation state |
| `affiliate_payout_batches` / `affiliate_payout_items` | controlled business payout workflow and chain evidence |
| `settings_versions` | immutable policy/configuration versions |
| `audit_events` | append-only actor/action/entity history |
| `outbox_events` | transactional integration events, retry status, delivery key |

## Redis use

Redis may cache catalog/quote data, implement rate limits, and provide short job leases. It must never be the only location for swap state, action consumption, wallet binding, idempotency, financial events, or audit history.
