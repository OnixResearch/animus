# agent-harness-runtime Change Spec: Runtime facade composition inventory

## Requirements

### Requirement: Composition inventory metadata

r[agent_harness_runtime.composability.inventory_metadata] The runtime facade inventory MUST record feature gate, required authority, dependency links, and expected composition links for runtime-facing public exports or export groups.

#### Scenario: Inventory records carry composition metadata

r[agent_harness_runtime.composability.inventory_metadata.schema]
- GIVEN a runtime-facing public export or export group is listed in the inventory
- WHEN inventory schema validation runs
- THEN the record includes explicit or valid default composition metadata
- AND malformed or missing required metadata fails validation.

### Requirement: Inventory metadata validation

r[agent_harness_runtime.composability.inventory_validation] Inventory validation MUST enforce controlled authority values, known feature labels, known dependency targets, and known composition-link targets.

#### Scenario: Invalid inventory metadata is rejected

r[agent_harness_runtime.composability.inventory_validation.fixtures]
- GIVEN an inventory fixture has an unknown authority value, missing feature label, unknown dependency link, unknown composition link, or malformed metadata field
- WHEN inventory validation runs
- THEN validation fails with a typed diagnostic
- AND the invalid record is not accepted as facade evidence.

### Requirement: Reusable surfaces cannot depend upward on host authority

r[agent_harness_runtime.composability.inventory_dependency_direction] Inventory validation MUST reject reusable-contract exports that depend on shell adapters, terminal UI, process execution, persistence, network, provider discovery, credentials, or other ambient host authority surfaces.

#### Scenario: Hidden shell dependency fails inventory validation

r[agent_harness_runtime.composability.inventory_dependency_direction.no_upward_leak]
- GIVEN a reusable-contract inventory record declares or implies dependency on shell, terminal, process, persistence, network, provider, credential, or ambient host authority
- WHEN dependency-direction validation runs
- THEN the inventory check fails with a dependency-direction diagnostic
- AND maintainers must reclassify the export or move the dependency behind host injection.

### Requirement: Safe inventory evidence

r[agent_harness_runtime.composability.inventory_evidence] Runtime facade inventory checks MUST emit bounded safe evidence containing class counts, feature counts, authority counts, composition counts, and BLAKE3 inventory hashes.

#### Scenario: Inventory evidence omits sensitive data

r[agent_harness_runtime.composability.inventory_evidence.safe]
- GIVEN inventory validation emits evidence for review or closeout
- WHEN evidence leak checks run
- THEN raw prompts, provider payloads, credentials, full paths, and unbounded output are omitted or rejected
- AND evidence remains suitable for Cairn archive records.

### Requirement: Composition inventory verification

r[agent_harness_runtime.composability.inventory.verification] The inventory metadata change MUST include focused positive and negative tests for schema validation, metadata validation, dependency direction, safe evidence, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers inventory metadata behavior

r[agent_harness_runtime.composability.inventory.verification.closeout]
- GIVEN maintainers intend to archive the inventory metadata change
- WHEN closeout validation runs
- THEN evidence names schema, metadata, dependency-direction, safe-evidence, workspace, inventory rail, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
