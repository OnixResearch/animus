# Tasks

- [x] [serial] Define typed agent-definition manifest DTOs for name, description, model preference, prompt fragments, capability ceilings, toolset limits, source scope, trust status, and manifest identity. r[agent_harness_runtime.clankers_agent_definitions.manifest]
- [x] [depends:manifest] Add valid user/explicit manifest tests proving raw parsing does not grant tool, model, memory, or filesystem authority. r[agent_harness_runtime.clankers_agent_definitions.manifest.valid]
- [x] [serial] Implement deterministic discovery over loaded user, project, and explicit host-provided sources with same-precedence duplicate rejection. r[agent_harness_runtime.clankers_agent_definitions.discovery]
- [x] [depends:discovery] Add duplicate project/user/explicit definition tests proving no arbitrary source ordering is used. r[agent_harness_runtime.clankers_agent_definitions.discovery.duplicate]
- [x] [serial] Add project/workspace trust admission model requiring host confirmation or policy admission before influence. r[agent_harness_runtime.clankers_agent_definitions.project_trust]
- [x] [depends:project_trust] Add unadmitted project-agent tests proving prompt, tool, model, memory, and workflow permissions are unchanged. r[agent_harness_runtime.clankers_agent_definitions.project_trust.denied]
- [x] [serial] Enforce manifest toolsets, model preferences, memories, context references, and workflow abilities as ceilings or requests below active capability policy. r[agent_harness_runtime.clankers_agent_definitions.capability_ceilings]
- [x] [depends:capability_ceilings] Add escalation tests for unauthorized tools, model roles, network references, and workflow abilities. r[agent_harness_runtime.clankers_agent_definitions.capability_ceilings.escalation]
- [x] [serial] Define safe discovery and activation receipts with definition/source/trust/capability/model/hash/policy/redaction metadata. r[agent_harness_runtime.clankers_agent_definitions.receipts]
- [x] [depends:receipts] Add leak fixtures for raw prompt text, full paths, credentials, provider JSON, and hidden memory bodies. r[agent_harness_runtime.clankers_agent_definitions.receipts.rejects_leaks]
- [x] [depends:manifest.valid,discovery.duplicate,project_trust.denied,capability_ceilings.escalation,receipts.rejects_leaks] Update docs and run focused agent-definition tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_agent_definitions.verification]
