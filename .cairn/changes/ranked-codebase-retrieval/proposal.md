# Proposal: Ranked codebase retrieval

## Why

Animus agents query code through exact syntactic paths today. The capability-scoped codebase graph host and the read-only codebase tool host return exact symbol and reference matches. There is no ranked retrieval layer. An agent cannot ask "where is the authentication flow" and receive relevance-ranked, source-linked evidence without first knowing the symbols it must search.

The reviewed design reference is zvec-grep at `github.com/zvec-ai/zvec-grep` revision `309a66995809243d3274fa8b5bea63ab11dda1a0` (v0.2.1), Apache-2.0. It unifies ripgrep, BM25, and vector search behind one local-first interface and fuses ranked candidates with reciprocal rank fusion (RRF). It is a Node/npm product. Animus should adapt the ranked-retrieval discipline, not the implementation.

Repeated broad scans cost tool calls and context. Ranked, source-linked, compact evidence reduces both. The exact path stays authoritative for known symbols.

## What Changes

- Add a no-std `animus-adapter-retrieval` family with typed ranked-query, route, fusion, index-state, budget, limit, candidate, evidence, and metadata-only receipt DTOs. r[animus.ranked_retrieval.model]
- Add pure deterministic BM25 scoring and reciprocal rank fusion over supplied safe values. r[animus.ranked_retrieval.retrieval]
- Add typed route selection (exact, ranked lexical, vector, hybrid) with fail-closed routing and explicit fusion policy. r[animus.ranked_retrieval.routes]
- Add freshness admission: indexed facts report fresh or possibly stale; possibly-stale facts block ranked answers. r[animus.ranked_retrieval.freshness]
- Add type-aware admission limits and binary detection as profile-policy input. r[animus.ranked_retrieval.admission]
- Add compact, source-linked evidence output with preview omitted unless requested. r[animus.ranked_retrieval.evidence]
- Keep the exact codebase-graph and managed-ripgrep paths authoritative and unchanged for known symbols. r[animus.ranked_retrieval.exact]
- Add frozen consumer mapping fixtures without runtime dependencies. r[animus.ranked_retrieval.consumers]
- Add positive and negative no-std, codec, Wasm, consumer, and Cairn verification. r[animus.ranked_retrieval.verification]

## Lifecycle Prerequisite

None. Animus already runs the native `.cairn/` lifecycle with current generated policy.

## Impact

- **Core**: a new `animus-adapter-retrieval` no-std adapter family under `crates/` owns only DTOs, validators, deterministic ranking and fusion decisions, budgets, and BLAKE3 identities over supplied safe values.
- **Consumers**: the read-only codebase tool host and retrieval-budget resources can admit ranked queries later. Embedding hosts bind through the same family.
- **Boundary**: no embedding model, index storage, filesystem, network, process, or provider authority moves into the family.

## Out of Scope

- Vector embeddings, embedding providers, and remote-transfer authorization (a separate Basalt policy surface).
- Persistent index storage format and refresh execution (a later capability-scoped persistence host).
- Implementing ripgrep or tree-sitter parsing; exact scans remain consumer-owned.
- Proving retrieval relevance, ranking quality, index truth, or model behavior.
- The paired A/B retrieval benchmark methodology (a later animus-eval-core change).

## Affected Specs

- `animus-ranked-retrieval`: boundary, model, retrieval, routes, freshness, admission, evidence, exact, consumers, and verification.
