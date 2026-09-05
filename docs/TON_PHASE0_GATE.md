# TON integration gate (Phase 0.1)

This is intentionally a gate document, not an implementation specification. It must be replaced by evidence-backed TON integration documents before production wallet/provider code begins.

## Required evidence

1. Two compatibility-tested native wallets complete a bot-only, headless TON Connect HTTP-bridge connection on testnet.
2. Per-user session storage, SSE resume, disconnect/revoke, and `ton_proof` verification are demonstrated.
3. `sendTransaction` with the required raw custom message payload, explicit network, and `validUntil` works for approval, rejection, and expiry.
4. Chain observation independently verifies masterchain-final execution. Wallet response and message lookup remain UX-only correlation.
5. A provider/DEX testnet route exposes a payload that can be allowlisted, decoded, compared with intent, and used to verify actual output, minimum received, and realized fee.
6. The verified fee triggers exactly one referral reward event under duplicate/restart scenarios.

## Non-negotiable constraints

- Raw `ton://` links are not the primary swap transport.
- A user wallet address is not bound until a valid one-time `ton_proof` is verified.
- Unknown evidence transitions to `pending` or `ambiguous`, never success.
- No contract, token master, router, wallet provider, bridge endpoint, or RPC/indexer provider is production-approved until recorded in a reviewed allowlist/configuration version.
