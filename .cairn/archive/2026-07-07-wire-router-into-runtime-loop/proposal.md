# Proposal: Wire the router into the runtime loop

## Why

The router module and the runtime loop both exist, but they never meet. `native_grep` for `router|select_recipe|SelectedPlan|RouteResult|Recipe` across `crates/animus-runtime/src` returns zero matches: the runtime never projects signals, never selects a recipe, never materializes a plan, and never aggregates route results. Meanwhile `admission.rs` already has a pure `plan_model_dispatch` gate that admits or denies a single model call before the adapter runs. The router should sit *before* that gate: project signals → select recipe → materialize execution plan → feed the selected model(s) to `ModelDispatchPlan` → aggregate results → emit receipt. Without this wiring, the router is dead code and the runtime cannot serve collaborative routes.

## What Changes

- Add a runtime router stage that, before model dispatch, calls the injected `SignalSource` to project signals, runs `select_recipe`, and materializes a `RouterExecutionPlan`; denied selection maps to a typed fail-closed outcome with no adapter call.
- Connect the router's `SelectedPlan` to the existing `ModelDispatchPlan` admission gate so the runtime admits each selected model through the same fail-closed path used for single-model dispatch.
- Add an injected `RouterService` adapter contract (implementation-free) in `animus-adapters` so the runtime never constructs provider clients or calls models directly; the host owns step execution, parallelism, quorum collection, and fallback.
- Add runtime tests proving: a Single route reuses the existing model-dispatch path; a denied recipe selection produces a typed fail-closed outcome with no adapter call; an admitted plan drives `ModelDispatchPlan` admission per selected model.
- Keep the existing simple model-adapter behavior available through compatibility defaults when no router service is injected.

## Impact

- **Files**: `crates/animus-runtime` router stage, `crates/animus-adapters` `RouterService` trait, runtime shell dispatch, runtime tests, runtime composition docs, facade inventory, and Cairn evidence.
- **Testing**: positive runtime tests for Single and one collaborative route; negative tests for denied selection, missing router service, denied `ModelDispatchPlan`, and fail-closed aggregation.
- **Non-claims**: this does not add live provider clients, Steel bindings for routing, credential lookup, network transport, or receipt persistence; the host owns step execution.
