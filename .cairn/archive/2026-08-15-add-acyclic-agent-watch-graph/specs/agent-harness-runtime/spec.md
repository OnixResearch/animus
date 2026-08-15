# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Session-local typed watch graph

r[agent_harness_runtime.agent_watch.graph] Animus MUST provide a session-local typed watch-graph reducer that owns watch edges, status updates, and enable and disable transitions without process, socket, persistence, or provider authority.

#### Scenario: Enables and disables reduce deterministically

- GIVEN a session-local watch state and a valid enable or disable transition
- WHEN the reducer applies the transition
- THEN the reducer returns the next deterministic watch state
- AND no message, transport, or persistence side effect occurs.

### Requirement: Cycle-closing enables are rejected atomically

r[agent_harness_runtime.agent_watch.acyclic] Animus MUST reject any watch enable that would close a cycle in the watch graph without changing watch state.

#### Scenario: Rejected enable leaves state unchanged

- GIVEN a watch graph in which a proposed enable would close a cycle
- WHEN the reducer applies that enable
- THEN the reducer rejects the enable with a typed cycle diagnostic
- AND the watch state before the enable remains unchanged.

### Requirement: Watcher-only redacted status

r[agent_harness_runtime.agent_watch.status] Watchers MUST receive only self-reported semantic status and sanitized provider retry progress, never an agent's hidden prompts or raw provider data.

#### Scenario: Raw provider data stays out of the projection

- GIVEN a watcher observes a delegated agent during provider retry
- WHEN the status projection runs
- THEN the watcher sees a bounded sanitized retry status
- AND no raw provider payload or hidden prompt crosses the projection.

### Requirement: Watch validation fails closed

r[agent_harness_runtime.agent_watch.fail_closed] Watch validation MUST reject unknown agents, duplicate edges, stale watchers, non-monotonic generation, and malformed status records with bounded safe error classes before watch state changes.

#### Scenario: Unknown agent is rejected

- GIVEN a watch transition names an agent outside the session
- WHEN watch validation runs
- THEN validation rejects the transition with a typed diagnostic
- AND no watch edge is created for the unknown agent.

### Requirement: Metadata-only watch receipts

r[agent_harness_runtime.agent_watch.receipts] Watch receipts MUST include graph topology identity, edge identities, status labels, and rejection classes, and MUST NOT include prompts, transcripts, or provider payloads.

#### Scenario: Receipt rejects transcript body

- GIVEN a watch receipt fixture includes a transcript or prompt body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Agent-watch verification evidence

r[agent_harness_runtime.agent_watch.verification] The adaptation MUST include focused positive and negative tests for graph transitions, acyclic enforcement, watcher-only redaction, fail-closed validation, receipt leak rejection, and feature isolation.

#### Scenario: Closeout evidence covers watch fixtures

- GIVEN maintainers intend to archive the watch-graph adaptation
- WHEN closeout validation runs
- THEN evidence names focused subagent, host-control, workspace, receipt, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
