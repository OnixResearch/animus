# Tasks

- [x] [serial] Define retrieval quality policy descriptors for relevance, confidence, freshness, match reason, category priority, manual pins, and diversity posture. r[agent_harness_runtime.memory_retrieval_quality.policy]
- [x] [depends:policy] Extend pure retrieval planning to apply deterministic quality policy without backend-specific ordering authority. r[agent_harness_runtime.memory_retrieval_quality.planning]
- [x] [depends:planning] Add safe optional rerank evidence DTOs and validation for bounded scores, reason labels, policy identity, and leak rejection. r[agent_harness_runtime.memory_retrieval_quality.rerank_evidence]
- [x] [depends:planning] Add deterministic diversity and duplicate-group outcomes with safe skip reasons. r[agent_harness_runtime.memory_retrieval_quality.diversity]
- [x] [depends:rerank_evidence,diversity] Add positive ranking/diversity tests and negative malformed-score, stale, over-budget, conflict, unsafe-rerank, and backend-label fixtures. r[agent_harness_runtime.memory_retrieval_quality.verification]
- [x] [depends:verification] Update docs and run focused retrieval-quality tests, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_retrieval_quality.verification]
