# Design: Surface memory family in contract index

## Context

`docs/memory-adapter-contracts.md` describes the generic memory adapter family, and `crates/animus-adapters/src/front/memory.rs` exports implemented DTOs and planners. The contracts index currently lists maintained families such as model, tool, context, observability, and orchestration, but memory is absent from that table. The README references section also lacks the memory techniques repository now being used as a bounded adaptation source.

## Decisions

### 1. Memory is a maintained public family

**Choice:** Add `family:memory` to the maintained contracts table with owner module, stability posture, authority boundary, safe receipt behavior, serialization posture, focused surface, and focused checks.

**Rationale:** Memory DTOs are public adapter contracts and should be discoverable through the same index as other families.

### 2. Conformance metadata owns the rail

**Choice:** Add memory family metadata and case dispositions to `cairn/adapter-conformance-index.ncl` or an equivalent checked manifest.

**Rationale:** The existing index rail should fail if memory docs claim a maintained family without safe conformance coverage.

### 3. External references are bounded

**Choice:** Add Agent Memory Techniques to README references and docs as a source for taxonomy, retrieval, evaluation, and production patterns, with explicit non-adoption claims for notebooks, framework dependencies, managed services, provider calls, datasets, and licensing-sensitive code.

**Rationale:** Reference provenance should be durable without making external code canonical.

### 4. Index evidence is safe

**Choice:** Index validation records family IDs, owner modules, check labels, and BLAKE3 hashes of manifest lines rather than raw prompts, provider payloads, credentials, or host-local paths.

**Rationale:** Contract discovery should remain safe for release evidence.

## Risks / Trade-offs

- Adding the family to the index may expose stale or incomplete focused checks. The conformance rail should identify gaps explicitly.
- Reference documentation can drift into implementation claims. Non-adoption language must stay clear.
- This is mostly surfacing work; behavior changes belong in separate memory taxonomy, routing, retrieval, evaluation, or production policy Cairns.
