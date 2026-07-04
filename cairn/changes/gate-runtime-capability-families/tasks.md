# Tasks

- [ ] [serial] Define the runtime capability-family map for loop, headless, provenance, batch, worker-team, policy/middleware, quality, research, scaffold, compaction, dataset export, and compatibility bundles. r[agent_harness_runtime.composability.capability_family_map]
- [ ] [depends:capability_family_map] Add inventory tests proving each mapped export belongs to exactly one primary family or an explicit shared-family class. r[agent_harness_runtime.composability.capability_family_map.inventory]
- [ ] [serial] Add Cargo feature gates and module/export guards for selected runtime capability families while preserving a compatibility bundle. r[agent_harness_runtime.composability.capability_feature_gates]
- [ ] [depends:capability_feature_gates] Add positive minimal, selected-family, compatibility, and all-family compile checks. r[agent_harness_runtime.composability.capability_feature_gates.compile]
- [ ] [serial] Update docs and README guidance so embedders choose minimal, selected-family, or compatibility features intentionally. r[agent_harness_runtime.composability.capability_feature_docs]
- [ ] [depends:capability_feature_docs] Add negative checks proving disabled families do not leak exports, docs examples, or required dependencies into minimal runtime surfaces. r[agent_harness_runtime.composability.capability_feature_docs.no_leak]
- [ ] [depends:capability_family_map.inventory,capability_feature_gates.compile,capability_feature_docs.no_leak] Update facade inventory and run focused feature-matrix, workspace, and Cairn gate checks with evidence. r[agent_harness_runtime.composability.capability_features.verification]
