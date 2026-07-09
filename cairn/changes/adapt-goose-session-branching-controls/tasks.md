# Tasks

- [ ] [serial] Define session branch request DTOs for edit-in-place, fork-from-message, duplicate, label, continuation target, cut-point identity, copy-settings policy, and provenance. r[agent_harness_runtime.goose_session_branching.requests]
- [ ] [depends:requests] Add pure branch planning over in-memory ledger snapshots, active heads, cut points, copy policies, policy admission, and checkpoint evidence. r[agent_harness_runtime.goose_session_branching.pure_planner]
- [ ] [depends:pure_planner] Model edit-in-place suffix discard with explicit policy, stale-cutpoint, suffix-span, and checkpoint diagnostics. r[agent_harness_runtime.goose_session_branching.edit_in_place]
- [ ] [depends:pure_planner] Add duplicate and fork semantics that copy safe descriptors only and reject live-process, credential, provider-session, or raw transcript inheritance. r[agent_harness_runtime.goose_session_branching.copy_boundaries]
- [ ] [depends:edit_in_place,copy_boundaries] Add safe branch receipts and leak checks for labels, prompts, transcripts, provider payloads, tool outputs, credentials, and full paths. r[agent_harness_runtime.goose_session_branching.receipts]
- [ ] [depends:receipts] Update docs and run focused branch-planning tests, stale/unsafe negative fixtures, receipt leak tests, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.goose_session_branching.verification]
