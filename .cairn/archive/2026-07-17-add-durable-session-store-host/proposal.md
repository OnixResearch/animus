## Why

The archived host-session-persistence change established typed service, admission, fake-store, safe-record, and failure contracts, but intentionally stopped before choosing a durable backend. The dogfood roadmap therefore still lists durable session storage as an actionable blocker.

Tau's append-only session files demonstrate the product value of incremental persistence and active-branch restoration, while also exposing failure modes Animus must address explicitly: torn tails, concurrent writers, raw private transcript handling, weak durability boundaries, and ambient file identity. Animus needs one non-default local store host that binds private replay content to safe evidence without moving filesystem authority into reusable crates.

## What Changes

- Add a non-default authority-scoped local session-store host behind an explicit store descriptor and capability-scoped location. r[agent_harness_runtime.durable_session_store.boundary]
- Separate private model-visible journal payloads from metadata-only safe ledger and receipt projections. r[agent_harness_runtime.durable_session_store.private_journal]
- Define versioned, bounded, BLAKE3-chained record framing with explicit sequence, parent, payload, and commit identities. r[agent_harness_runtime.durable_session_store.framing] r[agent_harness_runtime.durable_session_store.identity]
- Require single-writer admission, declared durability policy, committed message/leaf boundaries, and fail-closed concurrent access. r[agent_harness_runtime.durable_session_store.commit] r[agent_harness_runtime.durable_session_store.locking]
- Detect trailing partial frames, mid-file corruption, stale heads, and unsupported schema versions without silently rewriting history. r[agent_harness_runtime.durable_session_store.recovery]
- Produce safe store receipts and deterministic fake/file conformance evidence. r[agent_harness_runtime.durable_session_store.receipts] r[agent_harness_runtime.durable_session_store.verification]

## Impact

- **New authority block**: an optional std host crate or feature owns filesystem handles, locking, sync operations, permissions, and recovery orchestration.
- **Reusable crates**: retain pure framing validation, commit planning, recovery classification, and receipt projection only.
- **Privacy**: replay-private records are never durable evidence or default export input; safe sidecars contain hashes, counts, status classes, and non-claims.
- **Claims**: the first backend proves bounded local append/replay behavior for its declared durability profile. It does not claim encryption, distributed coordination, daemon recovery, multi-host replication, production backup, or immunity to filesystem and kernel failure.
