## Phase 1: Evidence binding

- [x] [serial] Pin the reviewed Riveter revision, record its license and selected rejection concepts, and add the bounded reference to `README.md`. r[agent_harness_runtime.rejected_attempt.boundary]
- [x] [serial] Define nominal task, run, attempt, VCS revision, workspace snapshot, Cairn handoff, summary, and receipt identities. r[agent_harness_runtime.rejected_attempt.model]
- [x] [serial] Add typed Nickel policy for accepted review evidence, freshness, redaction, summary and diff limits, VCS providers, and non-claims. r[agent_harness_runtime.rejected_attempt.policy]
- [x] [depends:agent_harness_runtime.rejected_attempt.model] Implement pure admission and context projection over supplied Cairn and VCS facts. r[agent_harness_runtime.rejected_attempt.planning]

## Phase 2: Host adapters and verification

- [x] [depends:agent_harness_runtime.rejected_attempt.planning] Add implementation-free review and VCS services plus an optional bounded `jj` host fixture. r[agent_harness_runtime.rejected_attempt.host]
- [x] [parallel] Add positive fixtures for metadata-only handoff, bounded summary context, and a current `jj` attempt. r[agent_harness_runtime.rejected_attempt.verification]
- [x] [parallel] Add negative fixtures for missing or malformed handoffs, stale revisions, wrong tasks, ambiguous attempts, unsafe or oversized diffs, and retry or completion overclaims. r[agent_harness_runtime.rejected_attempt.verification]
- [x] [serial] Compose with fresh-context and Ralph profiles, add metadata-only receipts, run focused checks and Cairn gates, and document non-claims. r[agent_harness_runtime.rejected_attempt.evidence] r[agent_harness_runtime.rejected_attempt.verification]
