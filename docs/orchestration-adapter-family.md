# Orchestration adapter family

Animus exposes durable board, task, handoff, scheduling, cancellation, and lifecycle-evidence host effects through `family:orchestration`, not through a new `animus-orchestration` crate. Reusable orchestration decisions remain in `animus-runtime`; host authority crosses the implementation-free `animus_adapters::orchestration` contracts.

## Decision and scope

The current boundary is an adapter family because the pure orchestration decision core is still runtime-owned. A future crate split requires a separate accepted change. This family adds no concrete board database, dispatcher, process supervisor, worker roster, VCS automation, filesystem storage layout, network transport, clock source, or persistence backend.

`animus_adapters::orchestration` defines no-std DTOs and service methods for:

- loading a host-supplied board/task graph snapshot by safe board label;
- appending a bounded task handoff through host-owned storage;
- admitting scheduling operations before a host scheduler runs work;
- requesting cancellation with typed safe labels;
- recording bounded lifecycle evidence with redaction markers and BLAKE3 hashes.

## Runtime ownership and bridge

`animus-runtime` continues to own Kanban graph validation, dependency promotion, workspace mutation admission, tenant-safe context assembly, and the existing `KanbanBoardService` export. New embedders can implement `animus_adapters::orchestration::OrchestrationService` and wrap it with `animus_runtime::OrchestrationAdapterBoardService` when they need the compatibility Kanban seam.

Existing `KanbanBoardService` implementations remain source-compatible. The bridge is a migration path, not a removal of the runtime-local seam.

## Fail-closed behavior

Adapter dispatch helpers return typed failures for missing services, unavailable backends, denied capabilities, invalid graph snapshots, unsafe handoffs, cross-tenant dependencies, out-of-scope workspace mutations, unsupported scheduling operations, and schema mismatches. They do not discover board stores, create schedulers, spawn workers, read host state, call VCS commands, open paths, contact networks, or construct persistence fallbacks.

## Safe evidence

Orchestration receipts record only family labels, operation kinds, board/task labels, status classes, counts, redaction markers, policy labels, and BLAKE3 hashes of bounded DTO projections. Receipt validation rejects raw prompts, provider payloads, credentials, full private paths, hidden reasoning, unbounded logs, host database rows, and raw task bodies.

## Focused checks

- `cargo test -p animus-adapters orchestration --lib`
- `cargo test -p animus-runtime adapter_family_bridge --lib`
- `scripts/cairn/check-adapter-conformance-index --root .`
- `scripts/cairn/check-runtime-facade-inventory --root .`

These checks prove the contract, compatibility bridge, fail-closed negative cases, safe evidence rejection, contracts-index coverage, and facade-inventory coverage. They do not prove production scheduling or durable host storage.
