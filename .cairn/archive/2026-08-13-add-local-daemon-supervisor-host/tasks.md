# Tasks

## B. Transport and registry

- [x] [serial] B1: Record the reviewed current Prime Agent daemon recovery cohort and map all behavior to accepted Animus role contracts. r[agent_harness_runtime.local_daemon_supervisor.verification]
- [x] [serial] B2: Define typed local endpoint, framing, authentication, peer, limit, deadline, cancellation, and failure contracts. r[agent_harness_runtime.local_daemon_supervisor.transport]
- [x] [depends:B2] B3: Define the safe durable worker registry, process identity facts, generations, health states, and receipts. r[agent_harness_runtime.local_daemon_supervisor.workers]

## S. Supervisor and worker shell

- [x] [depends:B3] S1: Implement pure routing, attachment, health, stop, retry, and recovery decisions over supplied facts. r[agent_harness_runtime.local_daemon_supervisor.routing]
- [x] [depends:S1] S2: Implement the explicit worker launcher, readiness handshake, one-root ownership, and complete-tree teardown. r[agent_harness_runtime.local_daemon_supervisor.workers]
- [x] [depends:S2] S3: Implement snapshot-first attach, bounded event flow, commands, detach, and message routing. r[agent_harness_runtime.local_daemon_supervisor.routing]
- [x] [depends:S3] S4: Implement stale registration, zombie, identity mismatch, failed start, stop finalization, and explicit retry behavior. r[agent_harness_runtime.local_daemon_supervisor.recovery]
- [x] [depends:S4] S5: Compose the worker with live Steel session startup and durable session and kernel restore. r[agent_harness_runtime.local_daemon_supervisor.session_restore]
- [x] [depends:S5] S6: Add metadata-only daemon and worker observations with containment-only claims. r[agent_harness_runtime.local_daemon_supervisor.workers]

## T. Tests and closeout

- [x] [depends:S6] T1: Add positive process and protocol tests for start, attach, detach, continued execution, reattach, stop, restart, retry, and restore. r[agent_harness_runtime.local_daemon_supervisor.verification]
- [x] [depends:S6] T2: Add negative tests for endpoint impersonation, malformed frames, stale generations, PID reuse, identity mismatch, zombie workers, stop races, leaked children, unavailable stores, restore drift, receipt leaks, and sandbox overclaims. r[agent_harness_runtime.local_daemon_supervisor.verification]
- [x] [depends:T2] T3: Prove reusable and default dependency isolation plus local-only transport scope. r[agent_harness_runtime.local_daemon_supervisor.transport]
- [x] [depends:T3] T4: Update docs and component metadata, add focused rails, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.local_daemon_supervisor.verification]
