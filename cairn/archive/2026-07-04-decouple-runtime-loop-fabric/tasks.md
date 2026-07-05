# Tasks

- [x] [serial] Define fabric-neutral runtime loop turn and decision DTOs with validated model request, tool request, final response, and failure decision variants. r[agent_harness_runtime.composability.fabric_neutral_contracts]
- [x] [depends:fabric_neutral_contracts] Add positive and negative DTO validation tests for valid dispatch/final decisions, malformed empty fields, unsupported decision kinds, and oversized safe text. r[agent_harness_runtime.composability.fabric_neutral_contracts.validation]
- [x] [serial] Change `LoopFabric` and runtime shell execution to consume neutral DTOs rather than Steel DTOs while preserving max-turn and fail-closed behavior. r[agent_harness_runtime.composability.loop_fabric_trait]
- [x] [depends:loop_fabric_trait] Add fake non-Steel fabric tests that run model dispatch, tool dispatch, final response, malformed decision, and missing-service paths without constructing Steel. r[agent_harness_runtime.composability.loop_fabric_trait.non_steel]
- [x] [serial] Implement a Steel integration adapter that converts checked Steel decisions into neutral runtime decisions before dispatch. r[agent_harness_runtime.composability.steel_adapter]
- [x] [depends:steel_adapter] Add positive Steel adapter tests and negative malformed Steel decision, denied policy, and adapter-output rejection tests. r[agent_harness_runtime.composability.steel_adapter.validation]
- [x] [serial] Gate Steel convenience runners and imports behind explicit feature or integration boundaries, and keep compatibility reexports documented. r[agent_harness_runtime.composability.steel_optional]
- [x] [depends:steel_optional] Add compile/facade checks proving runtime-neutral surfaces do not require Steel and Steel runners remain available when enabled. r[agent_harness_runtime.composability.steel_optional.compile]
- [x] [depends:fabric_neutral_contracts.validation,loop_fabric_trait.non_steel,steel_adapter.validation,steel_optional.compile] Update docs, facade inventory, and evidence with focused runtime, Steel, fake-fabric, and Cairn gate checks. r[agent_harness_runtime.composability.fabric_decoupling.verification]
