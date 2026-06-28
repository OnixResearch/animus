# Tasks

- [ ] [serial] Add an architecture note evaluating Clankers tool-catalog and capability-pack patterns for Animus, including adopt/adapt/reject decisions. r[agent_harness_runtime.clankers_tool_catalog.scope]
- [ ] [depends:clankers_tool_catalog.scope] Define an optional Animus-owned catalog descriptor schema for names, descriptions, runtime kind, capability classes, side-effect levels, approval policy, input/output schema references, and redaction policy. r[agent_harness_runtime.clankers_tool_catalog.schema]
- [ ] [depends:clankers_tool_catalog.schema] Add validation for duplicate names, built-in collisions, unknown runtime kinds, invalid schemas, dangerous capability without approval, and secret-adjacent capability without redaction. r[agent_harness_runtime.clankers_tool_catalog.validation]
- [ ] [depends:clankers_tool_catalog.validation] Add runtime-neutrality checks proving catalog parsing does not execute tools, start plugins/providers, read secrets, inherit environment, or open network connections. r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral]
- [ ] [depends:clankers_tool_catalog.validation] Add positive and negative fixtures for safe descriptors and invalid/dangerous descriptors. r[agent_harness_runtime.clankers_tool_catalog.fixtures]
- [ ] [depends:clankers_tool_catalog.runtime_neutral,clankers_tool_catalog.fixtures] Run focused manifest rails, dependency checks, workspace tests if touched, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_tool_catalog.closeout]
