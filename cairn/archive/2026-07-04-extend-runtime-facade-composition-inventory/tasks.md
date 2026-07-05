# Tasks

- [x] [serial] Extend the runtime facade inventory record schema with feature gate, required authority, dependency links, and expected composition links. r[agent_harness_runtime.composability.inventory_metadata]
- [x] [depends:inventory_metadata] Add positive schema tests for existing inventory records with defaulted or explicit metadata. r[agent_harness_runtime.composability.inventory_metadata.schema]
- [x] [serial] Update inventory validation to enforce controlled authority values, known feature labels, known dependency targets, and known composition-link targets. r[agent_harness_runtime.composability.inventory_validation]
- [x] [depends:inventory_validation] Add negative fixtures for missing feature labels, unknown authority values, unknown links, and malformed metadata. r[agent_harness_runtime.composability.inventory_validation.fixtures]
- [x] [serial] Add dependency-direction checks that reject reusable contracts depending on shell, terminal, process, persistence, network, provider, credential, or ambient host authority surfaces. r[agent_harness_runtime.composability.inventory_dependency_direction]
- [x] [depends:inventory_dependency_direction] Add negative fixtures proving hidden shell dependency, terminal dependency, provider discovery, and process authority links are rejected. r[agent_harness_runtime.composability.inventory_dependency_direction.no_upward_leak]
- [x] [serial] Update inventory docs and safe evidence output with class counts, feature counts, authority counts, composition counts, and BLAKE3 inventory hash. r[agent_harness_runtime.composability.inventory_evidence]
- [x] [depends:inventory_evidence] Add leak tests proving evidence omits raw prompts, provider payloads, credentials, full paths, and unbounded output. r[agent_harness_runtime.composability.inventory_evidence.safe]
- [x] [depends:inventory_metadata.schema,inventory_validation.fixtures,inventory_dependency_direction.no_upward_leak,inventory_evidence.safe] Run focused inventory rail, workspace checks, and Cairn gates with evidence. r[agent_harness_runtime.composability.inventory.verification]
