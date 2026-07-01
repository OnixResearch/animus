# Tasks

- [x] [serial] Define Animus-owned Hermes-style skill manifest DTOs with name, description, version, category, tags, related skills, provenance, trust status, and content identity. r[agent_harness_runtime.hermes_skill_manifest.descriptor]
- [x] [depends:descriptor] Add positive fixture tests proving valid Hermes-style manifests become skill resource summaries. r[agent_harness_runtime.hermes_skill_manifest.descriptor.valid]
- [x] [serial] Add pure manifest validation for invalid names, missing/overlong descriptions, unsupported versions, duplicate tags, missing body identities, malformed related names, and unsafe handles. r[agent_harness_runtime.hermes_skill_manifest.validation]
- [x] [depends:validation] Add negative manifest fixture tests for every validation failure class. r[agent_harness_runtime.hermes_skill_manifest.validation.invalid]
- [x] [serial] Map manifests into progressive skill disclosure without loading full bodies by default. r[agent_harness_runtime.hermes_skill_manifest.progressive_disclosure]
- [x] [depends:progressive_disclosure] Add prompt-construction tests proving explicit load includes only the selected skill body. r[agent_harness_runtime.hermes_skill_manifest.progressive_disclosure.selected]
- [x] [serial] Document and test that Hermes filesystem layout remains host policy and reusable crates consume descriptors only. r[agent_harness_runtime.hermes_skill_manifest.layout_boundary]
- [x] [depends:layout_boundary] Add host-store isolation tests proving no reusable runtime path inspects `~/.hermes`, `~/.pi`, or project skill directories. r[agent_harness_runtime.hermes_skill_manifest.layout_boundary.host_store]
- [x] [depends:descriptor.valid,validation.invalid,progressive_disclosure.selected,layout_boundary.host_store] Update docs and run focused manifest tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_skill_manifest.verification]
