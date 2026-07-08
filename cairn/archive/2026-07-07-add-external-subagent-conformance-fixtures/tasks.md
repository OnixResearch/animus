# Tasks

- [x] [serial] Add positive ACP and MCP external subagent descriptor fixtures with injected service labels and declared tools. r[agent_harness_runtime.subagent_external_conformance.positive_fixtures]
- [x] [serial] Add negative fixtures for missing service, undeclared tool, ambient CLI discovery, inherited credentials, malformed protocol, unsafe output import, and overbroad capability claims. r[agent_harness_runtime.subagent_external_conformance.negative_fixtures]
- [x] [depends:positive_fixtures,negative_fixtures] Add a conformance index or focused check that records expected status, failure class, and requirement coverage. r[agent_harness_runtime.subagent_external_conformance.index]
- [x] [depends:index] Add tests proving fixtures validate without live ACP/MCP services, network access, ambient CLI lookup, or credentials. r[agent_harness_runtime.subagent_external_conformance.no_live_dependency]
- [x] [depends:index,no_live_dependency] Update docs and focused external-subagent conformance verification evidence. r[agent_harness_runtime.subagent_external_conformance.verification]
