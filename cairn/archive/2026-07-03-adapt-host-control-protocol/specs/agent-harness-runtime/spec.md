# agent-harness-runtime Change Spec: Host control protocol adaptation

## Requirements

### Requirement: UI-neutral host topology DTOs

r[agent_harness_runtime.host_control.topology_dtos] Animus MUST define UI-neutral host-control DTOs for workspaces, tabs, panes, agents, labels, cwd or provenance summaries, focus state, rename and close operations, revision metadata, and selected semantic status fields without embedding TUI, PTY, socket, or terminal rendering types.

#### Scenario: Topology command validates selectors

r[agent_harness_runtime.host_control.topology_dtos.validation]
- GIVEN a host-control command targets a workspace, tab, pane, or agent selector
- WHEN command validation runs
- THEN valid selectors and supported operations are admitted as typed host commands
- AND missing selectors, stale selectors, cross-host selectors, duplicate labels, and unsupported operations are rejected with typed diagnostics.

### Requirement: Selector lifetime is explicit

r[agent_harness_runtime.host_control.selector_lifetime] Host-control selectors MUST distinguish live-session selectors from host-declared durable identities, and callers MUST NOT treat compact live topology IDs as durable across topology changes unless the host marks them durable.

#### Scenario: Compact live selector is not durable

r[agent_harness_runtime.host_control.selector_lifetime.not_durable]
- GIVEN a pane selector was returned before a tab or pane topology change
- WHEN a caller tries to reuse that selector after the host reports a revision change
- THEN validation requires re-resolution or matching revision evidence
- AND the selector is not accepted as a durable identity by default.

### Requirement: Bounded host output reads

r[agent_harness_runtime.host_control.bounded_reads] Host output reads MUST return bounded view DTOs with source class, line and byte counts, truncation status, redaction status, safe references, and optional BLAKE3 content hashes rather than unbounded transcripts.

#### Scenario: Large output is truncated explicitly

r[agent_harness_runtime.host_control.bounded_reads.truncated]
- GIVEN a host pane has more output than the requested bounded view allows
- WHEN a caller requests recent output
- THEN the response contains only the bounded view
- AND line count, byte count, truncation status, redaction status, and optional safe hash metadata make the omission explicit.

### Requirement: Declarative layout contract

r[agent_harness_runtime.host_control.layout_contract] Animus SHOULD define declarative layout export and apply DTOs for pane and split trees, labels, cwd summaries, optional command descriptors, focus, and zoom state, while explicitly separating layout reconstruction from live process preservation.

#### Scenario: Layout apply does not claim live process preservation

r[agent_harness_runtime.host_control.layout_contract.apply]
- GIVEN a host applies a declarative layout tree to create or replace a tab
- WHEN the layout apply response is produced
- THEN it reports created topology and focus state
- AND it does not claim that previous live PTYs, scrollback, or running processes were preserved unless a separate host capability provides that evidence.

### Requirement: Event subscriptions and waits share semantics

r[agent_harness_runtime.host_control.events_waits] Event subscriptions and one-shot waits MUST use the same typed event and semantic status contracts for agent status changes, output matches, pane lifecycle, cancellation, timeout, and host errors.

#### Scenario: Wait timeout is typed

r[agent_harness_runtime.host_control.events_waits.timeout]
- GIVEN a caller waits for an agent to reach `done`
- AND the semantic state does not reach `done` before the requested timeout
- WHEN the wait completes
- THEN the response is a typed timeout outcome
- AND it does not fabricate a terminal command completion or final agent result.

### Requirement: Transport-neutral command envelopes

r[agent_harness_runtime.host_control.transport_neutral] Host-control commands, responses, errors, and events MUST be transport-neutral DTOs with correlation IDs, typed errors, feature negotiation, and unknown-field handling; sockets, named pipes, CLI wrappers, PTYs, and terminal rendering MUST remain host adapter concerns.

#### Scenario: Command validation needs no IPC server

r[agent_harness_runtime.host_control.transport_neutral.no_ipc_dependency]
- GIVEN a command envelope and host capability snapshot are available in memory
- WHEN reusable validation logic checks the command
- THEN validation succeeds or fails deterministically over those inputs
- AND no socket, named pipe, PTY, terminal renderer, or CLI wrapper is required.

### Requirement: Host control verification

r[agent_harness_runtime.host_control.verification] The adaptation MUST include focused positive and negative tests for topology DTOs, selector lifetime, bounded reads, layout export/apply, event subscriptions, waits, typed errors, feature negotiation, unknown fields, and transport-neutral validation.

#### Scenario: Closeout evidence covers host-control rails

r[agent_harness_runtime.host_control.verification.closeout]
- GIVEN maintainers intend to archive the Host control protocol adaptation
- WHEN closeout validation runs
- THEN evidence names focused topology, selector, bounded-read, layout, event/wait, transport-neutral, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.
