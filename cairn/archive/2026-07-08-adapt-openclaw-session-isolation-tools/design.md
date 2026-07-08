# Design: Adapt OpenClaw session isolation and session-tool contracts

The adaptation should make session identity and cross-session access explicit before any persistence or delivery behavior exists. The core functions are pure planners over host-supplied metadata; stores, transcript files, clocks, and delivery remain host-owned.

## Decisions

### 1. Session keys are planned, not discovered

**Choice:** Define session key derivation as pure functions over source kind, agent identity, channel/account/peer metadata, group/thread identifiers, identity links, and selected direct-message scope.

**Rationale:** This captures OpenClaw's privacy lesson without adopting its filesystem layout or global store.

### 2. Direct-message scope defaults are visible policy

**Choice:** The planner emits diagnostics when a multi-sender setup uses a shared main direct-message scope with tool-enabled authority.

**Rationale:** Shared context is acceptable for single-user continuity, but dangerous for shared inboxes.

### 3. Session tools are visibility-scoped

**Choice:** Session-list/history/send/spawn planning uses self, tree, agent, or all visibility and clamps sandboxed sessions to tree or stricter host policy.

**Rationale:** Cross-session access is useful for orchestration but should not silently bypass sandbox or delegation boundaries.

### 4. Inter-session influence needs provenance

**Choice:** Inter-session messages and spawned child runs carry source session, source tool, source channel, child lineage, and bounded handoff/announce metadata.

**Rationale:** Transcript readers and replay validators need to distinguish external user input from routed agent instructions.

### 5. Maintenance is a plan

**Choice:** Session cleanup and bounds are represented as deterministic maintenance plans over loaded metadata, not as filesystem mutation.

**Rationale:** Hosts can implement JSONL, database, or in-memory stores while reusable code verifies age/count/byte policies and receipts.
