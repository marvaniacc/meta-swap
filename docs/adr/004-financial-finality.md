# ADR 004: Independent financial finality

**Decision:** Finalize swaps only from independently verified, masterchain-final execution evidence.

**Recommendation:** Wallet/provider responses and message lookup only improve correlation and UX. The verifier checks the immutable intent against finalized on-chain execution before ledger, fee, or referral finalization.

**Why:** External responses can be absent, duplicated, stale, or semantically incomplete.

**Alternatives:** Treat provider or wallet callback as success.

**Trade-offs:** More pending/ambiguous states and verification infrastructure.

**Risk:** Indexer delay/disagreement. Use independent read paths, reconciliation, and manual review.
