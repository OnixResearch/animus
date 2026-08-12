# Tasks

- [x] [serial] Define memory evaluation fixture DTOs for safe query labels, expected record IDs, scopes, categories, freshness, contradictions, abstention, and answer-use expectations. r[agent_harness_runtime.memory_evaluation.fixtures]
- [x] [depends:fixtures] Implement pure metric reducers for Precision@K, Recall@K, stale-hit rate, contradiction-hit rate, abstention correctness, and baseline deltas over safe ledgers. r[agent_harness_runtime.memory_evaluation.metrics]
- [x] [depends:metrics] Add baseline comparison descriptors that require no-memory or previous-policy evidence before quality claims. r[agent_harness_runtime.memory_evaluation.baselines]
- [x] [depends:metrics] Add optional external benchmark import descriptors without bundling datasets, providers, downloads, or judges. r[agent_harness_runtime.memory_evaluation.external_import]
- [x] [depends:baselines,external_import] Add positive metric tests and negative malformed-fixture, unsafe-body, missing-baseline, stale, contradiction, and leak fixtures. r[agent_harness_runtime.memory_evaluation.verification]
- [x] [depends:verification] Update docs and run focused memory evaluation tests, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_evaluation.verification]
