# Proposal: Aggregate router route results

## Why

`RouteResult` and `RouteResultStatus` exist as DTOs, and collaborative recipes (Ratings, ReMoM, Fusion, Workflow) fan out to multiple roles. But nothing aggregates role outputs into one answer: ratings rollup, quorum satisfaction checks, synthesis-input assembly, judge verdict resolution, and finalizer selection are all unowned. Without aggregation, a collaborative route cannot claim it "produced one normal response" — the runtime would have to glue worker outputs together ad hoc, which is exactly the freeform multi-agent behavior the vLLM adaptation rejected.

## What Changes

- Add a pure `aggregate_route_results(plan: &RouterExecutionPlan, role_outputs: &[RoleOutput]) -> Result<RouteResult, AggregationError>` that resolves a collaborative route to a single `RouteResult`.
- Add family-specific aggregation: ratings mean/median rollup with quorum check, ReMoM quorum satisfaction and synthesis-input assembly, Fusion judge verdict binding and finalizer selection, Workflow step chaining, and Single passthrough.
- Add typed `AggregationError` for insufficient quorum, malformed role output, judge deadlock, finalizer-missing, and output-contract mismatch.
- Require the final aggregated output to be revalidated against the recipe's `OutputContract` before success.
- Add positive tests for each family's happy path and negative tests for each aggregation failure.

## Impact

- **Files**: `crates/animus-router` aggregation module, `RoleOutput` DTO, `RouteResult` accessors, router tests, facade inventory, and Cairn evidence.
- **Testing**: positive aggregation fixtures per family; negative tests for quorum failure, malformed output, judge deadlock, finalizer missing, and contract mismatch.
- **Non-claims**: this does not call providers, execute the plan steps, emit receipts, or wire the runtime; aggregation is pure over already-collected role outputs.
