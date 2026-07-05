# Tasks

- [x] [serial] Define scoped worker task contracts with objective, scope, context packet, allowed capabilities, budgets, output schema, parent trace, and authority metadata. r[agent_harness_runtime.claude_code_agents.task_contract]
- [x] [depends:task_contract] Add negative worker task tests for missing scope, capabilities, budgets, output schema, parent trace, and overbroad authority. r[agent_harness_runtime.claude_code_agents.task_contract.overbroad]
- [x] [serial] Implement context packet DTOs and pure validation for selected parent messages, summaries, file observations, memory descriptors, and tool results. r[agent_harness_runtime.claude_code_agents.context_packet]
- [x] [depends:context_packet] Add leak-rejection tests for full transcripts, raw hidden memory, credentials, provider payloads, and unbounded tool outputs. r[agent_harness_runtime.claude_code_agents.context_packet.leak]
- [x] [serial] Add host-injected worker service contracts for enqueue, status, cancel, and collect with typed failure classes. r[agent_harness_runtime.claude_code_agents.worker_service]
- [x] [depends:worker_service] Add missing-service tests proving no fallback thread, process, shell, model, or network request is attempted. r[agent_harness_runtime.claude_code_agents.worker_service.missing]
- [x] [serial] Add pure coordinator merge logic for validated worker reports, evidence references, conflicts, and mutation intents. r[agent_harness_runtime.claude_code_agents.report_merge]
- [x] [depends:report_merge] Add conflict tests proving incompatible reports or mutation intents require arbitration before finalization. r[agent_harness_runtime.claude_code_agents.report_merge.conflict]
- [x] [serial] Document Claude Code agent teams as a bounded reference and prove workers cannot bypass parent policy. r[agent_harness_runtime.claude_code_agents.scope]
- [x] [depends:scope.no_bypass,task_contract.overbroad,context_packet.leak,worker_service.missing,report_merge.conflict] Run focused worker orchestration tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_agents.scope]
