# Proposal: Add memory evaluation rail

## Why

Animus has strong DTO validation and conformance fixtures for memory safety, but it does not yet define a reusable way to measure whether memory retrieval improves outcomes. Agent memory references recommend measuring precision, recall, staleness, contradiction, answer use, and no-memory baselines before claiming memory quality.

Animus should add a memory evaluation rail that remains pure, fixture-driven, and safe. Hosts can run LoCoMo, LongMemEval, or private suites externally, but reusable Animus should own the typed fixture and metric contracts.

## What Changes

- Define memory evaluation fixture DTOs for queries, expected safe record IDs, scope, category, freshness, contradiction, abstention, and answer-use expectations.
- Add pure metric projection for Precision@K, Recall@K, stale-hit rate, contradiction-hit rate, abstention correctness, and memory-vs-baseline deltas using safe IDs and labels.
- Add optional external benchmark import descriptors for LoCoMo-like and LongMemEval-like results without bundling datasets or provider judges.
- Add focused positive and negative fixtures for metric calculation, missing expected IDs, malformed results, unsafe bodies, and baseline comparison.

## Impact

- **Files**: future evaluation DTOs, pure metric reducers, docs, tests, check script or focused rail, and Cairn artifacts.
- **Testing**: positive metric fixtures and negative malformed-fixture, unsafe-body, missing-baseline, stale, contradiction, and leak fixtures.
- **Non-claims**: this does not add benchmark datasets, LLM judges, network downloads, provider calls, or product-quality claims.
