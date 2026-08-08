## Why

Prime Agent separates terminal presentation, process coordination, and agent execution into three process roles: the client owns rendering and input only, a daemon supervisor owns routing, attachments, and recovery, and one worker process per session owns the session tree, scheduler, kernel, and descendants. Closing the terminal detaches the client without stopping the worker. Workers persist transcripts as JSONL and recover after restart.

Animus has the pieces in contract form (`host-control-protocol`, `session-restore-contract`, `animus-std-control`, `background-work-queues`) but no unified ownership rule that says the client never owns execution. Without it, a TUI host can grow execution authority and break detach, recovery, and headless parity. Animus needs the three-role ownership split as typed contracts, with the explicit rule that process isolation is lifecycle containment, never a security sandbox.

## What Changes

- Add the three-role ownership contract: client owns rendering and input, supervisor owns routing and attachments, worker owns one root session tree with its queue, schedules, and descendants. r[agent_harness_runtime.daemon_workers.ownership]
- Add detach and reattach semantics: client disconnect never stops the worker, and reattach restores from live streams or recovery snapshots. r[agent_harness_runtime.daemon_workers.detach]
- Add supervisor duties as typed contracts: session discovery, message routing, attachment management, worker health, and catalog scans. r[agent_harness_runtime.daemon_workers.supervisor]
- Add the containment boundary: worker and kernel process isolation is lifecycle and failure containment, and no contract may claim security sandboxing. r[agent_harness_runtime.daemon_workers.containment]
- Add a Nickel-authored daemon policy for role boundaries, attachment limits, recovery rules, and non-claims. r[agent_harness_runtime.daemon_workers.profile]
- Add metadata-only daemon receipts for attach, detach, recovery, and health events. r[agent_harness_runtime.daemon_workers.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.daemon_workers.verification]

## Impact

- **Files**: new focused daemon contract module, Nickel policy, fixtures, focused check script, docs; composition seams in `host-control-protocol`, `session-restore-contract`, `animus-std-control`, and `background-work-queues`.
- **Reusable boundary**: role contracts are pure DTOs and decisions; process spawning, sockets, and transcript persistence stay host-owned, with `bounded-exec` as the containment mechanism reference.
- **Runtime composition**: interactive TUI, headless JSON/RPC, and automation clients share one execution path through the supervisor boundary.
- **Testing**: positive fixtures for ownership, detach/reattach, recovery, and supervisor routing; negative fixtures for client-side execution authority, sandbox claims, and orphan sessions; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Process spawning, socket protocols, or transcript storage inside reusable crates.
- Security sandboxing claims for worker or kernel processes; containment is lifecycle-only.
- Multi-machine supervision or remote clients.
- Proving crash-freedom, data durability under arbitrary host failure, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: daemon boundary, policy, ownership, detach/reattach, supervisor duties, containment, evidence, and verification.
