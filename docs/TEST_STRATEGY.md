# Test strategy

## Required layers

- Unit tests: atomic amount parsing/rounding, fee/referral formulas, policy validation, state transitions, message-key parameter construction.
- Property tests: no invalid state transition reaches success; repeated commands/events do not create extra economic events; amount math preserves bounds.
- PostgreSQL integration tests: migrations, constraints, optimistic concurrency, outbox atomicity, ledger immutability, idempotency.
- Telegram tests: webhook secret, update/callback duplicates, callback tampering, 429 scheduling, edit fallback.
- Wallet/provider contract tests: request/response fixtures, expiry, payload validation, malformed/late responses.
- Testnet E2E: wallet proof, signing, chain finality, successful/failed/expired swaps, fee realization, referral reward.
- Failure and recovery tests: process kill at each non-terminal state, PostgreSQL/Redis/provider outage, indexer delay, duplicate chain event.
- Security tests: replay, forged proof, substituted payload, unauthorized admin change, secret/log scanning.

## Mocking policy

Mock Telegram, wallet transport, provider, clock, and chain-reader boundaries for deterministic unit tests. Use real PostgreSQL and Redis containers for integration tests. Use official testnet wallets, provider paths, and read APIs for E2E acceptance. No automated mainnet financial test runs outside a controlled canary environment.
