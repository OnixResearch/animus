# agent-harness-runtime Change Spec: Fabric-neutral runtime loop contracts

## Requirements

### Requirement: Fabric-neutral loop contracts

r[agent_harness_runtime.composability.fabric_neutral_contracts] Animus runtime loop contracts MUST use runtime-owned neutral turn and decision DTOs rather than Steel-specific types for reusable fabric input and output.

#### Scenario: Fabric decisions validate without Steel

r[agent_harness_runtime.composability.fabric_neutral_contracts.validation]
- GIVEN a runtime loop decision requests model dispatch, tool dispatch, final response, or safe failure completion
- WHEN neutral DTO validation runs
- THEN valid decisions are accepted without constructing Steel values
- AND malformed empty fields, unsupported decision kinds, or oversized safe text fail before adapter dispatch.

### Requirement: Loop fabric trait is implementation-neutral

r[agent_harness_runtime.composability.loop_fabric_trait] The `LoopFabric` trait MUST accept and return neutral runtime DTOs so embedders can provide non-Steel fabrics without depending on Steel crates or values.

#### Scenario: Non-Steel fabric drives a runtime loop

r[agent_harness_runtime.composability.loop_fabric_trait.non_steel]
- GIVEN a fake non-Steel fabric and injected model and tool adapters
- WHEN the runtime shell executes a loop
- THEN model dispatch, tool dispatch, final response, malformed decision, and missing-service paths use the same typed transition and failure semantics as the default fabric path
- AND no Steel script, Steel policy, or Steel value is required.

### Requirement: Steel integration is an adapter

r[agent_harness_runtime.composability.steel_adapter] Animus MAY provide Steel as a canonical loop fabric adapter, but Steel-specific decisions MUST be converted into neutral runtime DTOs and revalidated before model, tool, or final-response handling.

#### Scenario: Steel output cannot bypass runtime validation

r[agent_harness_runtime.composability.steel_adapter.validation]
- GIVEN a Steel script returns a dispatch or final-response decision
- WHEN the Steel adapter binds that output to the runtime loop contract
- THEN valid output is converted to neutral DTOs
- AND malformed Steel decisions, denied policy activation, or invalid adapter output produce typed fail-closed reports without dispatching unchecked effects.

### Requirement: Steel dependency is optional for neutral runtime composition

r[agent_harness_runtime.composability.steel_optional] The neutral runtime loop surface MUST compile and be documented without requiring Steel; Steel convenience runners MUST be behind explicit feature or integration boundaries.

#### Scenario: Steel-disabled runtime remains usable

r[agent_harness_runtime.composability.steel_optional.compile]
- GIVEN an embedder disables Steel integration and depends on the neutral runtime surface
- WHEN compile and facade inventory checks run
- THEN neutral loop DTOs, transition helpers, adapter dispatch, and fake-fabric tests remain available
- AND Steel runner APIs are absent or explicitly feature-gated rather than pulled in transitively.

### Requirement: Fabric decoupling verification

r[agent_harness_runtime.composability.fabric_decoupling.verification] The fabric decoupling change MUST include focused positive and negative tests for neutral DTO validation, non-Steel fabric execution, Steel adapter conversion, feature-gated compile surfaces, facade inventory, and Cairn gates before archive.

#### Scenario: Closeout evidence covers fabric composition

r[agent_harness_runtime.composability.fabric_decoupling.verification.closeout]
- GIVEN maintainers intend to archive the fabric decoupling change
- WHEN closeout validation runs
- THEN evidence names neutral-runtime, fake-fabric, Steel-adapter, feature-gate, facade-inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
