# Tasks

- [x] [serial] Define resource descriptor DTOs for skills, prompt templates, context guidance, and host-provided resources with kind, name, description, scope, provenance, trust status, and content identity. r[agent_harness_runtime.pi_resources.descriptors]
- [x] [depends:descriptors] Add descriptor validation tests for missing descriptions, invalid names, unsupported kinds, unsafe content handles, and malformed provenance. r[agent_harness_runtime.pi_resources.descriptors.validate]
- [x] [serial] Define host-owned discovery boundary APIs where runtime consumes descriptors but does not scan directories or load packages. r[agent_harness_runtime.pi_resources.host_discovery]
- [x] [depends:host_discovery] Add purity/boundary tests or static checks proving runtime resource construction performs no filesystem traversal, package install, or dynamic code loading. r[agent_harness_runtime.pi_resources.host_discovery.no_scan]
- [x] [serial] Add project-trust status handling for project-local resource descriptors before prompt construction, command expansion, or tool activation. r[agent_harness_runtime.pi_resources.project_trust]
- [x] [depends:project_trust] Add negative tests proving untrusted project resources are not advertised or loaded into context. r[agent_harness_runtime.pi_resources.project_trust.denied]
- [x] [serial] Add progressive skill disclosure support for advertised name/description and explicit host-mediated full-content loading. r[agent_harness_runtime.pi_resources.skill_disclosure]
- [x] [depends:skill_disclosure] Add tests proving explicit skill load includes one selected skill body and excludes unrelated skill bodies. r[agent_harness_runtime.pi_resources.skill_disclosure.command]
- [x] [serial] Add deterministic resource command lookup and collision diagnostics. r[agent_harness_runtime.pi_resources.commands]
- [x] [depends:commands] Add duplicate-name, invalid-name, unsupported-kind, and unsafe-path diagnostic tests. r[agent_harness_runtime.pi_resources.commands.duplicate]
- [x] [depends:descriptors.validate,host_discovery.no_scan,project_trust.denied,skill_disclosure.command,commands.duplicate] Update docs and run focused resource tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.pi_resources.descriptors]
