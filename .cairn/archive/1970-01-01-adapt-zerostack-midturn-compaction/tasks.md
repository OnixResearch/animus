# Tasks

- [x] [serial] Define prompt pressure inputs, provider route metadata, and cached-token interpretation rules. r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure]
- [x] [depends:prompt_pressure] Add cache accounting tests for included, separate, absent, and malformed cached-token usage fields. r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure.cache]
- [x] [serial] Define mid-turn compaction policy and continuation plan DTOs. r[agent_harness_runtime.zerostack_midturn_compaction.plan]
- [x] [depends:plan] Add enabled, disabled, threshold-boundary, and fail-policy tests. r[agent_harness_runtime.zerostack_midturn_compaction.plan.disabled]
- [x] [serial] Add explicit ledger summary entry planning for mid-turn compaction. r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries]
- [x] [depends:ledger_entries] Add negative missing-ledger-service tests proving no model/tool fallback occurs. r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries.missing]
- [x] [serial] Define injected summarizer validation for bounded text, safe metadata, and summary kind. r[agent_harness_runtime.zerostack_midturn_compaction.summarizer]
- [x] [depends:summarizer] Add malformed, empty, overlong, and unsafe-metadata summary tests. r[agent_harness_runtime.zerostack_midturn_compaction.summarizer.malformed]
- [x] [serial] Define long tool-output artifact descriptors with bounded visible windows and BLAKE3 artifact hashes. r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts]
- [x] [depends:tool_artifacts] Add tests proving unbounded tool output is excluded from model-visible context and durable evidence by default. r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts.bounded]
- [x] [depends:prompt_pressure,plan,ledger_entries,summarizer,tool_artifacts] Update docs and run focused compaction tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_midturn_compaction.plan]
