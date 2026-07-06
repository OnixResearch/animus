# agent-harness-runtime Change Spec: Define router execution plans

## Requirements

### Requirement: Materialized router execution plans

r[agent_harness_runtime.model_router.execution_plan] A `RouterExecutionPlan` MUST be derived as a pure deterministic function of a `SelectedPlan`, model catalog, and budget, encoding ordered role steps, per-step candidate labels, parallelism caps, per-step quorum thresholds, remaining synthesis and repair attempt counters, and an ordered fallback chain, before any provider or host execution begins.

#### Scenario: Collaborative recipe materializes ordered steps

r[agent_harness_runtime.model_router.execution_plan.ordered_steps]
- GIVEN a `SelectedPlan` for a ReMoM or Fusion recipe with worker, synthesizer, judge, and finalizer roles
- WHEN execution-plan materialization runs
- THEN the plan emits ordered role steps with worker steps before synthesizer, synthesizer before judge, and judge before finalizer
- AND each step records its candidate labels, parallelism cap, and quorum threshold
- AND no provider request is sent during materialization.

#### Scenario: Single recipe materializes trivially

r[agent_harness_runtime.model_router.execution_plan.single]
- GIVEN a `SelectedPlan` for a Single route family with one primary model
- WHEN materialization runs
- THEN the plan has one role step with a quorum of one and no fallback chain
- AND synthesis and repair counters are zero.

### Requirement: Execution plan invariants fail closed

r[agent_harness_runtime.model_router.execution_plan.invariants] Plan materialization MUST return a typed denial when quorum exceeds candidate count, quorum is zero, repair is unbounded, a recipe role has no mapped step, the fallback chain is cyclic, or a fallback model is not eligible under backend or privacy constraints, and MUST NOT return a plan in those cases.

#### Scenario: Impossible quorum denied

r[agent_harness_runtime.model_router.execution_plan.invariants.impossible_quorum]
- GIVEN a selected plan requires a min-success quorum greater than its candidate count
- WHEN materialization runs
- THEN it returns an impossible-quorum denial
- AND no execution plan is produced.

#### Scenario: Cyclic fallback denied

r[agent_harness_runtime.model_router.execution_plan.invariants.cyclic_fallback]
- GIVEN a fallback chain references the same model label more than once in a cycle
- WHEN materialization runs
- THEN it returns a cyclic-fallback denial
- AND no execution plan is produced.

### Requirement: Execution plan verification

r[agent_harness_runtime.model_router.execution_plan.verification] The change MUST include focused positive and negative tests for each recipe family's plan, invariant denials, budget counters, fallback ordering, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers execution plans

r[agent_harness_runtime.model_router.execution_plan.verification.closeout]
- GIVEN maintainers intend to archive the execution-plan change
- WHEN closeout validation runs
- THEN evidence names focused materialization, invariant, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
