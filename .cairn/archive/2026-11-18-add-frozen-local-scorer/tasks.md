# Tasks

## Phase 1: Contracts

- [x] [serial] Author the fixture artifact and the comparison protocol in Nickel with checked JSON projections. r[agent_harness_runtime.campaign_manager_eval.local_scorer.artifact] r[agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]

## Phase 2: Pure scorer and routes

- [x] [serial] Implement artifact admission with typed failures before scoring. r[agent_harness_runtime.campaign_manager_eval.local_scorer.artifact]
- [x] [serial] Implement the brief feature projection, fixed-point scoring, rank conversion and reply rendering. r[agent_harness_runtime.campaign_manager_eval.local_scorer.schema] r[agent_harness_runtime.campaign_manager_eval.local_scorer.route]
- [x] [serial] Implement dispositions, deterministic `explore-only` fallback and bounded decision records with cost units. r[agent_harness_runtime.campaign_manager_eval.local_scorer.fallback] r[agent_harness_runtime.campaign_manager_eval.local_scorer.record]
- [x] [serial] Implement the seeded random-ranking baseline route. r[agent_harness_runtime.campaign_manager_eval.local_scorer.baseline]

## Phase 3: Comparison

- [x] [serial] Add the `--local-scorer` comparison command and its metadata-only report. r[agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
- [x] [serial] Run the comparison and record the result, including null or regression outcomes and the go/no-go decision. r[agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
  - No-go on the synthetic lane; see [evidence](evidence/comparison-2026-11-18.md).

## Phase 4: Verification

- [x] [serial] Add positive and negative tests for admission, projection, ranking, dispositions, records and the baseline; run formatting, Clippy, crate tests, the evaluator checks, Octet and Cairn validation and gates. r[agent_harness_runtime.campaign_manager_eval.local_scorer.verification]
  - All passed; Octet is `warning-only` with three findings outside this change. See [checks](evidence/comparison-2026-11-18.md#checks).
