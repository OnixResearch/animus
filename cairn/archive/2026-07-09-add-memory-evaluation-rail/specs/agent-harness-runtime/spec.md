# agent-harness-runtime Change Spec: Memory evaluation rail

## Requirements

### Requirement: Safe memory evaluation fixtures

r[agent_harness_runtime.memory_evaluation.fixtures] Memory evaluation fixtures MUST represent memory-dependent queries, expected relevant record identities, scopes, categories, freshness expectations, contradiction markers, abstention expectations, and answer-use labels using safe IDs, labels, counts, and hashes rather than raw prompts, transcripts, memory bodies, provider payloads, or full paths.

#### Scenario: Fixture omits raw memory bodies

r[agent_harness_runtime.memory_evaluation.fixtures.safe]
- GIVEN a memory evaluation fixture names expected records for a memory-dependent query
- WHEN fixture validation runs
- THEN validation accepts safe query labels, record IDs, scopes, categories, and BLAKE3 evidence hashes
- AND rejects raw prompt text, hidden memory bodies, credentials, provider payloads, and full path dumps.

### Requirement: Pure memory evaluation metrics

r[agent_harness_runtime.memory_evaluation.metrics] Memory evaluation metrics MUST be pure deterministic reducers over safe expected-record fixtures and observed retrieval or injection ledgers, producing Precision@K, Recall@K, stale-hit rate, contradiction-hit rate, abstention correctness, and answer-use summaries without provider calls or storage access.

#### Scenario: Metrics separate relevant, stale, and contradictory hits

r[agent_harness_runtime.memory_evaluation.metrics.reducers]
- GIVEN expected relevant records and observed retrieval outcomes contain relevant, stale, contradictory, and irrelevant safe record IDs
- WHEN metric reduction runs
- THEN precision, recall, stale-hit, contradiction-hit, and abstention metrics are computed deterministically
- AND the metric report contains no raw memory or prompt bodies.

### Requirement: Baseline-backed memory quality claims

r[agent_harness_runtime.memory_evaluation.baselines] Any Animus-owned memory-quality claim MUST cite a validated baseline descriptor for no-memory behavior or a previous memory policy, and MUST report safe metric deltas rather than unsupported qualitative claims.

#### Scenario: Missing baseline blocks quality claim

r[agent_harness_runtime.memory_evaluation.baselines.required]
- GIVEN a report claims a retrieval policy improves memory-dependent answers
- AND no validated baseline descriptor is present
- WHEN quality-claim validation runs
- THEN validation rejects the claim with a missing-baseline diagnostic
- AND no release evidence asserts the improvement.

### Requirement: External benchmark imports are host-owned

r[agent_harness_runtime.memory_evaluation.external_import] Hosts MAY import LoCoMo-like, LongMemEval-like, or private benchmark results into Animus evaluation DTOs, but reusable Animus logic MUST NOT download datasets, execute judges, call providers, or treat imported benchmark scores as raw memory evidence.

#### Scenario: Imported benchmark result stays bounded

r[agent_harness_runtime.memory_evaluation.external_import.safe]
- GIVEN a host imports an external benchmark result summary
- WHEN Animus validates the import descriptor
- THEN the descriptor records benchmark family, run label, safe metric labels, counts, hashes, and policy identity
- AND raw benchmark conversations, judge prompts, provider outputs, and credentials are rejected.

### Requirement: Memory evaluation verification

r[agent_harness_runtime.memory_evaluation.verification] The memory evaluation rail MUST include focused positive and negative tests for fixture validation, pure metric reduction, stale and contradictory hits, abstention, baseline enforcement, external import validation, malformed fixtures, unsafe bodies, and leak rejection.

#### Scenario: Closeout evidence covers evaluation fixtures

r[agent_harness_runtime.memory_evaluation.verification.closeout]
- GIVEN maintainers intend to archive the memory evaluation rail
- WHEN closeout validation runs
- THEN evidence names focused fixture, metric, baseline, external-import, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
