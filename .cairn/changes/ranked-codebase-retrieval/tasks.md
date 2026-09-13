# Tasks

## Phase 1: Boundary and model

- [ ] [serial] Record zvec-grep reference revision `309a66995809243d3274fa8b5bea63ab11dda1a0` at `github.com/zvec-ai/zvec-grep` (v0.2.1, Apache-2.0) and the selected ranked-retrieval invariants (BM25, RRF, routes, freshness, compact evidence) in repository references. r[animus.ranked_retrieval.boundary]
- [ ] [serial] Run and record current codebase-graph, codebase-tool-host, no-std, and Wasm baselines. r[animus.ranked_retrieval.verification]
- [ ] [serial] Add no-std ranked-query, route, fusion-policy, index-state, budget, limit, candidate, evidence, and receipt DTOs. r[animus.ranked_retrieval.model]

## Phase 2: Retrieval and routes

- [ ] [depends:animus.ranked_retrieval.model] Implement pure deterministic BM25 scoring and reciprocal rank fusion over supplied safe values. r[animus.ranked_retrieval.retrieval]
- [ ] [depends:animus.ranked_retrieval.retrieval] Implement typed route selection with fail-closed unknown or missing routes and explicit hybrid fusion policy. r[animus.ranked_retrieval.routes]
- [ ] [depends:animus.ranked_retrieval.routes] Implement freshness admission with explicit fresh and possibly-stale verdicts and stale blocking. r[animus.ranked_retrieval.freshness]
- [ ] [depends:animus.ranked_retrieval.model] Add type-aware admission-limit and binary-detection profile validation. r[animus.ranked_retrieval.admission]

## Phase 3: Evidence and consumers

- [ ] [depends:animus.ranked_retrieval.retrieval] Add compact file-oriented, source-linked evidence output with optional previews and metadata-only receipts. r[animus.ranked_retrieval.evidence]
- [ ] [depends:animus.ranked_retrieval.admission] Add frozen consumer mapping fixtures for the read-only codebase tool host and retrieval-budget resources without runtime dependencies. r[animus.ranked_retrieval.consumers]
- [ ] [depends:animus.ranked_retrieval.evidence] Add positive fixtures for one valid ranked query, exact-route passthrough, hybrid fusion, fresh index state, codec round trips, and consumer mappings. r[animus.ranked_retrieval.verification]
- [ ] [parallel] Add negative fixtures for missing postings, unknown route, missing fusion policy, stale index state, over-limit candidates, malformed evidence, missing profile, and authority overclaims. r[animus.ranked_retrieval.verification]
- [ ] [serial] Update README consumer mapping and non-claims. Add focused Cairn and Nix checks. r[animus.ranked_retrieval.boundary] r[animus.ranked_retrieval.verification]
- [ ] [depends:animus.ranked_retrieval.verification] Run focused tests before and after core changes, workspace tests, Clippy, no-std, Wasm, codecs, Cairn validation and gates, and relevant Nix checks. r[animus.ranked_retrieval.verification]
