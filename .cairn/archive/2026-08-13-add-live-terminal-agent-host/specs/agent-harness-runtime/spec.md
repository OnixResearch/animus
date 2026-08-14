# Agent Harness Runtime

## ADDED Requirements

### Requirement: Live terminal ownership stays in a non-default client

r[agent_harness_runtime.live_terminal_agent_host.terminal] Animus MUST provide raw mode, alternate screen, cursor, backend, input, resize, and restoration only through a non-default client that owns no provider, tool, Steel, session, persistence, or worker execution.

#### Scenario: The terminal client starts

r[agent_harness_runtime.live_terminal_agent_host.terminal.start]
- GIVEN explicit terminal and connection configuration pass admission
- WHEN the client enters live mode
- THEN a lifecycle guard MUST record each successful terminal-state transition before rendering
- AND reusable, default, worker, and session crates MUST gain no terminal authority.

### Requirement: The terminal event loop is bounded and explicit

r[agent_harness_runtime.live_terminal_agent_host.event_loop] The client MUST orchestrate bounded input, resize, tick, session-event, render, cancellation, disconnect, and shutdown observations through explicit state transitions and named limits.

#### Scenario: Session events arrive continuously

r[agent_harness_runtime.live_terminal_agent_host.event_loop.fair]
- GIVEN the session event source has more events than one admitted batch
- WHEN one event-loop iteration runs
- THEN it MUST process no more than the named batch limit and retain input and cancellation progress
- AND it MUST not allocate or render an unbounded event backlog.

### Requirement: Connection mode is explicit

r[agent_harness_runtime.live_terminal_agent_host.connection] The client MUST connect through either one explicitly configured local daemon endpoint or one explicitly injected direct local service, and it MUST NOT discover or automatically switch modes.

#### Scenario: The configured daemon is unavailable

r[agent_harness_runtime.live_terminal_agent_host.connection.unavailable]
- GIVEN daemon mode names an admitted endpoint that cannot be reached
- WHEN the client connects
- THEN it MUST show or return a typed unavailable state
- AND it MUST NOT auto-start a daemon or switch to direct execution.

### Requirement: UI controls emit command descriptors only

r[agent_harness_runtime.live_terminal_agent_host.commands] Prompt, queue, approval, cancellation, child, and session controls MUST produce typed client command descriptors that pass through existing admission and ownership boundaries before effects.

#### Scenario: A queue edit is stale

r[agent_harness_runtime.live_terminal_agent_host.commands.queue_conflict]
- GIVEN the client submits a queue edit with stale compare-and-swap preconditions
- WHEN the session returns a conflict
- THEN the client MUST reduce the safe conflict into visible state and request a fresh private snapshot only through the authorized API
- AND the key handler or widget MUST not mutate the queue directly.

### Requirement: Terminal cleanup is observed on every owned exit path

r[agent_harness_runtime.live_terminal_agent_host.cleanup] The terminal guard MUST attempt idempotent restoration after normal exit, initialization failure, backend error, cancellation, disconnect, and supported signal handling, and cleanup failure MUST remain explicit.

#### Scenario: Backend construction fails after raw mode

r[agent_harness_runtime.live_terminal_agent_host.cleanup.partial_start]
- GIVEN raw mode succeeded and later terminal initialization fails
- WHEN the start operation unwinds
- THEN the guard MUST attempt to restore every state transition that completed
- AND the client MUST not emit a successful live-terminal receipt without a complete cleanup observation.

### Requirement: Live terminal evidence is layered

r[agent_harness_runtime.live_terminal_agent_host.verification] The client MUST retain pure reducer and TestBackend tests, add positive and negative pseudo-terminal lifecycle tests, reject direct effect and leak paths, and classify manual live evidence separately.

#### Scenario: The terminal host is ready to archive

r[agent_harness_runtime.live_terminal_agent_host.verification.closeout]
- GIVEN maintainers intend to sync and archive the live terminal host
- WHEN focused and lifecycle validation runs
- THEN TUI, TestBackend, pseudo-terminal, daemon-client, command, cleanup, receipt, dependency, Cairn, and relevant Nix checks MUST pass
- AND arbitrary terminal compatibility, remote security, crash-proof cleanup, task success, production readiness, and release eligibility MUST remain non-claims.
