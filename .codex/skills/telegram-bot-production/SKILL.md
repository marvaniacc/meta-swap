---
name: telegram-bot-production
description: Guide production-safe Telegram Bot presentation, webhook, callback, notification, and localization work. Use for Telegram commands, updates, keyboards, message delivery, and bot UX changes.
---

# telegram-bot-production

## Purpose
Guide production-safe Telegram Bot presentation, webhook, callback, notification, and localization work. Use for Telegram commands, updates, keyboards, message delivery, and bot UX changes.

## When it applies
Apply this workflow whenever its described area is designed, implemented, changed, or reviewed.

## Required reading
- Read `AGENTS.md`, `docs/ARCHITECTURE.md`, `docs/TELEGRAM_UX_I18N.md`, `docs/DATA_MODEL.md`, `docs/SECURITY.md`, `docs/TEST_STRATEGY.md`, and ADR 003.
- Keep presentation separate from application, ledger, and verification logic.

## Mandatory baseline
- Preserve the non-custodial boundary: never request, accept, store, transmit, or log user signing material (private keys, seed phrases, mnemonics, or signing secrets).
- Keep Telegram Bot as the UI; do not introduce a Mini App. Activate only TON intra-chain execution in the MVP; do not add bridges or cross-chain swaps.
- Use typed atomic integer amounts and explicit, recorded rounding; never use floating point for assets, fees, rewards, or slippage.
- Keep PostgreSQL as durable financial and authorization truth. Redis may cache, rate-limit, or coordinate only.
- Treat wallet, provider, broadcast, BOC, and message-lookup responses as correlation/UX evidence, never financial finality. Finalize only after independently verified finalized chain evidence matches immutable intent.
- Make every financial transition idempotent, append-only, auditable, and restart-safe. Create referral rewards only from verified realized product fees.
- Use stable localization keys for user-facing text; domain and application code return structured reasons, not rendered copy.

## Non-negotiable checks
- Validate webhook authenticity; durably deduplicate update IDs and callbacks. Use opaque, expiring, user- and revision-bound server-side actions.
- Acknowledge callbacks promptly; send retries through the outbox and respect rate limits. Render only stable message keys with typed parameters.
- Use explicit pending/ambiguous copy: wallet/provider responses do not prove success.

## Prohibited actions
- Do not place amounts, addresses, fees, or mutable business facts in callback data; render business text in domain/application layers; introduce Mini Apps; or let delivery failures mutate financial state.

## Expected output
- Deliver message-key and action-contract changes, error/retry behavior, and user-visible pending/finality semantics.

## Required testing and review
- Add webhook, tampered/duplicate callback, localization, 429/outbox, and edit-fallback tests as applicable.

## Escalate and stop
- Stop and report undefined callback authorization, localization ownership, UX wording that implies unverified success, or a proposed Mini App.
