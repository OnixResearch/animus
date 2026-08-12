# Design: Provider stream parser rails

## Context

Streaming parser work already verifies several fixture paths, and provider parity docs call out parser-entrypoint tests. The improvement area is hardening parser behavior at raw provider boundaries so malformed chunks and over-budget streams return typed failures rather than assertions, panics, or late memory limits.

## Decisions

### 1. Parser entrypoints are total over input

**Choice:** Public parser entrypoints return typed `StreamParseFailure`-style outcomes for malformed data, unsupported events, invalid order, duplicate terminals, and missing terminals.

**Rationale:** Provider input is untrusted; panics are host denial-of-service risks.

### 2. Budgets apply before aggregation

**Choice:** Enforce frame bytes, record bytes, aggregate bytes, record count, event count, and completion budgets before buffering or concatenating chunks.

**Rationale:** Checking after concatenation does not prevent memory growth.

### 3. Raw fixture tests enter at parser boundary

**Choice:** Tests feed provider-like raw chunks/SSE/JSON lines into parser entrypoints and assert normalized event outcomes.

**Rationale:** Already-normalized event fixtures miss framing, UTF-8, and chunk-boundary failure modes.

### 4. Stream receipts stay summarized

**Choice:** Failure receipts include parser class, budget kind, counts, safe provider family, and BLAKE3 chunk identities where useful, never raw chunks.

**Rationale:** Malformed provider streams can contain sensitive or adversarial payloads.

## Risks / Trade-offs

- More failure classes can complicate callers. Keep classes coarse enough for policy and tests.
- Strict budgets may reject provider quirks. Make budgets configurable by admitted provider policy.
- Parser-entrypoint fixtures can grow large. Keep fixtures minimal and targeted.
