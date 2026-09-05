# Financial correctness and ledger

## Source of truth

| Fact | Source of truth |
|---|---|
| Product intent, state, policy snapshots, audit and ledger | PostgreSQL |
| Actual execution, output, and finality | finalized TON blockchain evidence |
| Quote and route | provider input only; not final financial truth |
| Wallet response/BOC/message lookup | correlation and UX evidence only |
| Cache, rate limits, leases | Redis; never sole truth |

## Success definition

A swap is successful only when the verifier records finalized chain evidence that matches the immutable intent: correct network, verified wallet sender, allowlisted router/targets, expected input asset and amount, execution semantics, actual destination asset/output, minimum-received constraint, and product-fee evidence where applicable.

`pending`, `wallet_response_received`, `candidate_observed`, and `ambiguous` are not successful results. A top-level transaction status is insufficient: verification must inspect the route's relevant messages/actions/transfers and actual amounts.

## Fee and referral rules

- `fee_expected` is a quote-time prediction only.
- `fee_realized` exists only after verified execution proves the actual product fee.
- `fee_earned` is a finalized business accounting fact derived from `fee_realized`.
- A referral reward is deterministically derived only from an eligible `fee_realized` event.
- A failed, expired, rejected, or ambiguous swap earns neither a realized fee nor an available referral reward.

## Ledger

The ledger is append-only. Each entry has a stable ID, event type, asset, atomic amount, correlation ID, effective time, chain evidence reference when applicable, and policy version. Corrections are compensating entries; finalized records are never updated or deleted. Database permissions should prevent normal application roles from mutating finalized ledger rows.

## Idempotency

Every externally repeated operation has a durable key: Telegram update, callback action, swap command, quote acceptance, transaction intent, outbox event, chain observation, fee event, reward event, and payout item. Database uniqueness constraints are the final guard. Redis locks may reduce contention but never provide the sole guarantee.

## Reconciliation

A worker periodically reconciles verified executions, fee events, ledger entries, referral rewards, and payouts. Any missing or contradictory evidence becomes a durable mismatch event and alert; it must not be silently repaired or converted to success.
