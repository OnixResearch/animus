# Proposal: Enrich memory retrieval quality policy

## Why

The current memory retrieval planner is safe and deterministic, but quality policy is intentionally simple: adapter-declared relevance, record confidence, scope, kind, and identity determine ordering before budget and conflict filters. Agent memory references highlight additional retrieval patterns that matter once memory becomes category-aware: recency, hybrid match reasons, diversity, rerank evidence, stale-fact handling, and contradiction avoidance.

Animus should adapt these as policy descriptors and pure planner inputs rather than adopting any concrete vector, BM25, cross-encoder, graph, or framework implementation.

## What Changes

- Define a retrieval-quality policy profile with safe weights or ordering posture for relevance, confidence, freshness, match reason, category priority, manual pins, and diversity groups.
- Add deterministic diversity and conflict handling so near-duplicate memories do not consume the whole prompt budget.
- Add optional rerank evidence DTOs that can be supplied by host-owned rerankers but validated and applied by reusable planning logic.
- Add focused positive and negative fixtures for ranking, budget, diversity, stale records, contradictory records, malformed rerank evidence, and backend-label neutrality.

## Impact

- **Files**: future memory planner DTOs/core, docs, tests, and Cairn artifacts.
- **Testing**: positive hybrid/rerank/diversity fixtures and negative malformed-score, stale, over-budget, conflict, unsafe-rerank, and backend-leak fixtures.
- **Non-claims**: this does not implement lexical search, vector search, reranking models, graph traversal, embeddings, or memory storage.
