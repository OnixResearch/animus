# agent-harness-runtime Change Spec: Memory retrieval quality policy

## Requirements

### Requirement: Declarative retrieval quality policy

r[agent_harness_runtime.memory_retrieval_quality.policy] Memory retrieval quality MUST be controlled by validated declarative policy descriptors for relevance, confidence, freshness, match reason, category priority, manual pins, and diversity posture, rather than by concrete backend identity or hidden provider behavior.

#### Scenario: Policy orders semantic and recent candidates deterministically

r[agent_harness_runtime.memory_retrieval_quality.policy.ordering]
- GIVEN admitted candidates with semantic match, recent match, different confidence scores, and category metadata
- WHEN retrieval quality policy is validated and applied
- THEN ordering follows the declared policy deterministically
- AND the plan records safe order reasons without exposing raw memory bodies.

### Requirement: Quality-aware retrieval planning remains pure

r[agent_harness_runtime.memory_retrieval_quality.planning] Quality-aware retrieval planning MUST remain pure deterministic logic over normalized records, validated policy, safe match reasons, category metadata, freshness, confidence, optional rerank evidence, diversity groups, conflicts, and prompt budgets, and MUST NOT call storage, embedding, provider, filesystem, clock, or network services.

#### Scenario: Equivalent records from different backends remain equivalent

r[agent_harness_runtime.memory_retrieval_quality.planning.backend_neutral]
- GIVEN two adapters return the same normalized candidates, scores, match reasons, and category metadata under the same policy
- WHEN quality-aware retrieval planning runs
- THEN selected records, skipped records, ranks, budget use, and ledgers are equivalent
- AND backend labels remain safe metadata instead of ranking authority.

### Requirement: Optional rerank evidence is safe

r[agent_harness_runtime.memory_retrieval_quality.rerank_evidence] Host-owned rerankers MAY provide bounded rerank scores and reason labels, but reusable logic MUST validate score bounds, safe labels, source identity, policy identity, and leak posture before rerank evidence can influence ordering.

#### Scenario: Unsafe rerank evidence is ignored or rejected

r[agent_harness_runtime.memory_retrieval_quality.rerank_evidence.unsafe]
- GIVEN a rerank adapter returns provider payloads, raw memory text, credentials, malformed scores, or mismatched policy identity
- WHEN rerank evidence validation runs
- THEN validation rejects the rerank evidence with a typed diagnostic
- AND retrieval planning proceeds only with admitted safe inputs or fails closed under strict policy.

### Requirement: Deterministic diversity handling

r[agent_harness_runtime.memory_retrieval_quality.diversity] Retrieval planning MUST support deterministic diversity or duplicate-group handling that prevents repeated near-duplicate memories from exhausting the prompt budget while preserving safe skip reasons.

#### Scenario: Duplicate group does not consume all budget

r[agent_harness_runtime.memory_retrieval_quality.diversity.duplicates]
- GIVEN multiple admitted candidates belong to the same diversity group
- AND policy limits repeated evidence from that group
- WHEN retrieval planning selects memory records
- THEN at most the policy-admitted group members consume budget
- AND skipped duplicates record safe duplicate or diversity reasons.

### Requirement: Memory retrieval quality verification

r[agent_harness_runtime.memory_retrieval_quality.verification] The retrieval-quality adaptation MUST include focused positive and negative tests for policy validation, deterministic ordering, backend neutrality, rerank evidence validation, diversity handling, stale records, conflicts, over-budget records, and receipt leak rejection.

#### Scenario: Closeout evidence covers retrieval quality fixtures

r[agent_harness_runtime.memory_retrieval_quality.verification.closeout]
- GIVEN maintainers intend to archive the retrieval-quality adaptation
- WHEN closeout validation runs
- THEN evidence names focused quality-policy, pure-planner, rerank, diversity, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
