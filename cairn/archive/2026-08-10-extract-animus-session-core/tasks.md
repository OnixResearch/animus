## Phase 1: Baseline

- [x] [serial] Record the published source identity and run focused baseline session tests. r[agent_harness_runtime.session_core_crate.compatibility]

## Phase 2: Boundary

- [x] [serial] Add `animus-session-core` with no-std, alloc, BLAKE3, and narrow adapter dependencies. r[agent_harness_runtime.session_core_crate]
- [x] [serial] Move the selected session family and required pure support reducers without host authority. r[agent_harness_runtime.session_core_crate.pure]
- [x] [serial] Preserve runtime module paths, root exports, features, schemas, and identities through re-exports. r[agent_harness_runtime.session_core_crate.compatibility]

## Phase 3: Enforcement

- [x] [parallel] Add positive, forbidden-host, and malformed manifest dependency fixtures. r[agent_harness_runtime.session_core_crate.dependencies]
- [x] [parallel] Add direct crate, facade, no-std, and Wasm checks with positive and negative cases. r[agent_harness_runtime.session_core_crate.validation]

## Phase 4: Ownership

- [x] [serial] Update README architecture, component catalog, runtime facade inventory, and checked policy evidence. r[agent_harness_runtime.session_core_crate.catalog]

## Phase 5: Completion

- [x] [serial] Run focused and workspace Cargo, Clippy, Octet, Cairn, catalog, facade, Wasm, and Nix checks. r[agent_harness_runtime.session_core_crate.validation]
- [x] [serial] Record bounded evidence, sync the accepted requirement, and archive the change. r[agent_harness_runtime.session_core_crate]
