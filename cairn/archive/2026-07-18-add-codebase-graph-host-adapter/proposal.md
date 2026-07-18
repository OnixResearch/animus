## Why

Animus can resolve bounded file, directory, diff, image, URL, and document context, but it has no repository symbol graph for targeted definition/reference navigation or incremental retrieval after edits. Agents therefore fall back to repeated text scans even when a bounded syntactic index could answer the query with less context and clearer provenance.

Grok Build's tree-sitter codebase graph is a useful mechanism: parallel initial indexing, incremental file-event updates, direct queries without cloning the full index, versioned language queries, and rebuildable caches. Animus should adapt that shape as an optional host-owned retrieval service while keeping filesystem watching, parser dependencies, mmap/cache I/O, and workspace authority outside reusable runtime crates.

## What Changes

- Define no-std codebase-graph service contracts and bounded query/result DTOs for symbols, definitions, references, scopes, locations, index state, and typed failures. r[agent_harness_runtime.codebase_graph_host.boundary]
- Author a typed index profile for language/query versions, admitted roots, exclusions, file and result budgets, event handling, cache posture, and domain-separated BLAKE3 profile identity. r[agent_harness_runtime.codebase_graph_host.profile]
- Split deterministic parsing, graph-delta, query, ordering, and result-projection logic from a thin filesystem/watch/cache shell. r[agent_harness_runtime.codebase_graph_host.pure_core]
- Implement bounded initial indexing and incremental create/modify/delete/rename reconciliation over explicit host events, with stale/overflow states requiring rescan before trusted answers. r[agent_harness_runtime.codebase_graph_host.incremental_index]
- Treat every on-disk index as a derived rebuildable cache bound to workspace/source/profile/parser/query identities. r[agent_harness_runtime.codebase_graph_host.cache]
- Integrate graph results with context references, retrieval budgets, and explicit read-only tool capability packs without granting filesystem or mutation authority. r[agent_harness_runtime.codebase_graph_host.retrieval]
- Emit metadata-only index/query receipts and explicit syntactic-evidence non-claims. r[agent_harness_runtime.codebase_graph_host.receipts]
- Add positive and negative initial, incremental, cache, boundary, retrieval, and leak fixtures. r[agent_harness_runtime.codebase_graph_host.verification]

## Impact

- **Adapter contracts**: gain bounded data-only service requests and responses; they do not gain tree-sitter, filesystem, watcher, mmap, thread-pool, or cache dependencies.
- **Optional std host block**: owns capability-scoped repository reads, parser/index realization, watcher events, cache files, and resource limits behind a non-default feature.
- **Runtime/tool/context integration**: consumes admitted bounded graph results through existing context-reference, retrieval-budget, tool-admission, and receipt boundaries.
- **Language scope**: the first evidenced profile targets Rust. Nickel, Steel, and other languages remain unsupported until separate grammar/query fixtures prove their exact syntactic scope.
- **Claims**: graph answers are bounded syntactic navigation evidence. They do not prove compiler name resolution, type correctness, semantic equivalence, repository completeness, or source correctness.
