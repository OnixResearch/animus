## Phase 1: Typed replay contract

- [x] [serial] Pin the reviewed Sandbox Agent and agentOS revisions, record selected concepts and licenses, and add bounded references to `README.md`. r[agent_harness_runtime.session_event_replay.boundary]
- [x] [depends:type-acp-session-identities] Add nominal session stream, generation, sequence, cursor, event class, and replay outcome types. r[agent_harness_runtime.session_event_replay.identity]
- [x] [serial] Add typed Nickel policy for replayable classes, range and byte limits, redaction, rebuild policy, and terminal outcomes. r[agent_harness_runtime.session_event_replay.policy]
- [x] [depends:agent_harness_runtime.session_event_replay.identity] Implement the pure replay planner over already-loaded ledger and runtime event facts. r[agent_harness_runtime.session_event_replay.planning]

## Phase 2: Projections and evidence

- [x] [depends:agent_harness_runtime.session_event_replay.planning] Project one admitted order to ACP, headless, and TUI adapters without creating another ledger. r[agent_harness_runtime.session_event_replay.projection]
- [x] [parallel] Add positive fixtures for initial replay, exact top-up, no-op, terminal replay, and reconnect after completed semantic events. r[agent_harness_runtime.session_event_replay.verification]
- [x] [parallel] Add negative fixtures for duplicate sequences, gaps, stale generations, branch drift, live-only delta replay, oversized ranges, unsafe payloads, and ambiguous rebuilds. r[agent_harness_runtime.session_event_replay.verification]
- [x] [serial] Add metadata-only receipts, run the focused session, ACP, projection, Cairn, and Nix rails, and document bounded non-claims. r[agent_harness_runtime.session_event_replay.evidence] r[agent_harness_runtime.session_event_replay.verification]
