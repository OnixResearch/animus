# agent-harness-runtime Change Spec: Host session persistence adapter

## Requirements

### Requirement: Session persistence is an injected host service

r[agent_harness_runtime.session_persistence_adapter.service] Animus MUST model durable session persistence as an injected host service with typed append, snapshot, replay, restore, compact, forget/tombstone, and store-readiness operations over safe ledger records.

#### Scenario: Host supplies an in-memory persistence service

r[agent_harness_runtime.session_persistence_adapter.service.injected]
- GIVEN a host supplies an in-memory persistence service descriptor
- WHEN the runtime requests append and replay operations
- THEN Animus calls the injected service through typed requests and outcomes
- AND reusable crates do not choose a filesystem layout, database backend, lock strategy, encryption scheme, clock source, or global store path.

### Requirement: Durable persistence admission is explicit

r[agent_harness_runtime.session_persistence_adapter.admission] Persistence operations MUST require explicit store descriptor, session scope, branch identity, schema version, redaction policy, freshness policy, and receipt policy before durable append, snapshot, replay, restore, compact, or tombstone work.

#### Scenario: Missing store descriptor fails closed

r[agent_harness_runtime.session_persistence_adapter.admission.missing_store]
- GIVEN a runtime run has safe events to persist
- AND no store descriptor is injected
- WHEN persistence admission runs
- THEN Animus returns a missing-store outcome
- AND it does not write to ambient filesystem paths, global stores, caches, current working directories, or best-effort logs.

### Requirement: Fake persistence store proves replay shape

r[agent_harness_runtime.session_persistence_adapter.fake_store] The first persistence adapter evidence SHOULD include a deterministic in-memory fake store that supports append, replay, restore, compaction-summary, and tombstone flows without filesystem, database, daemon, or network authority.

#### Scenario: Replay restores safe session state

r[agent_harness_runtime.session_persistence_adapter.fake_store.replay]
- GIVEN a fake store contains a valid sequence of safe ledger records for a session branch
- WHEN replay runs
- THEN Animus rebuilds safe session state, branch links, counts, and terminal status from the records
- AND it does not require raw transcripts, provider payloads, tool bodies, or private paths.

### Requirement: Persistent records are safe ledger facts

r[agent_harness_runtime.session_persistence_adapter.safe_records] Durable session records MUST contain only safe ledger facts such as schema version, session and branch identities, parent identities, BLAKE3 content identities, redaction classes, event counts, token/count summaries, status classes, safe adapter labels, and typed replay receipt references.

#### Scenario: Unsafe raw record is rejected

r[agent_harness_runtime.session_persistence_adapter.safe_records.raw_prompt]
- GIVEN a persistence request includes raw prompt text, raw model output, tool body, provider payload, credential, private path, or unbounded log
- WHEN record validation runs
- THEN Animus rejects the record with a leak diagnostic
- AND no durable session state is updated from that record.

### Requirement: Persistence failures are typed outcomes

r[agent_harness_runtime.session_persistence_adapter.failures] Persistence and replay MUST return typed outcomes for missing store, unavailable backend, stale snapshot, schema mismatch, unsafe record, partial write, replay gap, restore conflict, tombstone conflict, and leak rejection.

#### Scenario: Stale snapshot blocks restore

r[agent_harness_runtime.session_persistence_adapter.failures.stale_snapshot]
- GIVEN a restore request references a snapshot older than the admitted freshness policy
- WHEN restore admission runs
- THEN Animus returns a stale-snapshot outcome
- AND it does not silently continue from outdated state.

### Requirement: Session persistence verification

r[agent_harness_runtime.session_persistence_adapter.verification] The session persistence adapter change MUST include positive and negative tests for service descriptors, persistence admission, fake-store append, replay, restore, compaction summaries, tombstones, missing stores, stale snapshots, schema mismatches, unsafe records, partial writes, unavailable backends, global-store defaults, replay gaps, restore conflicts, and leak rejection before archive.

#### Scenario: Closeout scopes persistence claims

r[agent_harness_runtime.session_persistence_adapter.verification.closeout]
- GIVEN maintainers prepare to archive persistence adapter work
- WHEN closeout validation runs
- THEN evidence names fake-store append/replay/restore tests, negative fixtures, safe record checks, docs, Cairn validation, and Cairn gates
- AND filesystem layout, database backend, daemon persistence, encryption subsystem, production recovery, and live-provider continuity remain omitted non-claims unless separately indexed.
