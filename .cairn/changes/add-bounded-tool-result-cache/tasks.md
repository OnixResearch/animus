## Phase 1: Core decisions

- [ ] [serial] Implement the canonical key projection over tool request identity, normalized arguments, and optional capability context, backed by BLAKE3. r[animus.cache.projection]
- [ ] [serial] Implement the `no_std` retention decision core from supplied values. r[animus.cache.decision] r[animus.cache.deferral]
- [ ] [serial] Require an explicit capacity with no implicit default bound. r[animus.cache.bound]

## Phase 2: Standard shell store

- [ ] [depends:animus.cache.bound] Implement the bounded store in the std shell using core decisions. r[animus.cache.bound]
- [ ] [depends:animus.cache.deferral] Execute deferred releases strictly after the guard releases. r[animus.cache.deferral]
- [ ] [serial] Route cache misses to the existing tool adapter unchanged. r[animus.cache.boundary]

## Phase 3: Fixtures and evidence

- [ ] [parallel] Add positive fixtures for projection stability, bounded retention, eviction order, promotion at and below threshold, and deferred-release ordering. r[animus.cache.verification]
- [ ] [parallel] Add negative and boundary fixtures for missing bounds, reversed watermarks, zero capacity, single slot, threshold 0 and 100. r[animus.cache.verification]
- [ ] [serial] Document the claim boundary and the recorded cross-repo retention contract. r[animus.cache.boundary]
- [ ] [serial] Run workspace, Clippy, formatting, Cairn, and Nix checks, then record evidence. r[animus.cache.verification]
