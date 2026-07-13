# Session persistence adapter

Animus models durable session persistence as an injected host service. Reusable logic admits explicit store descriptors, session and branch identity, schema version, redaction policy, freshness policy, and receipt policy before append, replay, restore, compaction, or tombstone work.

The first evidence rung is an in-memory fake store over safe ledger records. Records contain schema labels, session/branch IDs, parent hashes, BLAKE3 identities, counts, status classes, safe adapter labels, and replay receipts only.

Interrupted-call recovery composes through an explicit host step: `animus_runtime::interrupted_tool_repair` emits an append-only application request, and the persistence host reports exact action coverage, old/new leaf identities, durability, replay validation, and executor non-invocation. Store failure, partial append, stale leaf, or missing replay validation blocks provider continuation. The reusable repair core never opens the store or invokes a tool.

[Redacted session-tree export](redacted-session-tree-export.md) starts only after a host has loaded and validated safe snapshot/replay metadata through this boundary. The exporter never reads the store, and its injected destination writer is not the session persistence service. Export synchronization evidence proves only the bounded destination write reported by that service.

Non-claims: filesystem layout, database backend, daemon persistence, encryption, production recovery, lock strategy, clock freshness, raw journal backup, and live-provider continuity remain host-owned and unimplemented unless separately indexed.
