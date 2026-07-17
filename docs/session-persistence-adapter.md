# Session persistence adapter

Animus models durable session persistence as an injected host service. Reusable logic admits explicit store descriptors, session and branch identity, schema version, redaction policy, freshness policy, and receipt policy before append, replay, restore, compaction, or tombstone work.

The safe-ledger evidence rung remains an in-memory fake over metadata-only records. A separate non-default [durable local session store](durable-session-store.md) now supplies replay-private file persistence behind an explicit capability root. Its pure runtime core and file host share framing/recovery fixtures, but private journal bodies never become safe-ledger evidence.

Interrupted-call recovery composes through an explicit host step: `animus_runtime::interrupted_tool_repair` emits an append-only application request, and the persistence host reports exact action coverage, old/new leaf identities, durability, replay validation, and executor non-invocation. Store failure, partial append, stale leaf, or missing replay validation blocks provider continuation. The reusable repair core never opens the store or invokes a tool.

[Redacted session-tree export](redacted-session-tree-export.md) starts only after a host has loaded and validated safe snapshot/replay metadata through this boundary. The exporter never reads the store, and its injected destination writer is not the session persistence service. Export synchronization evidence proves only the bounded destination write reported by that service.

The selected local host owns a bounded binary journal layout, exclusive create-new lease, private file mode, and `sync_all_each_commit` profile. Non-claims remain explicit: no encryption or key discovery, database/daemon backend, power-loss proof, filesystem correctness, backup, replication, retention, multi-writer coordination, production recovery, clock freshness, or live-provider continuity.
