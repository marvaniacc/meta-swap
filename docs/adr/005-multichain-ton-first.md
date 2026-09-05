# ADR 005: Multi-chain-ready, TON-first

**Decision:** Design chain-neutral core contracts but activate only TON intra-chain swaps in MVP.

**Recommendation:** Isolate chain mechanics behind adapters; do not expose cross-chain routes or bridges until separately designed and audited.

**Why:** EVM, Solana, Bitcoin, and TON have materially different transaction/finality models. Cross-chain introduces bridge and partial-completion risk.

**Alternatives:** TON-specific core; multi-chain/cross-chain launch.

**Trade-offs:** Early type discipline and abstraction cost; narrower initial product coverage.

**Risk:** Over-abstraction. Keep the shared core small and let adapters own chain mechanics.
