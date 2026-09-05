# Swap state machine

## States

| State | Meaning | Terminal |
|---|---|---:|
| `draft` | user started a flow | no |
| `awaiting_pair` / `awaiting_amount` | incomplete user input | no |
| `quoting` | provider request is in progress | no |
| `quote_available` | immutable, unexpired quote exists | no |
| `awaiting_confirmation` | user is reviewing quote | no |
| `building_intent` | quote is being revalidated and canonicalized | no |
| `awaiting_wallet_approval` | valid transaction request is pending user approval | no |
| `wallet_response_received` | wallet response is recorded; not final | no |
| `chain_candidate_observed` | possible matching chain transaction observed | no |
| `financial_verification_pending` | verifier awaits final evidence | no |
| `swap_succeeded` | on-chain execution verified | no; finalization follows |
| `swap_failed_onchain` | failure/bounce/invariant failure verified | no; finalization follows |
| `ambiguous` | evidence incomplete or conflicting | no |
| `finalized` / `finalized_failed` / `cancelled` | immutable terminal outcome | yes |

## Transition rules

- User input produces a new revision under optimistic locking.
- Quote confirmation revalidates expiry, user, assets, amount, slippage, policy versions, wallet binding, and route allowlist before entering `building_intent`.
- A valid immutable intent transitions to `awaiting_wallet_approval`; `valid_until` is enforced by both request policy and expiry workers.
- Wallet rejection returns to an unexpired quote if policy allows. No automatic signing, request recreation, or transaction rebroadcast occurs.
- Chain evidence drives `financial_verification_pending` to verified success, verified failure, or ambiguity. Only the verifier can enter success/failure states.
- Finalization writes execution result, ledger/audit events, fee/referral events, and outbox notification in one database transaction.

## Recovery and timeouts

All non-terminal states are durable and resumable. Quote and intent expiry workers perform deterministic transitions. Verification jobs retry reads with bounded backoff and fallback providers. A broadcast or wallet timeout causes chain discovery; it never authorizes blind resubmission. Ambiguity remains visible and is escalated to reconciliation/manual review.
