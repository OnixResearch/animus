# Design: Wire the router into the runtime loop

## Context

The runtime loop drives typed Steel/fabric decisions through injected model and tool adapters and emits ordered runtime events. Model dispatch already passes through a pure `plan_model_dispatch` admission gate. The router owns pure selection, materialization, aggregation, and receipt assembly but is never called. The missing piece is a runtime stage that sequences: signal projection → recipe selection → plan materialization → per-model `ModelDispatchPlan` admission → host-owned step execution → result aggregation → receipt emission.

## Decisions

### 1. The router is a pre-dispatch stage, not a parallel path

**Choice:** Add a runtime stage invoked before model dispatch that projects signals, selects a recipe, and materializes an execution plan. The stage returns either an admitted `RouterExecutionPlan` to drive or a typed fail-closed outcome. The existing single-model path becomes the `Single` route family's execution.

**Rationale:** One admission spine (router plan → `ModelDispatchPlan` → adapter) keeps fail-closed behavior uniform and avoids a second, less-audited dispatch path.

### 2. RouterService is an injected adapter contract

**Choice:** Define `trait RouterService` in `animus-adapters` with methods to execute role steps, collect quorum, run synthesis/repair, and resolve fallback — all returning typed outcomes. The runtime calls the injected `RouterService`; if none is injected, the runtime falls back to the compatibility single-model path. The runtime never constructs provider clients.

**Rationale:** Matches the accepted `clankers_injected_services.contracts` and `provider_boundary` requirements: reusable runtime code depends on injected contracts, not concrete clients.

### 3. SelectedPlan feeds ModelDispatchPlan admission per model

**Choice:** For each selected model label in the plan, the runtime builds a `ModelDispatchPlanRequest` using the recipe's required capability, the model's catalog entry, and the dispatch policy, and calls `plan_model_dispatch`. A denied `ModelDispatchPlan` for any required role maps to a typed fail-closed outcome and halts the route.

**Rationale:** Reuses the existing pure admission gate rather than duplicating capability/budget/stream/receipt checks. The router decides *what* to run; `ModelDispatchPlan` decides *whether* each call may proceed.

### 4. Compatibility default preserves the single-model path

**Choice:** When no `RouterService` is injected, the runtime uses `ModelDispatchPolicyDraft::compatibility_defaults()` and the `Single` route family so existing single-model behavior is unchanged. An equivalence test asserts the compatibility path matches the prior runtime behavior.

**Rationale:** Connects to the accepted `model_dispatch.compatibility` requirement and avoids forcing every embedder to wire a router.

## Risks / Trade-offs

- Two admission gates (router selection + `ModelDispatchPlan`) can double-deny confusingly. Mitigate by giving router denials and dispatch denials distinct typed reasons and asserting they surface separately in the runtime event stream.
- The `RouterService` trait can grow large. Keep the first version to the minimum methods needed for Single + one collaborative family; expand later.
- Runtime tests risk depending on a host. Use a test `RouterService` stub that returns deterministic typed outcomes, keeping the runtime testable without providers.
