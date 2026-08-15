## Phase 1: Watch-graph contract and pure core

- [x] [serial] Add the session-local typed watch-graph reducer with typed watch edges. r[agent_harness_runtime.agent_watch.graph]
- [x] [serial] Add atomic cycle-closing-enable rejection that leaves watch state unchanged. r[agent_harness_runtime.agent_watch.acyclic]
- [x] [serial] Add watcher-only status projection with self-reported semantic status and sanitized provider retry progress. r[agent_harness_runtime.agent_watch.status]

## Phase 2: Fail-closed validation and receipts

- [x] [serial] Add fail-closed validation for unknown agents, duplicate edges, stale watchers, non-monotonic generation, and malformed status records. r[agent_harness_runtime.agent_watch.fail_closed]
- [x] [depends:graph] Add metadata-only receipts with topology, edge identities, status labels, and rejection classes. r[agent_harness_runtime.agent_watch.receipts]

## Phase 3: Evidence

- [x] [parallel] Add positive fixtures for valid enable and disable sequences, acyclic multi-hop graphs, and sanitized status projection. r[agent_harness_runtime.agent_watch.verification]
- [x] [parallel] Add negative fixtures for cycle-closing enables, unknown agents, duplicate edges, stale watchers, malformed status, and receipt leak rejection. r[agent_harness_runtime.agent_watch.fail_closed] r[agent_harness_runtime.agent_watch.verification]
- [x] [serial] Verify default builds remain independent of the watch family. r[agent_harness_runtime.agent_watch.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document watch semantics, acyclicity, watcher-only redaction, and non-claims. r[agent_harness_runtime.agent_watch.status] r[agent_harness_runtime.agent_watch.acyclic]
- [x] [serial] Run focused subagent, host-control, workspace, formatting, clippy, Cairn validation, and Cairn gate rails, then sync and archive only after all deterministic gates pass. r[agent_harness_runtime.agent_watch.verification]
