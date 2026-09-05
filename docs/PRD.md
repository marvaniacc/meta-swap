# Product requirements: Meta Swap

## Product definition

Meta Swap is a non-custodial Telegram swap bot. A user chooses assets, obtains a quote, approves a transaction in their native wallet, and receives independently verified swap status and history in Telegram. The backend orchestrates intent, verification, accounting, referral attribution, and notifications; it never controls user assets.

## MVP scope

- Telegram Bot is the only product UI; use commands, messages, inline keyboards, and message edits.
- English is the only shipped locale, with localization-ready message keys and formatters.
- TON mainnet is the only active execution chain; testnet is required for development and acceptance.
- A curated TON/Jetton allowlist is the only supported asset universe.
- The bot supports quote, confirmation, native-wallet approval, transaction status, history, referral attribution, and affiliate reward visibility.
- The product is multi-chain-ready in its domain model, but does not support cross-chain swaps, bridges, arbitrary tokens, or internal user balances.

## Non-goals for MVP

- Telegram Mini App.
- Custody, private-key handling, seed recovery, or an internal tradable balance.
- Cross-chain swaps, bridge routing, or a universal token importer.
- Automatic, instant affiliate withdrawals.
- Support for every TON wallet; only compatibility-tested wallets are enabled.

## Primary user journey

1. A user starts `/swap`, selects source and destination assets, and enters an amount.
2. The bot displays an expiring quote: expected output, minimum received, slippage, network cost estimate, product fee, and relevant warnings.
3. The user confirms in the bot and approves the canonical request in a native wallet.
4. The backend observes and independently verifies final chain execution.
5. The bot reports verified success, verified failure, or a pending/ambiguous status; it never reports success from a wallet or provider response alone.
6. A verified realized product fee can create a pending referral entitlement for an eligible referrer.

## Product success criteria

- No user secret is exposed to the application.
- Every final financial result has chain evidence, an immutable intent, audit events, and ledger entries.
- Duplicate Telegram events, wallet replies, provider responses, and chain observations cannot produce a second swap, fee, reward, or payout.
- A restart preserves active conversations and all non-terminal financial workflows.
