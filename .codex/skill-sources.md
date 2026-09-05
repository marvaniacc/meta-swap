# Meta Swap skill provenance and review

## Governance

All project-local skills were authored for this repository from the reviewed Meta Swap documents listed below. They are concise instruction files only: they contain no executable scripts, credentials, production endpoints, copied external skill content, or dependency changes. Review status is **accepted for project-local governance**.

| Skill | Purpose and scope | Primary internal sources | Review status |
|---|---|---|---|
| `rust-financial-backend` | Rust domain, application, persistence, and worker workflow. | `AGENTS.md`; `docs/ARCHITECTURE.md`; `docs/DATA_MODEL.md`; `docs/FINANCIAL_CORRECTNESS.md`; `docs/SWAP_STATE_MACHINE.md`; ADR 001/002/004. | Accepted; authored locally. |
| `telegram-bot-production` | Telegram presentation, update/callback safety, delivery, and localization. | `AGENTS.md`; `docs/TELEGRAM_UX_I18N.md`; `docs/SECURITY.md`; `docs/TEST_STRATEGY.md`; ADR 003. | Accepted; authored locally. |
| `noncustodial-financial-correctness` | Intent, finality, ledger, idempotency, reconciliation, and recovery. | `AGENTS.md`; `docs/FINANCIAL_CORRECTNESS.md`; `docs/DATA_MODEL.md`; `docs/SWAP_STATE_MACHINE.md`; ADR 002/004/006. | Accepted; authored locally. |
| `ton-wallet-chain-verification` | Gate-bound TON sessions, adapter boundaries, and independent verification. | `AGENTS.md`; `docs/TON_PHASE0_GATE.md`; `docs/RELEASE_GATES.md`; `docs/MULTICHAIN.md`; ADR 002/004/005. | Accepted for interfaces/review only; production integration blocked by Phase 0.1. |
| `multichain-adapter-development` | Chain-neutral values and future adapter isolation. | `AGENTS.md`; `docs/MULTICHAIN.md`; `docs/ARCHITECTURE.md`; `docs/TON_PHASE0_GATE.md`; ADR 001/004/005. | Accepted; no new chain activation. |
| `referral-and-affiliate-accounting` | Immutable attribution, realized-fee rewards, lifecycle, and payout accounting. | `AGENTS.md`; `docs/REFERRALS.md`; `docs/FINANCIAL_CORRECTNESS.md`; `docs/DATA_MODEL.md`; ADR 004/006. | Accepted; no payout execution. |
| `security-review-financial-bot` | Threat-driven review of financial-bot boundaries and release gates. | `AGENTS.md`; `docs/SECURITY.md`; `docs/FINANCIAL_CORRECTNESS.md`; `docs/TON_PHASE0_GATE.md`; ADR 002–006. | Accepted; review workflow only. |
| `rust-test-and-recovery` | Rust correctness, concurrency, duplicate delivery, and restart testing. | `AGENTS.md`; `docs/TEST_STRATEGY.md`; `docs/SWAP_STATE_MACHINE.md`; `docs/FINANCIAL_CORRECTNESS.md`; ADR 001/004/006. | Accepted; testing workflow only. |

## Candidate external skill inventory

| Candidate / source URL | Publisher / owner | License | Maintenance evidence | Intended use | Security risk | Recommendation |
|---|---|---|---|---|---|---|
| Local Codex `skill-creator` at `/opt/codex/skills/.system/skill-creator/SKILL.md`; upstream guidance: <https://github.com/openai/skills> | OpenAI | Apache-2.0 (local `license.txt`) | Present in the Codex environment; supplies current initializer and validator scripts. | Create and structurally validate local skills. | Generic authoring guidance does not encode Meta Swap financial constraints; its initializer adds UI metadata. | Use only as a reviewed authoring reference; do not copy it as a project skill. |
| <https://core.telegram.org/bots/api> | Telegram | No repository license identified for documentation content. | Official Bot API documentation. | Reference only when later validating Telegram protocol behavior. | Protocol claims can change; credentials, webhook secrets, and untrusted updates require strict handling. | Use only as reference; no external skill installed or copied. |
| <https://docs.ton.org/> | TON Foundation / TON documentation maintainers | No repository license identified for documentation content. | Official TON documentation portal. | Reference only when Phase 0.1 evidence permits TON protocol work. | Protocol assumptions or third-party endpoints could bypass finality/gate requirements. | Use only as reference; no external skill installed or copied. |
| <https://www.rust-lang.org/learn> | Rust Project | Documentation terms not imported or relied upon. | Official Rust project learning/documentation entry point. | Reference only for language/toolchain facts. | Generic material cannot establish financial correctness or persistence guarantees. | Use only as reference; no external skill installed or copied. |

The external search interface returned an authentication error during discovery. No unreviewed source was substituted; the final skill set is deliberately based on the repository’s reviewed internal documents. No candidate was accepted as an external installed skill.

## Security review of project-local skills

| Skill | Review result | Security controls verified |
|---|---|---|
| `rust-financial-backend` | Approved | Atomic integer arithmetic, PostgreSQL durability, append-only events, idempotency, transaction boundaries, and recovery are mandatory. |
| `telegram-bot-production` | Approved | Webhook/callback validation, opaque server-side actions, durable dedupe, localization boundary, and non-final status wording are mandatory. |
| `noncustodial-financial-correctness` | Approved | Immutable intent, independently verified finality, no external-ack finalization, reconciliation, and custody prohibition are mandatory. |
| `ton-wallet-chain-verification` | Approved with Phase 0.1 block | Signing material prohibition, allowlists, proof/session controls, independent masterchain-final evidence, and gate-only scope are mandatory. |
| `multichain-adapter-development` | Approved | Chain mechanics are isolated, asset identity is canonical and typed, non-TON activation requires evidence, and bridges/cross-chain flows are prohibited. |
| `referral-and-affiliate-accounting` | Approved | Rewards require one verified realized fee, attribution is immutable, lifecycle events are append-only/idempotent, and payouts require independent verification. |
| `security-review-financial-bot` | Approved | Trust-boundary review, logging/secret controls, authorization, replay, finality, and release-gate findings are required. |
| `rust-test-and-recovery` | Approved | Property/integration testing, duplicate delivery, fault injection, restart recovery, and no signing-action replay are mandatory. |

## Scope confirmation

This change adds governance documentation and local skill instructions only. It adds no Rust application code, database schema, Telegram integration, wallet integration, provider integration, mainnet transaction, cross-chain flow, custodial function, dependency, secret, endpoint, or executable installation script.
