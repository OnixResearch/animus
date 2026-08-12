# Design: Add router round-robin selection

## Context

The current router splits selection, execution-plan materialization, and execution dispatch cleanly. Selection and materialization are pure, while concrete provider execution is host-owned. Fallback is already policy-bound through `BudgetLedger`, `FallbackChain`, and `evaluate_fallback`, and adapter execution denies provider-side fallback bypasses.

The gap is candidate fairness. Catalog order currently acts as the implicit tie-breaker for eligible entries. That is deterministic, but it is not an explicit operator policy and it gives no receipt trail for why one equivalent candidate was chosen repeatedly.

## Decisions

### 1. Rotation state is host-owned input, not reusable persistence

**Choice:** Define safe rotation inputs such as `RouterRotationPolicy`, `RouterRotationCursor`, and `RouterRotationSnapshot` that hosts may supply to router selection/materialization. Reusable crates validate labels and cursor bounds, compute rotated order and next-cursor metadata, and never persist cursor state.

**Rationale:** Round-robin depends on request history. History storage belongs in the host shell; reusable logic remains pure and deterministic over explicit inputs.

### 2. Rotation runs only after eligibility filtering

**Choice:** Rotate only the candidate slice that has already passed route role, backend allowlist, privacy/locality, budget, duplicate-label, and recipe-family checks. The rotated order must preserve max-candidate caps, min-success quorum checks, and per-role separation.

**Rationale:** A cursor must never become authority. It can choose fairly among admitted candidates, but it cannot smuggle in an unauthorized model or backend.

### 3. Fallback rotation materializes an ordered chain, then stops

**Choice:** If policy enables rotation for fallback entries, apply the same pure rotation while building `FallbackChain`. After materialization, `evaluate_fallback` consumes that chain in order with existing budget and no-revisit rules; providers still cannot bypass the chain.

**Rationale:** This preserves the accepted requirement that fallback consumes an ordered chain while allowing hosts to distribute which fallback candidate appears first across requests.

### 4. Stale or malformed cursor state fails safely

**Choice:** Cursor labels, route keys, role keys, and optional catalog-generation hashes are validated. Malformed cursor material is rejected. Stale cursor/catalog generation mismatches produce a typed reset-or-deny decision according to explicit policy rather than silently using provider fallback.

**Rationale:** Operators need predictable behavior during catalog changes. A stale cursor should not cause unsafe candidate selection or hidden health fallback.

### 5. Receipts expose safe fairness evidence

**Choice:** Rotation receipts record recipe ID, route family, role labels, rotation policy label, cursor class, selected safe model labels, candidate count, catalog hash, next-cursor hash, and BLAKE3 identity. They reject raw prompts, provider payloads, health probes, credentials, endpoints, full paths, and unbounded traces.

**Rationale:** Round-robin should be auditable without leaking provider internals or request contents.

## Risks / Trade-offs

- Round-robin can reduce cache locality or warm-model preference. Keep it opt-in by policy.
- Cursor persistence bugs in host code can skew distribution. Receipts should make repeated cursor reuse visible.
- Rotating fallback chains changes which eligible fallback is tried first, so tests must prove existing budget, no-revisit, privacy, and provider-bypass denials still hold.
