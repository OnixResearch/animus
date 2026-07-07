# agent-harness-runtime Change Spec: Introduce the animus-router crate barrier

## Requirements

### Requirement: Dedicated router crate boundary

r[agent_harness_runtime.model_router.crate_barrier] Animus MUST own a dedicated `animus-router` workspace crate that is `#![no_std]` with `alloc` only and that owns the routing decision surface (recipe/role/output-contract DTOs, request-signal projection, recipe selection, execution-plan materialization, route-result aggregation, safe receipt assembly, and router-specific field validation), while `animus-adapters` keeps implementation-free model/tool/port contracts and exposes only a transition re-export shim for router items.

#### Scenario: Embedders depend on the router without all adapter contracts

r[agent_harness_runtime.model_router.crate_barrier.smallest_dependency]
- GIVEN an embedder needs only deterministic routing decisions
- WHEN they select workspace crates
- THEN they can depend on `animus-router` without pulling model, tool, host-effect, provider, filesystem, process, network, terminal, credential, or persistence behavior
- AND the crate does not depend on model, tool, host-effect, provider, filesystem, process, network, terminal, credential, or persistence crates
- AND the only temporary dependency direction for compatibility is `animus-adapters` depending on `animus-router` for the re-export shim.

#### Scenario: Router crate stays pure and Wasm-eligible

r[agent_harness_runtime.model_router.crate_barrier.no_std_pure]
- GIVEN the `animus-router` crate is compiled
- WHEN the no-std purity smoke and Wasm surface rail run
- THEN the crate contains no `std::fs`, `std::env`, `std::process`, `std::time`, `println!`, `eprintln!`, `tokio`, `async`, filesystem traversal, environment reads, process execution, or JSON rendering helpers
- AND `cargo check -p animus-router --target wasm32-unknown-unknown --locked` passes.

### Requirement: Transition re-export preserves the public surface

r[agent_harness_runtime.model_router.crate_barrier.reexport] `animus-adapters::router` MUST re-export the moved `animus-router` public items for a transition window so existing dependents compile without churn, and a parity test MUST assert the re-exported surface is item-for-item identical to the moved module.

#### Scenario: Existing call sites compile unchanged

r[agent_harness_runtime.model_router.crate_barrier.reexport.unchanged]
- GIVEN call sites import router items from `animus_adapters::router`
- WHEN the module is moved into `animus-router` and a re-export shim is added
- THEN the workspace compiles without editing those call sites
- AND a parity test confirms every re-exported item resolves to the moved `animus-router` item with identical signatures.

### Requirement: Router crate barrier verification

r[agent_harness_runtime.model_router.crate_barrier.verification] The crate barrier change MUST include focused positive and negative tests for workspace registration, re-export parity, Wasm surface eligibility, no-std purity, conformance-index registration, facade inventory, and receipt-free closeout before archive.

#### Scenario: Closeout evidence covers the crate barrier

r[agent_harness_runtime.model_router.crate_barrier.verification.closeout]
- GIVEN maintainers intend to archive the router crate barrier change
- WHEN closeout validation runs
- THEN evidence names focused crate-registration, re-export parity, Wasm-surface, purity-smoke, conformance-index, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
