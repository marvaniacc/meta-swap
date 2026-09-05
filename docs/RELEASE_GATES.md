# Release gates

## Phase 0.1 to TON integration

The following evidence is mandatory before a production TON Connect/provider adapter is implemented:

- Two native wallets pass bot-only headless TON Connect testnet flows.
- `ton_proof` binding, session restore, revoke, network mismatch, and expiry behavior are verified.
- `sendTransaction` with the relevant raw custom payload is approved/rejected correctly on testnet.
- Chain financial verification reaches masterchain-final evidence independently of wallet response or message lookup.
- At least one provider/DEX yields a testnet route whose payload, min-out, actual output, and fee can be verified.
- Referral reward creation from realized fee is idempotently demonstrated.

## Testnet beta to mainnet canary

- Reconciliation mismatches are zero or explicitly resolved.
- Security review covers wallet, provider, verifier, admin, and payout boundaries.
- Alerts, backups, restore drill, kill switch, and incident runbooks are tested.
- Token/router/wallet allowlists, amount limits, and provider fallback policy are approved.

## Mainnet canary to broader release

- Canary transactions have clean finalization and reconciliation history.
- No critical correctness, secret, or privilege finding remains open.
- Support and manual-review ownership are active.
- Payout remains controlled until its own end-to-end verification gate passes.
