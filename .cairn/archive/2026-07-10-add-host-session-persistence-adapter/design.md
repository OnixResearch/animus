# Design: Host session persistence adapter

## Context

Accepted requirements cover ledger records, session tree replay, provenance, compaction summaries, and restore contracts. The remaining gap is an adapter boundary for host-owned durable storage that dogfood shells can use without moving filesystem or database authority into reusable crates.

## Decisions

### 1. Persistence service is injected

**Choice:** A `SessionPersistenceService`-style boundary accepts validated append, snapshot, replay, restore, compact, and tombstone requests.

**Rationale:** Storage backend, file layout, database, locks, encryption, clocks, and retention policy are host decisions.

### 2. Records are safe and schema-versioned

**Choice:** Durable writes include schema version, session scope, branch identity, parent identity, event identity, redaction class, BLAKE3 content identities, counts, and outcome classes only.

**Rationale:** Replay must be deterministic enough to rebuild safe state, while raw transcripts and provider/tool bodies remain outside durable ledgers.

### 3. Fake store first

**Choice:** The initial implementation uses a deterministic in-memory fake persistence service for tests and dogfood smoke.

**Rationale:** It proves append/replay/restore semantics without choosing a production storage backend.

### 4. Failures are typed outcomes

**Choice:** Missing store, unavailable backend, stale snapshot, schema mismatch, unsafe record, partial write, replay gap, and leak rejection are typed persistence outcomes.

**Rationale:** Persistence failures should not be hidden by best-effort logs or implicit rebuilds.

## Risks / Trade-offs

- A fake store can be confused with production persistence. Receipts and docs must scope the claim.
- Too much durable metadata can leak sensitive workflow shape. Keep fields safe and justified.
- Restore semantics can grow broad. Limit the first adapter to typed safe replay and session identity restoration.
