# Telegram UX and localization

## UI policy

Telegram Bot is the only MVP UI. Commands include `/start`, `/swap`, `/history`, `/wallet`, `/referrals`, `/settings`, and `/help`. Inline keyboards use short opaque action identifiers, never amounts, addresses, fee values, or mutable business data.

The authoritative action record is server-side, bound to user, aggregate revision, expiration, and one-time/idempotent consumption. Telegram update IDs and callback IDs are deduplicated durably. Callback queries receive an immediate acknowledgement; longer work continues asynchronously.

## Quote and approval UX

The quote message shows source/destination, expected output, minimum received, slippage, network-cost estimate, product fee, quote expiry, and a security warning. Confirmation creates a new canonical intent only after revalidation. Wallet approval screens must tell users to reject the request if wallet-visible destination, amount, asset, or conditions differ from the bot summary.

Pending language must be explicit: wallet approval, broadcast acknowledgement, and candidate observation are not final success. `ambiguous` is a visible state with a support/reconciliation path.

## Localization boundary

- English (`en`) is the sole MVP catalog and fallback locale.
- Store `preferred_locale`, Telegram locale snapshot, and locale source on the user.
- Domain/application layers return structured reason codes, never rendered copy.
- Presentation resolves stable `MessageKey` values with typed parameters.
- Notification events store message key, locale, template version, structured parameters, and delivery state.
- Callback semantics remain language-independent.
- Asset symbols and addresses are not translated; labels and explanations are.
- Amount, fee, percentage, timestamp, and transaction-reference formatters receive typed values and never reparse rendered strings.

## Rate limits and delivery

Outbound Telegram access is centralized. Respect `retry_after` for 429 responses, use bounded backoff with jitter for idempotent delivery, and write retries through the outbox. A failed edit falls back to a new message and updates the UI pointer without changing financial state.
