# Design: Add memory evaluation rail

## Context

Existing memory tests prove fail-closed adapter behavior, normalized record validation, retrieval planning, admission separation, safe receipts, controls, and conformance. They do not answer whether selected memories are relevant, stale, contradictory, or useful for memory-dependent tasks. Evaluation should be explicit before Animus claims quality improvements.

## Decisions

### 1. Evaluation fixtures use safe identities

**Choice:** Define fixtures around query labels, expected record IDs, category labels, freshness, contradiction markers, abstention expectations, and safe answer-use labels.

**Rationale:** Tests can measure retrieval quality without storing raw prompts, raw transcripts, hidden memory bodies, or provider output.

### 2. Metrics are pure reducers

**Choice:** Add pure metric reducers over expected safe IDs and observed retrieval/injection ledgers for Precision@K, Recall@K, stale-hit rate, contradiction-hit rate, abstention correctness, and baseline deltas.

**Rationale:** Metric behavior should be deterministic and testable without model calls.

### 3. External benchmarks are imported, not embedded

**Choice:** Hosts may project LoCoMo-like or LongMemEval-like runs into Animus evaluation result DTOs, but datasets, judges, downloads, and provider calls remain shell-owned.

**Rationale:** Animus can compare evidence shapes without inheriting benchmark operational dependencies.

### 4. Baseline comparison is required for claims

**Choice:** Any memory-quality improvement claim must cite a no-memory or previous-policy baseline descriptor plus safe metric deltas.

**Rationale:** Memory infrastructure can add cost without helping; claims need bounded evidence.

## Risks / Trade-offs

- Safe fixtures may be less expressive than raw transcripts. Use evidence handles and BLAKE3 hashes for traceability.
- Metrics can be gamed by narrow fixtures. Require positive and negative cases, stale and contradiction cases, and baseline comparison.
- LLM-as-judge behavior is host-owned. Animus should record imported judge labels, not execute judges in core.
