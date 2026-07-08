# Proposal: Add orchestration adapter family

## Why

Animus already has runtime-owned Kanban orchestration DTOs and a runtime-local `KanbanBoardService` seam, but embedders that want durable board storage, task handoffs, scheduling, or process supervision should not need runtime internals to understand the host effect boundary.

A new `animus-orchestration` crate would duplicate the existing `animus-runtime` role before there is enough independent surface to justify a crate barrier. The better next step is to make orchestration host effects a public adapter family: reusable runtime keeps deterministic decisions, while `animus-adapters` exposes implementation-free contracts that hosts can implement.

## What Changes

- Define `family:orchestration` as the public adapter family for host-owned board/task graph services, handoff persistence, scheduling admission, cancellation, and lifecycle evidence.
- Keep pure task graph validation, dependency promotion, workspace admission, and tenant-safe context assembly in `animus-runtime`.
- Promote or bridge the existing `KanbanBoardService`-style seam into `animus-adapters` without adding a new orchestration crate.
- Require fail-closed behavior for missing services, denied capability, unavailable backends, invalid graph snapshots, unsafe handoffs, and out-of-scope mutations.
- Add conformance-index, facade-inventory, docs, and focused positive/negative tests before archive.

## Impact

- **Files**: future `animus-adapters` orchestration module and exports, `animus-runtime` bridge over injected orchestration services, optional `animus-std` fixture/no-op host adapter, docs, contracts index, runtime facade inventory, and adapter conformance fixtures.
- **Testing**: positive load/append/schedule-admission fixture paths; negative missing service, unavailable backend, denied capability, invalid graph, unsafe handoff, cross-tenant leak, out-of-scope mutation, raw evidence leak, and compatibility alias cases.
- **Non-claims**: this does not add a new `animus-orchestration` crate, concrete board database, dispatcher, process supervisor, VCS automation, filesystem storage layout, network transport, live multi-agent scheduler, or durable persistence backend.
