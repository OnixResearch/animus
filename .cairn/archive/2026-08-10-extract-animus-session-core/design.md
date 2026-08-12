## Context

The approved workspace decomposition selects one authority-coherent session family from `animus-runtime`. The published source is `526519b8b66a68a62aece40f0d5b6c6ed4e6a08e`. The selected modules are pure, but they currently share the broad runtime crate boundary.

## Decisions

### Decision: Add one no-std session core crate

**Choice:** Add `animus-session-core` with `#![no_std]`, `alloc`, BLAKE3, and the narrow `animus-adapters` DTO dependency. Move session authority profiles, runtime authority seals, event replay, conversation projection, branch planning and projection, active-turn recovery, interrupted-call repair, redacted export planning, and their pure support reducers.

**Rationale:** This is the smallest compiler-checked boundary that isolates the approved session family. Moving the pure agent-control reducer and branch planner avoids a dependency cycle without moving host authority.

### Decision: Preserve the runtime facade

**Choice:** Make `animus-runtime` depend on `animus-session-core` and re-export each current module and root item from its existing path. Keep current feature names and defaults.

**Rationale:** Existing consumers should not need source, feature, schema, or identity changes during the first compatibility release.

### Decision: Enforce authority by manifest and target checks

**Choice:** Admit only `animus-adapters` and BLAKE3 in the new crate. Add valid, forbidden-host, and malformed manifest fixtures. Add direct no-default and `wasm32-unknown-unknown` checks.

**Rationale:** Source layout alone cannot prevent host authority from entering the reusable crate.

### Decision: Keep catalogs authoritative

**Choice:** Update the component catalog and runtime facade inventory to name `animus-session-core` as the implementation owner while retaining `animus-runtime` as the compatibility owner.

**Rationale:** The checked catalogs must describe both implementation ownership and stable public paths.

## Risks / Trade-offs

- Moving several related modules can expose hidden dependencies. Stop if a host dependency or cycle is required.
- `no_std` conversion can change imports. Tests must prove that public values, schemas, identities, and receipts remain stable.
- Catalog and exact-byte evidence can require reviewed regeneration. Do not accept unrelated drift.
