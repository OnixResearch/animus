## Phase 1: Store profile and crate boundary

- [ ] [serial] Define the authority-scoped crate/feature boundary, explicit store descriptor, capability-location handle, session/branch scope, durability policy, lock policy, encryption posture, private-data posture, and named size bounds. r[agent_harness_runtime.durable_session_store.boundary]
- [ ] [serial] Define replay-private payload records and metadata-only safe ledger/sidecar records with explicit cross-channel BLAKE3 binding and no evidence path from private bodies. r[agent_harness_runtime.durable_session_store.private_journal]
- [ ] [depends:agent_harness_runtime.durable_session_store.boundary] Add any human-authored store/durability policy as typed Nickel with checked generated projection and Rust DTO parity; do not evaluate Nickel at runtime. r[agent_harness_runtime.durable_session_store.boundary] r[agent_harness_runtime.durable_session_store.commit]

## Phase 2: Framing and commit core

- [ ] [depends:agent_harness_runtime.durable_session_store.private_journal] Define versioned bounded frame, transaction, commit, active-leaf, recovery, and receipt DTOs with domain-separated BLAKE3 identity functions. r[agent_harness_runtime.durable_session_store.framing] r[agent_harness_runtime.durable_session_store.identity]
- [ ] [depends:agent_harness_runtime.durable_session_store.framing] Implement pure frame encoding inputs, frame validation, hash-chain validation, sequence validation, transaction planning, commit validation, and committed-head selection without filesystem, clocks, locks, or random IDs. r[agent_harness_runtime.durable_session_store.framing] r[agent_harness_runtime.durable_session_store.commit] r[agent_harness_runtime.durable_session_store.identity]
- [ ] [depends:agent_harness_runtime.durable_session_store.commit] Implement pure recovery classification for clean end, incomplete tail frame/group, stale uncommitted head, sequence gap, committed hash mismatch, unsupported schema, and mid-file corruption. r[agent_harness_runtime.durable_session_store.recovery]
- [ ] [depends:agent_harness_runtime.durable_session_store.identity] Implement metadata-only store receipts for open, append, commit, replay, recovery, lock, and close outcomes. r[agent_harness_runtime.durable_session_store.receipts]

## Phase 3: Thin local file shell

- [ ] [depends:agent_harness_runtime.durable_session_store.commit] Implement explicit capability-scoped store open/create, restrictive permissions, bounded append/read, declared data synchronization, and active-leaf commit in the authority-scoped host block. r[agent_harness_runtime.durable_session_store.boundary] r[agent_harness_runtime.durable_session_store.commit]
- [ ] [depends:agent_harness_runtime.durable_session_store.boundary] Implement exclusive writer admission and bounded snapshot readers; reject conflicting writers, stale lease identities, unsupported lock semantics, and last-writer-wins behavior. r[agent_harness_runtime.durable_session_store.locking]
- [ ] [depends:agent_harness_runtime.durable_session_store.recovery] Implement trailing-fragment quarantine/ignore only for uncommitted tails, and fail closed on committed corruption, stale-head ambiguity, unsupported migration, permission failure, and recovery write failure. r[agent_harness_runtime.durable_session_store.recovery]
- [ ] [depends:agent_harness_runtime.durable_session_store.private_journal] Enforce encryption posture admission and fail when encryption is required but no explicit host encryption service is available; do not add ambient key discovery. r[agent_harness_runtime.durable_session_store.private_journal]

## Phase 4: Conformance evidence

- [ ] [parallel] Run a shared fake/file positive corpus for append, grouped commit, replay, active-leaf restore, branching, compaction record, repair record, restart, deterministic identities, and safe sidecar projection. r[agent_harness_runtime.durable_session_store.verification]
- [ ] [parallel] Add negative file fixtures for zero/over-budget records, malformed lengths, torn frame, torn transaction, sequence gap, parent/hash mismatch, committed corruption, stale head, unsupported schema, lock conflict, permission denial, sync failure, required-encryption absence, unsafe location fallback, and raw private-data receipt leakage. r[agent_harness_runtime.durable_session_store.verification]
- [ ] [depends:agent_harness_runtime.durable_session_store.recovery] Add process-restart tests that prove only committed transaction groups and their active leaf are replayed, while uncommitted trailing data never becomes model-visible. r[agent_harness_runtime.durable_session_store.commit] r[agent_harness_runtime.durable_session_store.recovery] r[agent_harness_runtime.durable_session_store.verification]

## Phase 5: Documentation and closeout

- [ ] [serial] Update session persistence, request lifecycle, component catalog, runtime facade, dogfood readiness, safety, private-data, and focused-rail docs with the durable profile and explicit non-claims. r[agent_harness_runtime.durable_session_store.boundary] r[agent_harness_runtime.durable_session_store.receipts]
- [ ] [serial] Add a focused durable-session-store rail covering Nickel drift when present, crate isolation, fake/file conformance, restart, corruption, lock, permission, sync, and leak fixtures. r[agent_harness_runtime.durable_session_store.verification]
- [ ] [serial] Run focused runtime/std-session tests, formatting, Clippy, Nickel checks, safe-receipt and neutral-ledger rails, composition conformance, dogfood claim hygiene, Cairn validation and gates, and the relevant Nix checks; record filesystem-specific blockers without promoting them to pass. r[agent_harness_runtime.durable_session_store.verification]
