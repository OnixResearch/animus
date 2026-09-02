# Design: Ranked codebase retrieval

## Context

Animus owns typed agent-loop fabric and adapter contracts. The codebase graph host and read-only codebase tool host provide exact syntactic queries. Ranked retrieval is the missing layer: intent-driven discovery with ranked lexical fusion, distinct from exact symbol lookup.

zvec-grep at `github.com/zvec-ai/zvec-grep` revision `309a66995809243d3274fa8b5bea63ab11dda1a0` is the reviewed design reference for BM25 ranking, reciprocal rank fusion across query groups, route selection (exact, FTS, vector, hybrid), explicit freshness and stale reporting, and compact source-linked agent output. Animus adapts these as a no-std, authority-free family. Consumers own indexing, storage, refresh, embedders, and execution.

## Success Contract

Given in-memory postings, term, ranking, route, fusion, index-state, budget, profile, and result facts, the family returns a deterministic admitted ranked response or a typed blocked decision. A ranked query binds one route, one fusion policy, finite limits, and one index-state verdict.

## Decisions

### 1. Keep the exact path authoritative

Ranked retrieval supplements the codebase-graph exact queries and managed ripgrep scans. Known symbols stay on the exact route. Ranked results do not suppress exact matches.

### 2. Put ranked semantics in a no-std adapter family

A new `animus-adapter-retrieval` crate owns DTOs, validators, deterministic BM25 and RRF decisions, budgets, and metadata-only BLAKE3 receipts. It has no filesystem, embedding, network, index-process, clock, or provider authority.

### 3. Model routes as typed choices

The route type enumerates exact, ranked lexical, vector, and hybrid modes. Hybrid mode requires an explicit fusion policy. An unknown or missing route fails closed.

### 4. Fail closed on missing or stale index facts

Index state reports fresh or possibly stale. A possibly-stale verdict blocks ranked answers and names a bounded rebuild or an exact-route fallback. This mirrors the codebase-graph stale-query posture.

### 5. Keep agent evidence compact and source-linked

Output is file-oriented and source-linked. Full previews are omitted unless requested. Receipts carry counts, safe labels, BLAKE3 identities, and raw-output-omitted markers.

### 6. Express type-aware admission as profile policy

Index admission limits (code, text, structured, image) and binary detection are typed profile inputs validated by the family, not hard-coded behavior.

### 7. Bind decisions to domain-separated identities

Query, fusion, route, and result-set identities use domain-separated BLAKE3 hashes so consumers can detect drift and bind evidence.

### 8. Consumers own runtime semantics

Index construction, storage, refresh, embedder choice, and execution stay consumer-owned. The family validates supplied facts only.

## Alternatives Considered

### Port the zvec-grep Node implementation

Rejected. The product is a Node 22 npm package. Animus cores are no-std Rust with deterministic receipts. The ranked discipline is portable; the runtime is not.

### Adopt Tantivy directly in the shell and skip the pure family

Rejected as the primary deliverable. A bounded pure BM25 and RRF core must exist first so decisions are deterministic, testable, and Wasm-compatible. A Tantivy-backed storage host can bind later.

### Lexical ranking without hybrid fusion

Rejected. Multi-group fusion is the point of intent-driven agent discovery. Single-route ranking remains available.

### Fold ranked retrieval into the codebase graph host

Rejected. The graph host answers exact syntactic queries. Ranked retrieval is a distinct capability with distinct budgets, admission, and freshness semantics.

## Risks and Controls

- Consumers can overclaim search quality. Non-claims state the family proves deterministic ranking structure, not relevance.
- Index truth can drift. The freshness verdict blocks ranked answers; rebuild is consumer-owned.
- Fusion can bury an exact intent. The exact route stays available and default for known symbols.

## Non-Claims

This design does not prove embedding quality, retrieval relevance, index completeness, ranking quality, production search performance, provider behavior, persistence, production readiness, or release eligibility.
