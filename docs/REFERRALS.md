# Referral and affiliate accounting

## Product rule

Referral is available from MVP, but it is not a custodial user balance. A referrer earns a policy-defined share only from a verified, realized product-fee event.

```text
verified execution + realized fee + eligible immutable attribution
-> reward_pending -> vested/held -> payable -> verified_paid
```

## Attribution

- A new user enters through `/start ref_<code>`.
- The backend validates and records an attribution candidate; callback or client-provided referrer data is not trusted.
- At first eligible action, attribution becomes bound and immutable.
- A user has at most one referrer. Referrer and referred user cannot be the same account.
- Wallet overlap, unusual account behavior, and other risk signals can hold or reject rewards but do not rewrite historical attribution.

## Economics

- Reward equals `realized_product_fee_atomic * referral_share_bps`, rounded by an explicit versioned policy.
- Quote, fee, referral share, and policy versions are snapshotted for audit.
- Failed or ambiguous swaps produce no available reward.
- Reward is idempotent by realized fee event and referral relationship.

## Payout model

Affiliate rewards vest after a policy/risk window and become payable only when a verified payout address and minimum threshold exist. Payouts are controlled batches from business treasury funds, require approved operations, and become `paid` only after independent chain verification. The treasury is never part of user swap custody.

## Bot UX

`/referrals` exposes a referral link, share action, invited/active counts, pending/vested/paid reward summaries, reward history, payout details, and referral rules. All figures use message keys and typed asset formatting.
