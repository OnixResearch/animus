# agent-harness-runtime Change Spec: Split adapter families into focused crates

## Requirements

### Requirement: Adapter facade remains compatible

r[agent_harness_runtime.adapter_family_crates.facade] Animus MUST preserve the `animus-adapters` facade while extracting selected adapter families into focused reusable crates, and each moved public item MUST remain available through its previous facade path during the migration window.

#### Scenario: Existing adapter imports continue to compile

r[agent_harness_runtime.adapter_family_crates.facade.compat]
- GIVEN downstream code imports a moved model, tool, context, observability, safe-receipt, workspace, memory, policy, or orchestration item from `animus_adapters`
- WHEN that family is extracted into a focused crate
- THEN the previous `animus_adapters` import path continues to resolve through a documented re-export shim
- AND parity tests prove the direct crate item and facade item have identical signatures.

### Requirement: Extracted family crates stay independent and authority-free

r[agent_harness_runtime.adapter_family_crates.independent_crates] Extracted adapter-family crates MUST own implementation-free DTOs, validation helpers, traits, and safe receipt primitives only, and reusable family crates MUST NOT read files, inspect environment variables, spawn processes, open networks, access credentials, depend on terminal state, use clocks, or perform persistence.

#### Scenario: Family crate is usable as a small Lego block

r[agent_harness_runtime.adapter_family_crates.independent_crates.smallest_dependency]
- GIVEN an embedder needs only one extracted family such as safe receipts or observability
- WHEN they depend on that family crate directly
- THEN they do not pull unrelated model, tool, memory, workspace, provider, filesystem, process, network, terminal, credential, or persistence behavior
- AND the crate passes no-std or documented no-ambient-authority purity checks.

### Requirement: Adapter-family migration is cataloged

r[agent_harness_runtime.adapter_family_crates.migration] Every extracted adapter family MUST update the component catalog, contracts index, runtime facade inventory, adapter conformance index, and crate-level documentation with owner, feature, dependency, authority, and focused-check metadata.

#### Scenario: Missing catalog metadata blocks closeout

r[agent_harness_runtime.adapter_family_crates.migration.catalog]
- GIVEN an adapter family has moved into a focused crate
- WHEN catalog and conformance validation runs
- THEN missing owner, feature, dependency, authority, or focused-check metadata is reported as drift
- AND the change cannot be archived until the catalog and docs name the new block boundary.

### Requirement: Adapter-family split verification

r[agent_harness_runtime.adapter_family_crates.verification] Adapter-family crate splits MUST include focused positive and negative tests for direct crate usage, facade re-export parity, no-std or no-ambient-authority purity, conformance-index coverage, dependency-cycle rejection, missing-export rejection, and safe receipt leak rejection.

#### Scenario: Closeout evidence covers the moved family

r[agent_harness_runtime.adapter_family_crates.verification.closeout]
- GIVEN maintainers intend to archive an adapter-family extraction wave
- WHEN closeout validation runs
- THEN evidence names per-family tests, facade parity, purity checks, catalog checks, adapter conformance checks, Wasm checks where applicable, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
