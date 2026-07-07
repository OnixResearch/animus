# agent-harness-runtime Change Spec: Evaluate router recipe coverage

## Requirements

### Requirement: ResearchSpec-framed router evaluation

r[agent_harness_runtime.model_router.recipe_coverage_eval] Animus MUST frame router recipe coverage evaluation as a ResearchSpec-style study with a recorded baseline, explicit hypothesis, run plan, and keep/promotion/archive gates, driven by the batch-trajectory-runner with a deterministic test `RouterService` stub, and the study MUST NOT mutate routing logic, recipe selection, or default behavior.

#### Scenario: Study run is reproducible without providers

r[agent_harness_runtime.model_router.recipe_coverage_eval.reproducible]
- GIVEN the checked-in corpus and deterministic test `RouterService` stub
- WHEN the study run executes
- THEN it produces a stable adoption matrix across runs
- AND no live provider, network, credential, or worker-process call is invoked.

#### Scenario: Findings do not promote by default

r[agent_harness_runtime.model_router.recipe_coverage_eval.non_promoting]
- GIVEN the study produces a finding that suggests tuning recipe thresholds
- WHEN the study completes
- THEN routing defaults are unchanged
- AND promoting the finding requires a separate change with its own Cairn gates.

### Requirement: Evaluation telemetry is safe metadata

r[agent_harness_runtime.model_router.recipe_coverage_eval.telemetry] Router selection telemetry MUST be feature-gated, off by default, and limited to safe metadata (recipe id, family, denial class, budget counters, signal bucket labels), and MUST pass `validate_receipt` leak checks with no raw prompts, provider payloads, worker outputs, or credentials.

#### Scenario: Telemetry rejects raw payloads

r[agent_harness_runtime.model_router.recipe_coverage_eval.telemetry.no_leaks]
- GIVEN a telemetry fixture attempts to embed a raw prompt, worker output, or credential
- WHEN leak validation runs
- THEN the fixture is rejected with a typed leak diagnostic
- AND no raw payload is recorded.

### Requirement: Recipe coverage evaluation verification

r[agent_harness_runtime.model_router.recipe_coverage_eval.verification] The change MUST include focused positive and negative tests for study reproducibility, adoption-matrix stability, telemetry leak rejection, non-mutation of routing state, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers recipe coverage evaluation

r[agent_harness_runtime.model_router.recipe_coverage_eval.verification.closeout]
- GIVEN maintainers intend to archive the recipe-coverage evaluation change
- WHEN closeout validation runs
- THEN evidence names focused study-run, matrix, telemetry, fail-closed, non-promoting, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
