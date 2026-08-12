## Phase 1: Contracts and reviewed profile

- [x] [serial] Inventory context-reference, retrieval-budget, cap-std, tool-executor, mutating-tool notification, component-catalog, and receipt owners and document the no-duplication boundary. r[agent_harness_runtime.codebase_graph_host.boundary] r[agent_harness_runtime.codebase_graph_host.verification]
- [x] [serial] Define no-std service request/result, symbol/location/span, index status, file event, budget, failure, provenance, and receipt DTOs without parser or host path types. r[agent_harness_runtime.codebase_graph_host.boundary] r[agent_harness_runtime.codebase_graph_host.receipts]
- [x] [serial] Author and export a typed Nickel Rust-first index profile with language/query versions, capability-root labels, exclusions, named budgets, stale-event policy, cache posture, redaction, and domain-separated BLAKE3 identity. r[agent_harness_runtime.codebase_graph_host.profile]

## Phase 2: Functional index and query core

- [x] [depends:agent_harness_runtime.codebase_graph_host.profile] Implement pure profile validation, file-observation admission, stable path ordering, parser contribution projection, and graph-delta application over in-memory inputs. r[agent_harness_runtime.codebase_graph_host.pure_core]
- [x] [depends:agent_harness_runtime.codebase_graph_host.pure_core] Implement pure create/modify/delete/rename reconciliation plus stale classification for overflow, event gaps, ambiguous rename, failed reads, and unsupported events. r[agent_harness_runtime.codebase_graph_host.incremental_index]
- [x] [depends:agent_harness_runtime.codebase_graph_host.pure_core] Implement deterministic definition, reference, symbol, and scope query planning, stable bounded result ordering, truncation, and syntactic-evidence projection. r[agent_harness_runtime.codebase_graph_host.retrieval]
- [x] [depends:agent_harness_runtime.codebase_graph_host.profile] Implement pure cache descriptor validation and BLAKE3 identity binding for workspace, source snapshot, profile, parser, query, schema, and graph identities. r[agent_harness_runtime.codebase_graph_host.cache]
- [x] [depends:agent_harness_runtime.codebase_graph_host.receipts] Implement metadata-only index/query receipt projection and leak rejection. r[agent_harness_runtime.codebase_graph_host.receipts]

## Phase 3: Optional host block

- [x] [depends:agent_harness_runtime.codebase_graph_host.boundary] Add a focused non-default std host block with an explicit parser cohort and prove default/no-std dependency isolation. r[agent_harness_runtime.codebase_graph_host.boundary]
- [x] [depends:agent_harness_runtime.codebase_graph_host.pure_core] Implement capability-scoped bounded initial repository reads and Rust parsing while rejecting traversal, symlink escape, special files, binary/NUL input, unsupported encoding, and over-budget observations. r[agent_harness_runtime.codebase_graph_host.incremental_index]
- [x] [depends:agent_harness_runtime.codebase_graph_host.incremental_index] Implement watcher/event ingestion, stale-state blocking, bounded rescan recovery, cancellation, cleanup, and direct query handling without cloning the full index per request. r[agent_harness_runtime.codebase_graph_host.incremental_index]
- [x] [depends:agent_harness_runtime.codebase_graph_host.cache] Implement optional cache load/save/quarantine/rebuild under an explicit cache capability root without source mutation or global fallback. r[agent_harness_runtime.codebase_graph_host.cache]
- [x] [depends:agent_harness_runtime.codebase_graph_host.retrieval] Wire admitted graph queries through context references, retrieval budgets, and a read-only host tool capability pack; wire committed edit notifications without giving the graph service mutation authority. r[agent_harness_runtime.codebase_graph_host.retrieval]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive fixtures for Rust initial indexing, definitions, references, local scopes, stable ordering, bounded truncation, create/modify/delete/rename updates, direct queries, cache reuse, stale rescan recovery, context expansion, tool dispatch, and safe receipts. r[agent_harness_runtime.codebase_graph_host.verification]
- [x] [parallel] Add negative fixtures for unsupported language/query version, malformed profile/hash/cache, binary/NUL/encoding failures, oversized files or results, traversal, symlink escape, special files, watcher overflow/gap, ambiguous rename, deleted roots, stale answers, missing service/capability, cache corruption, unsafe path projection, and receipt leaks. r[agent_harness_runtime.codebase_graph_host.incremental_index] r[agent_harness_runtime.codebase_graph_host.cache] r[agent_harness_runtime.codebase_graph_host.verification]
- [x] [depends:agent_harness_runtime.codebase_graph_host.verification] Add source-shape guards proving no-std and runtime crates do not read repositories, start watchers, mmap cache files, spawn index workers, or import tree-sitter host dependencies. r[agent_harness_runtime.codebase_graph_host.boundary] r[agent_harness_runtime.codebase_graph_host.verification]

## Phase 5: Documentation and closeout

- [x] [serial] Update context-reference, retrieval-budget, tool-executor, cap-std, component-catalog, runtime-facade, safety, dogfood, focused-rail, and README reference documentation with syntactic-evidence and unsupported-language boundaries. r[agent_harness_runtime.codebase_graph_host.boundary] r[agent_harness_runtime.codebase_graph_host.receipts]
- [x] [serial] Add a focused codebase-graph rail covering Nickel drift, Rust parser/query fixtures, incremental reconciliation, stale recovery, cache identity, capability roots, tool/context integration, dependency isolation, and receipt safety. r[agent_harness_runtime.codebase_graph_host.verification]
- [x] [depends:agent_harness_runtime.codebase_graph_host.verification] Run focused adapter/host/context/tool tests, formatting, Clippy, component and composition rails, safe-receipt checks, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.codebase_graph_host.verification]
