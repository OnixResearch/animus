# agent-harness-runtime Change Spec: Runtime capability feature gates

## Requirements

### Requirement: Runtime capability-family map

r[agent_harness_runtime.composability.capability_family_map] Animus MUST maintain a capability-family map for runtime public exports and modules so embedders can distinguish loop, headless, provenance, batch, worker-team, policy, quality, research, scaffold, compaction, dataset-export, shared, and compatibility surfaces.

#### Scenario: Each export has a family classification

r[agent_harness_runtime.composability.capability_family_map.inventory]
- GIVEN a runtime-facing public export is listed in the facade inventory
- WHEN capability-family validation runs
- THEN the export belongs to exactly one primary family or an explicit shared-family class
- AND unclassified exports fail validation before archive.

### Requirement: Capability feature gates

r[agent_harness_runtime.composability.capability_feature_gates] Runtime capability families SHOULD be gated by named Cargo features where doing so reduces dependency footprint, public surface area, or accidental authority coupling, while preserving an explicit compatibility bundle.

#### Scenario: Minimal and selected-family builds compile

r[agent_harness_runtime.composability.capability_feature_gates.compile]
- GIVEN an embedder builds minimal runtime, a selected capability family, the compatibility bundle, or all families
- WHEN compile checks run
- THEN enabled family exports and tests are available
- AND disabled family exports are absent rather than partially usable.

### Requirement: Feature selection documentation

r[agent_harness_runtime.composability.capability_feature_docs] Animus documentation MUST explain how embedders choose minimal, selected-family, compatibility, and all-family runtime feature sets, including authority and dependency implications.

#### Scenario: Disabled families do not leak into minimal docs or builds

r[agent_harness_runtime.composability.capability_feature_docs.no_leak]
- GIVEN a minimal-runtime build and its documented import path
- WHEN docs examples, compile checks, and facade inventory validation run
- THEN disabled headless, provenance, batch, worker-team, TUI, provider, process, and persistence families are not required
- AND any feature-required API is labeled with its required feature.

### Requirement: Capability feature verification

r[agent_harness_runtime.composability.capability_features.verification] The capability feature change MUST include focused positive and negative tests for family mapping, feature-gated compile profiles, disabled-family leakage, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers feature composition

r[agent_harness_runtime.composability.capability_features.verification.closeout]
- GIVEN maintainers intend to archive the capability feature change
- WHEN closeout validation runs
- THEN evidence names family-map, minimal compile, selected-family compile, compatibility compile, disabled-family leak, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
