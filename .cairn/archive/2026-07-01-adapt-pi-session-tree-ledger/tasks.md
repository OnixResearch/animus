# Tasks

- [x] [serial] Define neutral ledger entry, entry ID, parent link, active leaf, label, summary, replay metadata, and typed ledger error DTOs. r[agent_harness_runtime.pi_session_tree.ledger_contract]
- [x] [depends:ledger_contract] Add pure validation for duplicate IDs, missing parents, cycles, invalid active leaves, unsupported entry kinds, and orphaned tool-result shapes. r[agent_harness_runtime.pi_session_tree.ledger_contract.validate]
- [x] [depends:ledger_contract] Add pure active-leaf context replay that walks root-to-leaf and preserves supported user, assistant/model, tool-result, and summary semantics. r[agent_harness_runtime.pi_session_tree.replay_context]
- [x] [depends:replay_context] Add positive replay tests for linear history, two-branch history, tool-result ordering, and sibling branch exclusion. r[agent_harness_runtime.pi_session_tree.replay_context.branch]
- [x] [depends:replay_context] Add compaction-summary and branch-summary replay behavior with bounded summary text and safe metadata. r[agent_harness_runtime.pi_session_tree.summary_entries]
- [x] [depends:summary_entries] Add negative replay tests proving summarized raw spans are not emitted and malformed summary boundaries fail closed. r[agent_harness_runtime.pi_session_tree.summary_entries.replay]
- [x] [depends:ledger_contract] Add label attach/read behavior that is metadata-only by default. r[agent_harness_runtime.pi_session_tree.labels]
- [x] [depends:labels] Add tests proving labels do not enter model-visible context without explicit host injection. r[agent_harness_runtime.pi_session_tree.labels.replay_neutral]
- [x] [depends:ledger_contract] Add runtime missing-ledger and missing-session checks before Steel/model/tool execution. r[agent_harness_runtime.pi_session_tree.missing_service]
- [x] [depends:missing_service] Add negative runtime tests for resume requests without ledger service and with missing session IDs. r[agent_harness_runtime.pi_session_tree.missing_service.halts]
- [x] [depends:missing_service] Add Preserves schema/round-trip tests or documented DTO serialization fixtures once field names settle. r[agent_harness_runtime.pi_session_tree.ledger_contract]
- [x] [depends:ledger_contract.validate,replay_context.branch,summary_entries.replay,labels.replay_neutral,missing_service.halts] Update docs and run focused ledger tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.pi_session_tree.ledger_contract]
