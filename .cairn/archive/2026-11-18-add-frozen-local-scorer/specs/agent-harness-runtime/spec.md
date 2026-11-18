# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: A frozen local scorer is a guidance-only manager route

r[agent_harness_runtime.campaign_manager_eval.local_scorer.route] The evaluation lane MUST provide a frozen local scorer route that reads only the rendered brief, scores every listed candidate with an admitted frozen policy, and replies in the existing manager reply schema with candidate scores only. The route MUST NOT add offers or retire, prune or stop candidates, and MUST NOT change the manager, Campaign or the verifier.

#### Scenario: The scorer answers with guidance only

- GIVEN an admitted fixture artifact and a brief listing candidates and offers
- WHEN the route answers
- THEN the reply scores each listed candidate in brief order
- AND it adds no offers.

### Requirement: The policy artifact is versioned and identity-bound

r[agent_harness_runtime.campaign_manager_eval.local_scorer.artifact] The route MUST admit only an `animus.campaign-manager-local-scorer.v1` artifact whose BLAKE3 identity over its canonical encoding equals the selected identity. Admission MUST fail with a typed error before any scoring for unknown fields, another schema, policy, feature or action schema, out-of-bounds weights or rank bounds, an unsupported runtime API, or an identity mismatch. A file name MUST NOT serve as identity.

#### Scenario: A mismatched artifact is refused before scoring

- GIVEN artifact bytes whose identity differs from the selected identity, or with one changed weight
- WHEN the route is built
- THEN admission fails with a typed error and no score is computed.

### Requirement: Features, ranks and ties are declared

r[agent_harness_runtime.campaign_manager_eval.local_scorer.schema] The scorer MUST project only observations visible in the brief at decision time into the declared per-mille features, in brief order. It MUST compute scores in checked fixed-point integer arithmetic, MUST treat a score outside the declared rank bounds as out of range rather than clamp it, and MUST give equal scores equal ranks without random tie-breaking.

#### Scenario: Hidden facts cannot reach the policy

- GIVEN two tasks whose briefs render identically but whose hidden targets differ
- WHEN the scorer answers both
- THEN the features and replies are identical.

### Requirement: Failures fall back deterministically and visibly

r[agent_harness_runtime.campaign_manager_eval.local_scorer.fallback] The route MUST record a `scored`, `malformed-brief`, `out-of-range`, `operation-budget`, `incompatible-artifact` or `unavailable` disposition for every call. Every disposition except `scored` MUST answer with the `explore-only` reply for the same brief and record that fallback and its cost.

#### Scenario: A malformed brief falls back

- GIVEN a brief listing a signal above the task schema's bound
- WHEN the scorer answers
- THEN the reply equals `explore-only`'s reply
- AND the record shows `malformed-brief` with the fallback charged.

### Requirement: Decisions are recorded, not regenerated

r[agent_harness_runtime.campaign_manager_eval.local_scorer.record] The route MUST retain, per call, the artifact identity, per-candidate features and scores, the disposition, the fallback and deterministic cost units, up to a fixed bound past which it refuses calls. Reports and replay MUST use these records and MUST NOT re-run inference.

#### Scenario: Records bind what was decided

- GIVEN a completed job
- WHEN its decision records are read
- THEN each scored call carries the admitted identity, its features and its scores.

### Requirement: Random ranking is a budget-matched baseline

r[agent_harness_runtime.campaign_manager_eval.local_scorer.baseline] The lane MUST provide a random-ranking route that emits one uniform rank per listed candidate within the declared rank bounds from a declared seed and the call index, adds no offers, and runs on the same tasks, turns and selection budgets as the other arms.

#### Scenario: The same seed repeats

- GIVEN two runs of the random route with the same seed
- WHEN both answer the same brief sequence
- THEN their replies are identical.

### Requirement: The comparison is predeclared and reported in full

r[agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation] The lane MUST load arms, seeds, tasks, outcomes and the adoption rule from a checked Nickel protocol before running, and its report MUST give per-arm completion, rewards, fallback counts, cost units and latency, mark broader-grant arms as non-comparable, and state the go/no-go decision with null or regression results unchanged.

#### Scenario: A losing scorer is reported as no-go

- GIVEN a scorer whose mean `found` is below the adoption rule
- WHEN the comparison runs
- THEN the report states no-go with the measured values.

### Requirement: The scorer boundary is verified

r[agent_harness_runtime.campaign_manager_eval.local_scorer.verification] The change MUST include positive and negative tests for admission, projection, ranking, dispositions, records and the baseline, and MUST pass formatting, Clippy, the crate tests, the evaluator checks, Octet and Cairn validation.

#### Scenario: Negative controls stay red

- GIVEN tampered artifacts and malformed briefs
- WHEN the tests run
- THEN each is refused or falls back with its declared disposition.
