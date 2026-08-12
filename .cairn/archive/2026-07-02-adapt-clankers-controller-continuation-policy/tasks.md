# Tasks

- [x] [serial] Define typed continuation state for queued prompts, active loop work, validation follow-ups, busy prompt state, cancellation, pending dispatch, and completion acknowledgements. r[agent_harness_runtime.clankers_continuation_policy.state]
- [x] [depends:state] Add queued-prompt state tests proving no shell-local hidden flag is needed for next-action decisions. r[agent_harness_runtime.clankers_continuation_policy.state.queued_prompt]
- [x] [serial] Implement deterministic continuation priority among queued prompt replay, loop continuation, validation/auto-test follow-up, and completion. r[agent_harness_runtime.clankers_continuation_policy.priority]
- [x] [depends:priority] Add priority tests proving queued user prompts win over automation and leave automation pending. r[agent_harness_runtime.clankers_continuation_policy.priority.queued_wins]
- [x] [serial] Add exact pending-work ID validation for dispatch and completion acknowledgements. r[agent_harness_runtime.clankers_continuation_policy.acknowledgements]
- [x] [depends:acknowledgements] Add stale, duplicate, missing, and mismatched acknowledgement tests preserving current state. r[agent_harness_runtime.clankers_continuation_policy.acknowledgements.stale]
- [x] [serial] Enforce shell-owned boundaries for provider calls, timers, queue waits, persistence, UI events, and validation command execution. r[agent_harness_runtime.clankers_continuation_policy.shell_owned]
- [x] [depends:shell_owned] Add pure-core tests proving continuation decisions do not call providers, commands, storage, or UI. r[agent_harness_runtime.clankers_continuation_policy.shell_owned.no_provider]
- [x] [serial] Define safe continuation receipts with prior state class, selected action, pending work ID hash, reason, rejection class, policy hash, and BLAKE3 receipt hash. r[agent_harness_runtime.clankers_continuation_policy.receipts]
- [x] [depends:receipts] Add leak fixtures for raw prompts, model outputs, tool outputs, provider JSON, transcripts, and unbounded logs. r[agent_harness_runtime.clankers_continuation_policy.receipts.rejects_leaks]
- [x] [depends:state.queued_prompt,priority.queued_wins,acknowledgements.stale,shell_owned.no_provider,receipts.rejects_leaks] Update docs and run focused continuation tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_continuation_policy.verification]
