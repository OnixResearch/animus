# Sequenced Session Event Replay Delta

## Requirements

### Requirement: Session event replay has one authoritative boundary

r[agent_harness_runtime.session_event_replay.boundary] Animus MUST derive reconnect replay from the existing admitted session ledger and runtime event order. It MUST NOT create a second transcript, tool-correlation ledger, model-context store, or persistence authority.

#### Scenario: Replay uses the admitted ledger

- GIVEN a durable session has an admitted ledger generation and ordered runtime facts
- WHEN a client requests event replay
- THEN Animus derives the replay plan from that ledger and shared event order

#### Scenario: A detached event store is rejected

- GIVEN an adapter supplies events that cannot be bound to the admitted ledger generation
- WHEN replay admission runs
- THEN Animus denies the detached event source

### Requirement: Session replay identities are nominal and complete

r[agent_harness_runtime.session_event_replay.identity] Animus MUST use distinct types for session identity, stream identity, ledger generation, event sequence, cursor identity, policy identity, and replay receipt identity.

#### Scenario: A complete cursor is accepted

- GIVEN a cursor binds the current session, stream, generation, sequence, branch or active leaf, policy, and redaction profile
- WHEN Animus validates the cursor
- THEN it can plan a bounded top-up, no-op, rebuild, or denial

#### Scenario: A stale generation is not treated as a numeric offset

- GIVEN a cursor sequence exists but its ledger generation is stale
- WHEN Animus validates the cursor
- THEN it denies silent continuation and selects only an explicit rebuild or terminal outcome

### Requirement: Event durability classes are explicit

r[agent_harness_runtime.session_event_replay.policy] Animus MUST classify session events as durable semantic, live-only delta, or metadata-only observation under a versioned policy. Live-only deltas MUST NOT enter reconnect replay as completed semantic events.

#### Scenario: Completed semantic events replay

- GIVEN a completed semantic event is committed to the admitted ledger
- WHEN replay selects its sequence
- THEN the event can appear once in the bounded replay result

#### Scenario: Streaming deltas remain live-only

- GIVEN partial model text or reasoning deltas were observed but not committed as a completed semantic event
- WHEN a reconnect replay is planned
- THEN those deltas are omitted and cannot establish completed content

### Requirement: Replay planning is deterministic and bounded

r[agent_harness_runtime.session_event_replay.planning] The pure replay planner MUST support initial replay, `sequence > offset` top-up, exact no-op, bounded rebuild, gap, duplicate, stale-generation, truncation, and terminal outcomes. Policy MUST bound event count and total projected bytes.

#### Scenario: Offset top-up returns later events

- GIVEN a current cursor points to an admitted sequence
- WHEN later replayable events exist in the same stream generation
- THEN the planner selects only events with greater sequences in canonical order

#### Scenario: A gap fails closed

- GIVEN the selected range contains a missing or duplicated sequence
- WHEN the planner validates the range
- THEN it returns a gap or duplicate outcome without silently renumbering events

### Requirement: All client projections preserve one event order

r[agent_harness_runtime.session_event_replay.projection] ACP, headless, TUI, and future transport adapters MUST project the same admitted event order and durability classes. Transport adapters MUST NOT widen payload visibility or replay authority.

#### Scenario: Projections agree

- GIVEN one admitted replay plan
- WHEN ACP, headless, and TUI fixtures project it
- THEN their semantic event identities and order agree within each supported surface

### Requirement: Replay evidence is metadata-only

r[agent_harness_runtime.session_event_replay.evidence] Replay receipts MUST bind BLAKE3 identities, ranges, counts, classes, omissions, rebuild status, terminal status, and non-claims. They MUST reject transcript bodies, reasoning, tool arguments, tool results, credentials, environment values, and physical paths.

#### Scenario: Safe receipt passes

- GIVEN a replay result contains only admitted metadata
- WHEN receipt validation runs
- THEN it emits a stable metadata-only receipt

### Requirement: Replay includes positive and negative verification

r[agent_harness_runtime.session_event_replay.verification] The focused replay rail MUST include positive reconnect behavior and negative duplicate, gap, stale-generation, branch-drift, live-delta, bound, and redaction cases.

#### Scenario: Focused rail passes

- GIVEN all required fixtures and projection checks are present
- WHEN the focused rail runs
- THEN it reports the supported replay boundary and non-claims
