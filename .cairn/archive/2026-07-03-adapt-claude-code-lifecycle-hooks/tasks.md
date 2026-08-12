# Tasks

- [x] [serial] Define bounded Claude Code hook adaptation scope and exclude arbitrary shell hook authority from reusable runtime crates. r[agent_harness_runtime.claude_code_hooks.scope]
- [x] [depends:scope] Add tests proving hook-requested broader actions cannot bypass DTO, permission, and capability checks. r[agent_harness_runtime.claude_code_hooks.scope.no_bypass]
- [x] [serial] Define stable hook-point vocabulary for session, prompt, model, tool, mutation, validation, receipt, and workflow lifecycle events. r[agent_harness_runtime.claude_code_hooks.points]
- [x] [depends:points] Add negative hook registry tests for unknown hook points and malformed descriptors. r[agent_harness_runtime.claude_code_hooks.points.unknown]
- [x] [serial] Define safe hook event payload DTOs with IDs, classes, redacted labels, sizes, hashes, status, and bounded summaries. r[agent_harness_runtime.claude_code_hooks.payloads]
- [x] [depends:payloads] Add leak-rejection tests proving raw prompts, credentials, provider payloads, raw tool bodies, full paths, and unbounded outputs are absent. r[agent_harness_runtime.claude_code_hooks.payloads.bounded]
- [x] [serial] Add host-owned hook execution adapter contracts with required capability declarations and unavailable-service failures. r[agent_harness_runtime.claude_code_hooks.execution]
- [x] [depends:execution] Add negative tests proving required hooks fail safely when the hook service is missing or denied. r[agent_harness_runtime.claude_code_hooks.execution.missing_service]
- [x] [serial] Add hook outcome handling for observe, annotate, deny, and request-revalidation paths. r[agent_harness_runtime.claude_code_hooks.outcomes]
- [x] [depends:outcomes] Add malformed-modification tests proving hook changes are revalidated and rejected before use. r[agent_harness_runtime.claude_code_hooks.outcomes.invalid_modification]
- [x] [depends:scope.no_bypass,points.unknown,payloads.bounded,execution.missing_service,outcomes.invalid_modification] Run focused hook tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_hooks.scope]
