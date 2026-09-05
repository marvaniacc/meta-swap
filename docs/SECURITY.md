# Security requirements and threat model

## Trust boundaries

Untrusted inputs include Telegram updates, callback data, Mini App data if ever introduced, wallet/provider/bridge network responses, token metadata, indexer responses, and admin browser requests. PostgreSQL financial records and encrypted dApp session secrets are high-value assets. User private keys are intentionally outside the system boundary.

| Threat | Mitigation |
|---|---|
| Telegram spoofing/tampering | webhook secret validation, durable update dedupe, opaque expiring actions bound to user and revision |
| replay/duplicate request | database idempotency keys, unique constraints, one-time nonce/action consumption, optimistic locking |
| wallet/session hijack | per-user encrypted session keypair, `ton_proof`, strict domain/nonce/time validation, revoke and restore policy |
| transaction substitution | canonical intent hash, router/token allowlist, payload decode/compare, explicit UI disclosure |
| stale/manipulated quote | immutable quote snapshot, expiry, revalidation, minimum-received enforcement, provider sanity checks |
| fake/malicious token | curated chain-scoped allowlist, trusted decimals/address metadata, kill switch |
| fee/referral manipulation | versioned policy snapshots, realized-fee trigger, immutable ledger, approval/audit for config changes |
| admin escalation | RBAC, MFA, least privilege, dual approval for sensitive policy/payout changes, immutable audit |
| DB/Redis compromise | network isolation, least-privilege roles, encryption, backups, Redis non-authoritative design |
| denial/rate abuse | per-user/chat/IP limits, payload limits, timeouts, circuit breakers, queue backpressure |

## Secrets and logging

Store secrets in a secret manager or encrypted deployment store and rotate them. Restrict database and provider credentials by role. Never log private keys, seed phrases, session private keys, raw bridge plaintext/ciphertext, full BOCs, authorization headers, bot tokens, or sensitive user data. Audit events record actor, action, target, policy/template version, trace/correlation IDs, and redacted before/after hashes.
