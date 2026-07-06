# Design: Define router execution plans

## Context

`SelectedPlan` is a flat record: recipe id, family, selected model labels, output contract, bounds. Executing a collaborative recipe needs structure: which roles run first, which can fan out in parallel, how many successes are required before synthesis, how many repair rounds remain, and which fallback model steps in if quorum fails. Today that structure lives nowhere, so any runtime consumer would reinvent it inconsistently.

## Decisions

### 1. Execution plan is a pure derivation of SelectedPlan + catalog + budget

**Choice:** Add `materialize_execution_plan(selected: &SelectedPlan, catalog: &[CatalogEntry], budget: &Budget) -> Result<RouterExecutionPlan, ExecutionPlanError>` as a pure function. The plan is fully determined by its inputs; no I/O, no clocks, no provider calls.

**Rationale:** Keeps the routing decision testable without providers and makes plan materialization the second pure gate after recipe selection.

### 2. Steps are ordered role groups with parallelism caps

**Choice:** The plan is an ordered `Vec<RoleStep>` where each step carries a role, the candidate model labels assigned to it, a parallelism cap drawn from `Bounds::max_parallelism`, and a quorum threshold drawn from `Bounds::min_success_quorum` scoped to that step's candidates.

**Rationale:** Order encodes escalation/synthesis/finalization sequencing; parallelism caps prevent unbounded fan-out; per-step quorum lets Ratings and ReMoM require enough successes before the next stage.

### 3. Budgets are tracked as remaining counters, not re-derived

**Choice:** The plan carries `remaining_synthesis_attempts` and `remaining_repair_attempts` counters initialized from `Bounds`, plus a `FallbackChain` ordered list. Execution consumers decrement counters; re-materialization is idempotent from the original inputs.

**Rationale:** Collaborative routes need to know whether another synthesis or repair attempt is permitted without recomputing policy each time. Counters make the budget decision a pure decrement check.

### 4. Fallback chain is acyclic and eligibility-checked

**Choice:** The fallback chain is an ordered list of `(role, model_label)` pairs derived from catalog entries whose role is `Fallback`, with an acyclicity check and an eligibility check against `Budget::allowed_backends` and `Signals::privacy`.

**Rationale:** Fallback can widen authority and hide failures; requiring an explicit, bounded, acyclic chain prevents silent rerouting and connects to the later budget/fallback gating cairn.

## Risks / Trade-offs

- Per-step quorum could over-constrain simple Single routes. Mitigate by defaulting Single steps to a quorum of one and proving Single plans materialize trivially.
- Counters-as-state can invite mutation. Keep the plan immutable; consumers return a new plan with decremented counters from a pure helper, matching the functional-core discipline.
- Fallback eligibility overlaps `negotiate-router-capability-eligibility`. Keep this change to backend/privacy eligibility already available on `CatalogEntry` and `Signals`; richer capability matching is deferred to that cairn.
