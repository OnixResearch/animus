# Proposal: Define router execution plans

## Why

`select_recipe` already returns a `SelectedPlan` naming a recipe, route family, selected model labels, output contract, and bounds. But nothing turns that plan into a *driveable* execution shape: per-role dispatch order, fan-out/parallelism grouping, quorum threshold against the candidate count, remaining synthesis and repair budgets, and an acyclic fallback chain. Without a materialized execution plan, collaborative recipes (Ratings, ReMoM, Fusion, Workflow) cannot be executed deterministically — the runtime would have to improvise ordering and budget tracking ad hoc.

## What Changes

- Add a pure `RouterExecutionPlan` derived from `SelectedPlan` plus catalog and budget inputs, encoding ordered role steps, parallelism groups, quorum threshold, remaining synthesis/repair attempt counters, and an ordered fallback chain.
- Add invariant checks: quorum ≤ candidate count, repair and synthesis budgets bounded and non-zero where required, fallback chain acyclic and policy-consistent, every recipe role mapped to exactly one step.
- Add typed `ExecutionPlanError` denials for impossible quorum, unbounded repair, zero quorum, missing role mapping, cyclic fallback, and fallback-model-not-eligible.
- Add positive and negative tests for each recipe family plan and each invariant violation.

## Impact

- **Files**: `crates/animus-router` execution-plan module, recipe/bounds accessors as needed, router tests, facade inventory, and Cairn evidence.
- **Testing**: positive plan fixtures for Single, Confidence, Ratings, ReMoM, Fusion, and Workflow families; negative tests for impossible quorum, unbounded repair, missing role, cyclic fallback, and fallback-eligibility denial.
- **Non-claims**: this does not execute the plan, call providers, assemble worker inputs, emit receipts, or wire the runtime loop; those belong to later cairns.
