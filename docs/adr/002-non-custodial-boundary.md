# ADR 002: Non-custodial boundary

**Decision:** User signing keys never enter the system.

**Recommendation:** Native wallets sign and broadcast user swap transactions. Backend-held TON Connect session keys are dApp transport secrets, not wallet signing keys.

**Why:** This is the product promise and removes the largest custody/security liability.

**Alternatives:** Embedded/custodial wallet or server-side signing.

**Trade-offs:** Wallet compatibility and user approval add UX complexity.

**Risk:** Session-secret mishandling can expose wallet communication. Encrypt, scope, rotate/revoke, and never treat sessions as signing authority.
