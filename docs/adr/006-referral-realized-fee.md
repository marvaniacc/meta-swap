# ADR 006: Referral rewards derive from realized fees

**Decision:** Affiliate entitlement is created only after a verified realized product fee.

**Recommendation:** Use immutable attribution, append-only reward events, vesting/risk holds, and controlled payout batches from business funds.

**Why:** It prevents reward creation for failed/ambiguous swaps and avoids an internal custodial balance.

**Alternatives:** Reward on quote, volume, or wallet submission; instant withdrawal balance.

**Trade-offs:** Affiliates wait for verification/vesting and payouts require operations.

**Risk:** Fraud or payout error. Apply idempotency, anomaly holds, address verification, approvals, and on-chain payout verification.
