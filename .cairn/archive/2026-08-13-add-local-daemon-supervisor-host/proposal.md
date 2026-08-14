# Proposal: Add a local daemon supervisor host

## Why

Animus defines client, supervisor, worker, detach, recovery, health, containment, and transactional handoff contracts. Existing checks use deterministic injected hosts.

A Prime Agent-style product needs a concrete local supervisor, one worker process per root session tree, and a transport for attach and command routing.

Current Prime Agent also handles stale registrations, zombie workers, identity mismatch, asynchronous stop finalization, and worker retry. Animus lacks these concrete behaviors.

## What Changes

- Add a non-default local supervisor process with one exact versioned local transport and explicit endpoint authority. r[agent_harness_runtime.local_daemon_supervisor.transport]
- Add a worker launcher and registry that bind one process generation to one root session tree. r[agent_harness_runtime.local_daemon_supervisor.workers]
- Implement attach, detach, snapshot, command, event, health, and message routing without session execution in the supervisor. r[agent_harness_runtime.local_daemon_supervisor.routing]
- Add stale, zombie, identity, stop-finalization, restart, and explicit retry recovery. r[agent_harness_runtime.local_daemon_supervisor.recovery]
- Compose workers with the live Steel agent shell and durable restore. r[agent_harness_runtime.local_daemon_supervisor.session_restore]
- Add adversarial process, transport, teardown, recovery, receipt, and containment tests. r[agent_harness_runtime.local_daemon_supervisor.verification]

## Prerequisites

- `add-live-steel-agent-shell`
- `agent_harness_runtime.daemon_workers.boundary`
- `agent_harness_runtime.daemon_workers.supervisor`
- `agent_harness_runtime.daemon_workers.detach`
- `agent_harness_runtime.daemon_handoff.transaction`
- `agent_harness_runtime.session_restore.restore_matrix`
- `agent_harness_runtime.host_control.transport_neutral`

## Impact

- **Product surface**: separate non-default supervisor and worker binaries or packages.
- **Authority**: process, local endpoint, credential, clock, signal, and registry storage stay in the daemon host.
- **Execution**: workers own session execution. The supervisor owns routing and health only.
- **Evidence**: process isolation remains lifecycle containment, not a security sandbox.

## Out of Scope

- Remote network clients, multi-machine supervision, PTY migration, security sandboxing, service-manager installation, high availability, or production operations.

## Affected Specs

- `agent-harness-runtime`: transport, workers, routing, recovery, session restore, and verification.
