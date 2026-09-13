## Context

Animus owns the live Steel loop and already receipts selected steps. Selected receipts are internal and metadata only. The gap is universal coverage with external verifiability. This change closes that gap without changing loop semantics.

The Squeak workspace is the failure reference: live power without versioned evidence.

## Decisions

### Decision: New narrow pure family

**Choice:** Add `animus-adapter-action-receipt` as a no-std adapter family following the `animus-adapter-command` precedent.

**Rationale:** The stack prefers narrow components with independent tests over logic inside the loop runtime.

### Decision: Shell publishes through an injected store

**Choice:** `animus-std-action-receipt` owns the publish port and store handling. Recording is session scoped.

**Rationale:** The shell owns persistence while the core stays pure and store-free.

### Decision: Reuse existing gate verdicts

**Choice:** The receipt projection consumes existing model-request snapshot and tool-stage verdicts instead of duplicating them.

**Rationale:** This avoids double bookkeeping and keeps one authority path.

### Decision: Verify without a store

**Choice:** Receipts verify through the existing deterministic verifier.

**Rationale:** Verifiability must not depend on store availability.

## Risks / Trade-offs

- Coverage gaps: mitigated by the universality scenario and its negation fixture.
- Double bookkeeping: mitigated by reusing existing verdicts.
- Store coupling: mitigated by the injected port and store-free verification.
