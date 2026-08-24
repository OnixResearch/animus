## Phase 1: Independent target resolution

- [x] [serial] r[animus.dialect.target] Add versioned provider-transport and model-dialect identities to target resolution.
- [x] [parallel] r[animus.dialect.target.independent] Add a fixture that binds one transport to two distinct dialects.
- [x] [parallel] r[animus.dialect.target.unknown] Add negative unknown-provider and unknown-dialect fixtures.

## Phase 2: Ownership boundaries

- [x] [serial] r[animus.dialect.ownership] Split provider-host effects from deterministic dialect projection.
- [x] [parallel] r[animus.dialect.ownership.provider] Add a fixture that keeps auth, billing, transport, and continuation in the provider host.
- [x] [parallel] r[animus.dialect.ownership.dialect] Add a fixture that keeps prompt, tools, schema, instructions, and subagent protocol in the dialect profile.

## Phase 3: Snapshot and transition binding

- [x] [serial] r[animus.dialect.binding] Bind both identities and compatibility into the complete request snapshot.
- [x] [parallel] r[animus.dialect.binding.shared_transport] Add a shared-transport multi-family fixture.
- [x] [parallel] r[animus.dialect.binding.leak] Add a negative cross-family leakage fixture.
- [x] [serial] r[animus.dialect.transition] Enforce dialect compatibility during provider transitions.
- [x] [parallel] r[animus.dialect.transition.incompatible] Add a negative silent-generic-substitution fixture.

## Phase 4: Verification

- [x] [serial] r[animus.dialect.verification] Add the focused positive, negative, and boundary matrix and run the rails.
