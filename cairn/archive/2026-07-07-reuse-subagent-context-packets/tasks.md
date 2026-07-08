# Tasks

- [x] [serial] Add shared subagent context packet DTOs and validation helpers for safe context item classes. r[agent_harness_runtime.subagent_context_packets.shared_packet]
- [x] [depends:shared_packet] Require explicit admission and leak validation for unsafe context item classes. r[agent_harness_runtime.subagent_context_packets.unsafe_admission]
- [x] [depends:shared_packet] Add deterministic BLAKE3 packet identity and budget summary fields for receipts and ledger projection. r[agent_harness_runtime.subagent_context_packets.identity]
- [x] [depends:shared_packet,identity] Add projection shims for recipe-backed, fresh-context, side-question, and worker-team validators. r[agent_harness_runtime.subagent_context_packets.kind_projection]
- [x] [depends:unsafe_admission,kind_projection] Add positive minimized-context tests and negative leak, over-budget, and unsafe-raw tests for every child-run kind. r[agent_harness_runtime.subagent_context_packets.fail_closed]
- [x] [depends:fail_closed] Update docs and focused context-packet verification evidence. r[agent_harness_runtime.subagent_context_packets.verification]
