# ADR 003: Telegram Bot-only UI

**Decision:** Do not use a Telegram Mini App in MVP.

**Recommendation:** Telegram messages and inline keyboards provide the product UI; native wallet apps provide connect/signing UX via a compatibility-tested headless TON Connect path.

**Why:** Preserve the simplicity of the bot interaction while retaining native-wallet approval.

**Alternatives:** Mini App; raw `ton://` swap links.

**Trade-offs:** Headless bridge integration and wallet compatibility testing are more complex.

**Risk:** Raw deep links may be unable to track signing or may produce blind-signing UX. They are not the primary swap path.
