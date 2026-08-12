# Design: Split Preserves boundary bridges by payload family

## Context

`docs/preserves-boundary-schemas.md` already distinguishes stable external wire contracts, compatibility import/export adapters, internal DTO round trips, and review-only fixtures. The crate graph should match those tiers so embedders can select only the serialization bridges they need.

## Decisions

### 1. Split by payload ownership

**Choice:** Adapter DTO messages, runtime event/report messages, provenance/session records, and policy-visible DTOs become separate bridge families with explicit dependency roots.

**Rationale:** Payload ownership determines which crates a bridge should depend on. Adapter DTO serialization should not require runtime modules; runtime event serialization should not require optional provenance/session records.

### 2. Preserve `animus-preserves` facade behavior

**Choice:** Existing `animus-preserves` exports remain available through facade re-exports while direct family imports are introduced for new code.

**Rationale:** This avoids breaking existing serialization consumers while letting minimal embedders choose smaller bridges.

### 3. Schema catalog records family ownership

**Choice:** Each schema descriptor records bridge family, coverage tier, owner crate, stable identity, metadata policy, and focused fixtures.

**Rationale:** A schema catalog is valuable only if it says which contracts are stable, which are compatibility-only, and which are review-only.

### 4. Revalidation remains mandatory after decode

**Choice:** Every bridge family decodes into typed DTOs and re-runs the owning DTO validators before exposing payloads to runtime, policy, host shell, or persistence code.

**Rationale:** Serialization must not bypass adapter/runtime validation or permit unsafe compatibility metadata.

## Risks / Trade-offs

- More bridge crates can create version coordination overhead. Mitigate with a shared schema identity policy and facade-level compatibility tests.
- Some payloads may straddle adapter/runtime ownership. Mitigate by assigning the schema descriptor to one owner and making dependencies explicit in the catalog.
- Direct crate splits may be too large for one wave. Mitigate by first adding family modules and catalog entries, then extracting crates in later waves.
