# agent-harness-runtime Change Spec: Add an Animus component catalog

## Requirements

### Requirement: Typed component catalog source

r[agent_harness_runtime.component_catalog.source] Animus MUST maintain a Nickel-authored component catalog that names reusable contracts, host-injection surfaces, shell adapters, optional serialization bridges, runtime preludes, crate features, and focused validation rails as typed data.

#### Scenario: Embedder can discover the smallest block

r[agent_harness_runtime.component_catalog.source.smallest_block]
- GIVEN an embedder needs a model-only, tool-only, runtime-loop, observability, context, serialization, or host-shell surface
- WHEN they inspect the component catalog projection
- THEN the catalog identifies the smallest component id, owner crate or module, public import guidance, feature gate, and focused check for that surface
- AND the catalog does not imply provider, filesystem, process, network, terminal, credential, or persistence authority unless that authority is explicitly classified as shell-owned.

### Requirement: Catalog metadata is authority-aware

r[agent_harness_runtime.component_catalog.metadata] Each component catalog entry MUST record component class, required authority, dependency links, composition links, capability family, focused checks, and safe evidence labels, and validation MUST reject unknown labels, unknown dependency targets, cycles, missing checks, and unsafe evidence fields.

#### Scenario: Unsafe catalog entry fails closed

r[agent_harness_runtime.component_catalog.metadata.rejects_unsafe]
- GIVEN a catalog entry claims reusable-contract status while requiring filesystem, network, credential, process, terminal, or persistence authority
- WHEN catalog validation runs
- THEN validation rejects the entry with a deterministic diagnostic
- AND generated docs are not updated from the invalid catalog.

### Requirement: Catalog projections stay in sync

r[agent_harness_runtime.component_catalog.generated_docs] Animus MUST generate or check a human-readable component catalog projection from the Nickel source and MUST keep README, contracts-index, and runtime-facade guidance linked to that projection without treating prose as the source of truth.

#### Scenario: Documentation drift is detected

r[agent_harness_runtime.component_catalog.generated_docs.drift]
- GIVEN the Nickel catalog changes a component feature gate, authority class, or focused check
- WHEN the catalog docs drift rail runs
- THEN the rail reports stale generated or linked documentation
- AND closeout cannot claim the catalog is current until the projection is regenerated or corrected.

### Requirement: Component catalog verification

r[agent_harness_runtime.component_catalog.verification] The component catalog change MUST include focused positive and negative fixtures for valid blocks, invalid authority labels, unknown dependencies, missing checks, unsafe evidence labels, generated-doc drift, and dependency cycles before archive.

#### Scenario: Closeout evidence covers catalog validity

r[agent_harness_runtime.component_catalog.verification.closeout]
- GIVEN maintainers intend to archive the component catalog change
- WHEN closeout validation runs
- THEN evidence names catalog fixtures, docs drift checks, runtime-facade cross-checks, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
