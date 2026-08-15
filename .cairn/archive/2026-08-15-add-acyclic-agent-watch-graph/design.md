## Context

The subagent facade owns delegation boundaries, host-control owns event and wait contracts, and continuation policy owns post-turn state. None of these owns a session-local watch graph. A watch graph is pure decision state: which agents watch which peers, and whether an enable closes a cycle.

## Decisions

### 1. Own the watch graph as a pure reducer

**Choice:** Add a session-local pure watch-graph reducer over typed watch edges. It accepts enables, disables, and status updates and returns the next graph state. No process, socket, persistence, or provider authority.

**Rationale:** Graph topology and cycle admission are deterministic decisions testable without host effects.

### 2. Reject cycle-closing enables atomically

**Choice:** An enable that points at an agent that can already reach the enabled agent closes a cycle and is rejected. Watch state is unchanged on rejection and the rejection is typed.

**Rationale:** Acyclic watch topology prevents feedback loops and ambiguous status propagation. The reject-without-mutation rule keeps state deterministic.

### 3. Watcher-only redacted status

**Choice:** Watchers receive self-reported semantic status and sanitized provider retry progress only. Hidden prompts and raw provider data never cross the watch projection.

**Rationale:** Status visibility must not leak conversation internals. Redaction is part of the projection contract.

### 4. Fail closed

**Choice:** Validation rejects unknown agents, duplicate edges, stale watchers, non-monotonic generation, and malformed status records before the graph changes.

**Rationale:** Deterministic admission keeps watch state safe under concurrent delegation.

### 5. Bounded receipts

**Choice:** Receipts are metadata-only with watch graph topology, edge identities, status labels, and rejection classes. No prompts, provider payloads, or transcripts.

**Rationale:** This matches the evidence culture for event-ledger surfaces.

## Risks / Trade-offs

- Watch graphs are session-local. Cross-session discovery stays outside this change.
- Self-reported status is not ground truth. The redacted projection makes the status untrusted by design.
- Acyclic enforcement rejects legitimate bidirectional collaboration unless one direction is modeled as a reverse ack.
