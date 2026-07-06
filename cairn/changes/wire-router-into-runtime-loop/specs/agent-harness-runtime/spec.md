# agent-harness-runtime Change Spec: Wire the router into the runtime loop

## Requirements

### Requirement: Router is a pre-dispatch runtime stage

r[agent_harness_runtime.model_router.runtime_gate] Animus runtime dispatch MUST run a router stage before model dispatch that projects signals, selects a recipe, and materializes an execution plan, mapping denied selection to a typed fail-closed outcome with no adapter call, and admitted plans MUST drive each selected model through the existing `ModelDispatchPlan` admission gate.

#### Scenario: Denied recipe selection stops dispatch

r[agent_harness_runtime.model_router.runtime_gate.denied_selection]
- GIVEN projected signals and budget admit no recipe or no eligible model
- WHEN the router stage runs
- THEN the runtime returns a typed fail-closed outcome
- AND no model adapter, provider client, network, or credential operation is invoked.

#### Scenario: Single route reuses the model-dispatch path

r[agent_harness_runtime.model_router.runtime_gate.single_path]
- GIVEN a Single route family is selected with one primary model
- WHEN the runtime stage runs
- THEN the selected model is admitted through `plan_model_dispatch` with compatibility defaults
- AND the existing single-model behavior is preserved.

### Requirement: RouterService is an injected contract

r[agent_harness_runtime.model_router.runtime_gate.service] The runtime MUST consume an injected `RouterService` adapter contract for step execution, quorum collection, synthesis/repair, and fallback, and MUST NOT construct provider clients, read credentials, open network connections, or call models directly from reusable runtime code.

#### Scenario: Missing RouterService falls back compatibly

r[agent_harness_runtime.model_router.runtime_gate.service.missing]
- GIVEN no `RouterService` is injected
- WHEN the runtime needs to route a request
- THEN it uses the compatibility Single path with `ModelDispatchPolicyDraft::compatibility_defaults()`
- AND it does not attempt provider, network, credential, or worker-process fallback.

#### Scenario: Denied ModelDispatchPlan halts the route

r[agent_harness_runtime.model_router.runtime_gate.service.denied_plan]
- GIVEN an admitted router plan selects a model whose `ModelDispatchPlan` is denied
- WHEN admission runs for that model
- THEN the runtime returns a typed fail-closed outcome
- AND no later role step, synthesis, or fallback for that route is executed.

### Requirement: Router runtime wiring verification

r[agent_harness_runtime.model_router.runtime_gate.verification] The change MUST include focused positive and negative runtime tests for Single and collaborative routes, denied selection, missing service, denied `ModelDispatchPlan`, fail-closed aggregation, compatibility equivalence, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers runtime wiring

r[agent_harness_runtime.model_router.runtime_gate.verification.closeout]
- GIVEN maintainers intend to archive the runtime-wiring change
- WHEN closeout validation runs
- THEN evidence names focused stage, admission, compatibility, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
