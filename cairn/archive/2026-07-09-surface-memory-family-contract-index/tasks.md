# Tasks

- [x] [serial] Add `family:memory` to `docs/contracts-index.md` with owner module, stability posture, authority boundary, safe receipt behavior, serialization posture, focused surface, and focused checks. r[agent_harness_runtime.memory_contract_index.family_surface]
- [x] [depends:family_surface] Add memory family metadata and case dispositions to the checked adapter conformance manifest. r[agent_harness_runtime.memory_contract_index.conformance_manifest]
- [x] [depends:family_surface] Add bounded Agent Memory Techniques reference documentation and README reference with explicit non-adoption claims. r[agent_harness_runtime.memory_contract_index.bounded_reference]
- [x] [depends:conformance_manifest,bounded_reference] Add positive index/conformance tests and negative missing-family, unsafe-reference, stale-check, and leak fixtures. r[agent_harness_runtime.memory_contract_index.verification]
- [x] [depends:verification] Run contract-index checks, focused memory docs checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_contract_index.verification]
