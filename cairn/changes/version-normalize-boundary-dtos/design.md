# Design: Version and normalize boundary DTOs

## Context

Many Animus contracts already use typed newtypes, Draft structs, and validation helpers. The remaining risk is drift: consumers need to know which shape they are receiving, and maintainers need a clear rule for whether constructor inputs are normalized labels or deliberately preserved raw bodies.

## Decisions

### 1. Contract identities are stable data

**Choice:** Add stable identity strings such as `animus.model-request.v1`, `animus.tool-response.v1`, `animus.model-stream-frame.v1`, and `animus.safe-receipt.v1` near the DTOs they identify.

**Rationale:** Contract identities make docs, receipts, fixtures, and cross-language bridges explicit without forcing all consumers onto one serialization stack.

### 2. Safe labels normalize, raw bodies are explicit

**Choice:** Fields that are safe labels, names, identifiers, schema identities, capability labels, and receipt metadata store trimmed normalized text. Fields that intentionally preserve raw bodies use explicit raw/body types or documented constructors and remain bounded by policy.

**Rationale:** Normalization should not be accidental, and raw retention should be visible during review.

### 3. Version handling is fail-closed by default

**Choice:** Parsers and adapters that consume contract identities accept supported versions and fail closed on unsupported versions unless a compatibility rule explicitly maps the old version.

**Rationale:** Silent version drift can drop safety metadata.

### 4. Preserves remains an optional bridge

**Choice:** Version identities and fixtures live in reusable DTOs and docs; Preserves round-trip tests cover DTOs already exposed through `animus-preserves`.

**Rationale:** Contract versioning should help no-std embedders and cross-language bridges without making Preserves mandatory.

## Risks / Trade-offs

- Adding identities broadly can become churn. Start with public boundary DTOs that cross adapters, runtime events, streams, and receipts.
- Normalizing some existing fields can be behavior-affecting. Audit and preserve raw bodies only where documented.
- Version mismatch tests may require fixtures. Keep fixtures literal and small.
