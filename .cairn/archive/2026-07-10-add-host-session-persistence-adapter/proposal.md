# Proposal: Add host session persistence adapter

## Why

Animus has neutral ledger, provenance, session restore, branch tree, compaction, and safe receipt contracts, but the dogfood readiness roadmap still lists session persistence as an actionable blocker. Reusable crates should not own storage, yet host shells need a typed persistence adapter that can persist and replay safe session facts without raw transcripts, provider payloads, private paths, or global stores.

A host-owned persistence adapter will make durable storage explicit, scoped, receipt-backed, and fail-closed when unavailable.

## What Changes

- Define a host session persistence service boundary for append, snapshot, replay, restore, compact, and forget/tombstone operations over safe ledger records.
- Require explicit store descriptors, session scope, freshness, schema version, redaction policy, and receipt policy before durable writes.
- Add deterministic in-memory fake persistence fixtures for dogfood and replay tests.
- Preserve failure classes for missing store, unavailable backend, stale snapshot, schema mismatch, unsafe record, partial write, and leak rejection.
- Keep filesystem/database layout, clocks, locking, encryption, compaction storage, and global session stores host-owned.

## Impact

- **Files**: persistence adapter DTOs, fake in-memory service, replay/restore reducers, docs, tests, dogfood receipt integration, and Cairn evidence.
- **Testing**: positive append/replay/restore fake cases plus negative missing store, stale snapshot, schema mismatch, unsafe record, partial write, unavailable backend, global-store default, and receipt leak fixtures.
- **Non-claims**: this does not add a default global store, database dependency, file layout, encryption subsystem, live provider continuity, daemon persistence, or production session recovery.
