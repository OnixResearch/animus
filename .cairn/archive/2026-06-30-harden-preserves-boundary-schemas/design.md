# Design: Preserves boundary schema hardening

## Context

Animus has an optional Preserves bridge and multiple planned wire surfaces. Without an explicit schema plan, each surface can invent ad hoc field names, versioning rules, and compatibility behavior.

## Decisions

### 1. Define schema coverage tiers

Tiered coverage distinguishes internal DTO round trips, stable external wire contracts, compatibility import/export adapters, and review-only fixture shapes. Stable external wire contracts require explicit schema identity and compatibility tests.

### 2. Version every boundary payload family

Runtime events, adapter requests/responses, final reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows carry schema family and version identity when they cross external boundaries.

### 3. Fail closed on malformed or unsafe payloads

Wrong kind, missing required fields, invalid text, unsupported versions, unsafe raw material, and unbounded payloads return typed boundary errors. Unknown future fields may be preserved only when the receiving boundary can prove they are safe metadata.

### 4. Keep schema validation at the boundary

Reusable core logic consumes typed Rust values after Preserves decoding and validation. Core behavior must not depend on raw Preserves AST inspection except in boundary parsing modules.

### 5. Maintain fixtures as durable evidence

Fixtures include valid minimal payloads, valid maximal payloads, version-skew payloads, unknown-safe metadata, and negative malformed/leak attempts. Text and binary forms should round trip when both are supported.

## Risks

- Schema coverage can overfit unstable internals. Mark pre-stable payloads as internal and avoid public compatibility promises.
- Compatibility preservation can leak data. Preserve unknown fields only when they are structurally safe and bounded.
