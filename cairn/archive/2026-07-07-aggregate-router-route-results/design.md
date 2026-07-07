# Design: Aggregate router route results

## Context

A collaborative recipe runs several roles and must return one response. The pieces exist in isolation — `RouteResult`, `RouteResultStatus`, `OutputContract`, `Role` — but the logic that folds N role outputs into one validated answer is absent. This is the core that makes Ratings/ReMoM/Fusion actually collaborative rather than a bag of independent calls.

## Decisions

### 1. Aggregation is pure over collected role outputs

**Choice:** `aggregate_route_results(plan, role_outputs)` is a pure function. It takes the materialized execution plan and the collected role outputs and returns a `RouteResult` or typed `AggregationError`. No provider calls, no I/O, no clocks.

**Rationale:** Aggregation is the third pure gate (after selection and materialization). Keeping it pure makes collaborative routes testable without standing up models.

### 2. Family-specific aggregators share a common shape

**Choice:** Each route family has a dedicated pure aggregator: `aggregate_ratings`, `aggregate_remom`, `aggregate_fusion`, `aggregate_workflow`, and `passthrough_single`. Each consumes the plan's role steps and the matching role outputs, checks quorum, assembles synthesis input, resolves the judge/finalizer, and revalidates against `OutputContract`.

**Rationale:** Families differ enough (quorum vs. judge verdict vs. step chaining) that one generic loop would obscure control flow. Named aggregators keep each family's logic explicit and individually testable, matching the tiger-style explicit-control-flow principle.

### 3. Judge verdict is binding; finalizer resolves disagreement

**Choice:** In Fusion, the judge's verdict binds which worker output is forwarded to the finalizer; if the judge deadlocks or returns no verdict, aggregation fails with `JudgeDeadlock` rather than picking arbitrarily. The finalizer then shapes the final output to the recipe's `OutputContract`.

**Rationale:** Silent arbitrary selection would hide disagreement — the opposite of safe collaboration. Deadlock is a typed failure the caller can repair or fall back from.

### 4. Output contract is revalidated before success

**Choice:** Before returning a successful `RouteResult`, aggregation revalidates the final output against the recipe's `OutputContract` (text, json, tool_call, artifact). Mismatch returns `AggregationError::ContractMismatch`.

**Rationale:** Connects to the accepted `vllm_semantic_router_recipes.output_contract` requirement: collaborative routes must validate the final response contract before reporting success.

## Risks / Trade-offs

- Synthesis-input assembly could embed raw worker outputs. Mitigate by having aggregation consume bounded `RoleOutput` summaries (text or hash, never raw provider payloads) and reject oversized inputs.
- Judge verdict logic can grow complex. Keep the first version to "judge selects one worker output by label"; richer scoring is deferred.
- Aggregation overlapping with execution is a risk. Keep this change to the pure fold over *already-collected* outputs; the runtime wiring cairn owns step execution and collection ordering.
