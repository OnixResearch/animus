# Design: Adapt Pi session tree ledger for Animus

## Context

Pi stores sessions as append-only JSONL entries linked by `id` and `parentId`. That shape supports in-place branch navigation, labels, compaction entries, and branch summaries while preserving abandoned paths. Animus should adapt the lifecycle semantics, not the storage implementation.

Animus already separates core state, adapter contracts, Preserves boundary payloads, Steel turn decisions, and std-shell orchestration. The ledger must fit those crate boundaries and remain testable without filesystem, clocks, providers, or UI.

## Decisions

### 1. Ledger entries are neutral runtime DTOs

**Choice:** Define Animus-owned entry types for user/model/tool/summary/label/runtime metadata with validated IDs and parent links.

**Rationale:** The runtime needs a stable replay contract that can be serialized through Preserves or stored by embedders without inheriting Pi's exact JSONL schema.

**Boundary:** The functional core validates and replays in-memory entries. The shell or embedder owns persistence, timestamps, path layout, and storage migration.

### 2. Active leaf determines model-visible context

**Choice:** Rebuild model-visible context by walking from the active leaf to the root, ordering entries root-to-leaf, and applying summary entry semantics.

**Rationale:** Branching must not duplicate whole sessions or lose abandoned history. The active leaf is the minimal state needed to select a conversation path.

### 3. Summaries are first-class ledger entries

**Choice:** Model-visible compaction and branch handoff summaries become explicit entry kinds with safe metadata and bounded payloads.

**Rationale:** Compaction and branch navigation need durable, reviewable context transitions rather than hidden in-memory rewrites.

### 4. Labels are metadata, not model context

**Choice:** Labels attach to target entries for operator navigation but do not enter model-visible context unless a host explicitly converts them.

**Rationale:** Labels are useful for UI/RPC selectors and provenance but should not silently steer model behavior.

### 5. Persistence stays outside the reusable core

**Choice:** Runtime APIs depend on injected ledger services and typed in-memory snapshots, not files, directories, databases, or Pi session managers.

**Rationale:** Animus embedders should be able to back the same contract with memory, JSONL, SQLite, object stores, or host applications.

## Risks / Trade-offs

- A too-rich ledger can become a product database API. Keep v1 entry kinds limited to runtime replay needs.
- Summary behavior can accidentally hide tool-result/tool-call pairing bugs. Negative tests must reject orphaned tool results and invalid summary boundaries.
- Stable IDs can tempt core code to generate randomness or read clocks. Require caller-supplied IDs and validate them in pure code.
