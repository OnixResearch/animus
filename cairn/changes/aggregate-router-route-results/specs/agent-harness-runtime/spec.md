# agent-harness-runtime Change Spec: Aggregate router route results

## Requirements

### Requirement: Pure route-result aggregation

r[agent_harness_runtime.model_router.route_aggregation] Animus MUST provide a pure deterministic `aggregate_route_results` function that folds collected role outputs for a materialized execution plan into a single `RouteResult`, dispatching to family-specific aggregators for ratings, ReMoM, fusion, workflow, and single routes, and revalidating the final output against the recipe's `OutputContract` before reporting success.

#### Scenario: ReMoM quorum synthesizes one result

r[agent_harness_runtime.model_router.route_aggregation.remom_quorum]
- GIVEN a ReMoM execution plan collected enough worker outputs to meet quorum
- WHEN aggregation runs
- THEN the synthesizer assembles a synthesis input from bounded worker summaries
- AND the final output is revalidated against the recipe's `OutputContract`
- AND exactly one `RouteResult` is returned without calling a provider.

#### Scenario: Single route passes through

r[agent_harness_runtime.model_router.route_aggregation.single_passthrough]
- GIVEN a Single execution plan with one primary role output
- WHEN aggregation runs
- THEN the output is revalidated against the `OutputContract` and returned as the single `RouteResult`
- AND no quorum, judge, or finalizer step runs.

### Requirement: Aggregation failures fail closed

r[agent_harness_runtime.model_router.route_aggregation.fail_closed] Aggregation MUST return a typed `AggregationError` when quorum is insufficient, a role output is malformed, the judge deadlocks, the finalizer is missing, the final output violates the `OutputContract`, or a raw provider payload is supplied, and MUST NOT claim success in those cases.

#### Scenario: Insufficient quorum denied

r[agent_harness_runtime.model_router.route_aggregation.fail_closed.quorum]
- GIVEN a Ratings or ReMoM plan collected fewer successes than its quorum threshold
- WHEN aggregation runs
- THEN it returns an `InsufficientQuorum` error
- AND no `RouteResult` is produced.

#### Scenario: Judge deadlock denied

r[agent_harness_runtime.model_router.route_aggregation.fail_closed.judge_deadlock]
- GIVEN a Fusion plan's judge returns no binding verdict
- WHEN aggregation runs
- THEN it returns a `JudgeDeadlock` error
- AND no arbitrary worker output is selected as the final result.

### Requirement: Route-result aggregation verification

r[agent_harness_runtime.model_router.route_aggregation.verification] The change MUST include focused positive and negative tests for each family's aggregation, quorum satisfaction, judge verdict binding, finalizer resolution, contract revalidation, raw-payload rejection, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers route aggregation

r[agent_harness_runtime.model_router.route_aggregation.verification.closeout]
- GIVEN maintainers intend to archive the route-aggregation change
- WHEN closeout validation runs
- THEN evidence names focused family-aggregation, fail-closed, contract-revalidation, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
