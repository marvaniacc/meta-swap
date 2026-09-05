# ADR 001: Modular monolith

**Decision:** Start with a Rust modular monolith with separable API and worker process roles.

**Recommendation:** Keep domain, application, infrastructure, presentation, and worker boundaries in one workspace; use PostgreSQL outbox/leases before introducing a broker or microservice.

**Why:** Financial state transitions, idempotency, audit, and recovery are easier to reason about in one transaction boundary.

**Alternatives:** Microservices; unstructured single crate.

**Trade-offs:** Independent scaling is deferred; module discipline is mandatory.

**Risk:** Coupling can grow. Enforce ports and dependency direction in review.
