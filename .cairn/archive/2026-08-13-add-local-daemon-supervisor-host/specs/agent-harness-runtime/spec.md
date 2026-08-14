# Agent Harness Runtime

## ADDED Requirements

### Requirement: The first daemon transport is explicit and local

r[agent_harness_runtime.local_daemon_supervisor.transport] Animus MUST expose the concrete daemon only through a non-default versioned local transport with explicit endpoint authority, framing, authentication, peer facts, message bounds, deadlines, cancellation, and typed failures.

#### Scenario: No endpoint is configured

r[agent_harness_runtime.local_daemon_supervisor.transport.no_endpoint]
- GIVEN a client or supervisor has no explicit admitted local endpoint
- WHEN transport startup runs
- THEN it MUST fail before bind or connect
- AND it MUST NOT discover a global home-directory socket, open TCP, or select an ambient endpoint.

### Requirement: One worker generation owns one root session tree

r[agent_harness_runtime.local_daemon_supervisor.workers] The supervisor MUST bind one generation-checked worker process and safe registry record to one root session tree, while the worker alone owns session, provider, tool, Steel, child, queue, schedule, and kernel execution.

#### Scenario: A worker becomes ready

r[agent_harness_runtime.local_daemon_supervisor.workers.ready]
- GIVEN the supervisor launched an admitted worker for one root identity
- WHEN the worker completes the versioned readiness handshake with matching process and generation facts
- THEN the registry MAY mark that exact generation ready
- AND no other worker generation may own the same root concurrently.

### Requirement: Supervisor routing never executes sessions

r[agent_harness_runtime.local_daemon_supervisor.routing] The supervisor MUST own discovery, attachment, command and message routing, snapshot-first event subscriptions, health decisions, and worker lifecycle without provider, tool, Steel, or session execution.

#### Scenario: A client detaches

r[agent_harness_runtime.local_daemon_supervisor.routing.detach]
- GIVEN a client has an active attachment to a running worker
- WHEN the client detaches or disconnects
- THEN the supervisor MUST remove only that attachment and its bounded subscription
- AND worker session execution MUST continue unless a separate authorized stop command succeeds.

### Requirement: Worker recovery classifies current process facts

r[agent_harness_runtime.local_daemon_supervisor.recovery] Recovery MUST distinguish healthy, starting, stopping, stopped, disconnected, stale, identity-mismatched, zombie, unresponsive, and failed-start states, and explicit retry MUST create a new generation only after policy admits replacement.

#### Scenario: A registered PID belongs to another process

r[agent_harness_runtime.local_daemon_supervisor.recovery.pid_reuse]
- GIVEN a registry record names a live PID whose current identity facts do not match the recorded worker
- WHEN health recovery evaluates the worker
- THEN it MUST classify identity mismatch and deny attachment to that process
- AND replacement MUST use a new admitted generation without signaling the unrelated process.

#### Scenario: The last client disconnects during stop

r[agent_harness_runtime.local_daemon_supervisor.recovery.stop_finalization]
- GIVEN an authorized stop has entered the stopping state
- WHEN all clients disconnect before worker exit
- THEN the supervisor MUST continue bounded stop finalization and owned teardown
- AND the registry MUST not report stopped until the required process-tree observation completes.

### Requirement: Replacement workers restore through accepted session owners

r[agent_harness_runtime.local_daemon_supervisor.session_restore] A replacement worker MUST validate durable session and kernel heads through accepted recovery contracts before continuation, and the supervisor MUST route only safe restore status.

#### Scenario: Kernel restore is incompatible

r[agent_harness_runtime.local_daemon_supervisor.session_restore.incompatible]
- GIVEN a replacement worker finds a valid session head but incompatible kernel cohort facts
- WHEN restore admission runs
- THEN the worker MUST return a typed degraded or blocked state without provider or tool continuation
- AND the supervisor MUST not clear the kernel or claim successful resume.

### Requirement: Local daemon evidence includes adversarial process paths

r[agent_harness_runtime.local_daemon_supervisor.verification] The host MUST include positive and negative production-entrypoint tests for transport, authentication, framing, start, ready, attach, detach, routing, stop, teardown, stale and zombie recovery, identity mismatch, retry, restore, leaks, containment claims, and dependency isolation.

#### Scenario: The local daemon is ready to archive

r[agent_harness_runtime.local_daemon_supervisor.verification.closeout]
- GIVEN maintainers intend to sync and archive the local daemon host
- WHEN focused and lifecycle validation runs
- THEN daemon, handoff, host-control, process, live-shell, store, restore, receipt, Cairn, and relevant Nix checks MUST pass
- AND remote security, sandboxing, high availability, service management, production readiness, and release eligibility MUST remain non-claims.
