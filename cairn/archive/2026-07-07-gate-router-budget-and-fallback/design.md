# Design: Gate router budget and fallback

## Context

`Bounds` and `FallbackPolicy` are validated at recipe construction, but a route executes over multiple steps whose failures must be resolved against remaining budget. Today there is no pure value tracking "how many synthesis attempts are left" or "is this fallback model still eligible and unfunded." This change adds the pure enforcement layer that sits between step execution and the next decision, keeping `Bounds` authoritative rather than advisory.

## Decisions

### 1. BudgetLedger is an immutable, decremented pure value

**Choice:** `BudgetLedger::from(bounds)` produces an initial ledger. Each consumption (a candidate dispatched, a parallelism slot used, a synthesis or repair attempt) returns a new `BudgetLedger` with decremented counters via pure helpers. Exhaustion is reported as `BudgetExhausted { kind, remaining }`.

**Rationale:** Immutable decremented values keep budget tracking pure and testable, matching the functional-core discipline. No mutation, no shared state.

### 2. Fallback evaluation is a pure decision over failure + ledger + chain + policy

**Choice:** `evaluate_fallback(failure, ledger, chain, policy)` returns `Deny`, `Repair(attempt)`, `FallbackTo(model)`, or `FailRoute(reason)`. The decision is fully determined by its inputs: a `Repair` is allowed only if `remaining_repair_attempts > 0`; `FallbackTo` is allowed only if the chain has an unconsumed, eligible model and remaining candidate/parallelism budget; otherwise `Deny` or `FailRoute`.

**Rationale:** Centralizes the deny-vs-fallback-vs-repair decision so the runtime never improvises. Connects to the accepted `vllm_semantic_router_recipes.bounds` and `model_dispatch` no-fallback posture.

### 3. Fallback chain is consumed in order with no revisits

**Choice:** The ledger records which model labels have already been tried in the route. `evaluate_fallback` only returns `FallbackTo` for the next unconsumed chain entry, never a previously tried model, and checks backend/privacy eligibility via the catalog.

**Rationale:** Prevents cyclic retry and silent rerouting to an already-failed model. Acyclicity was checked at materialization; consumption tracking enforces it at runtime.

### 4. FallbackPolicy defaults to deny

**Choice:** The default `FallbackPolicy` denies fallback. A policy that permits fallback must explicitly name the allowed fallback role/backend and the exhaustion class that triggers it. Any fallback not explicitly policy-authorized is `Deny`.

**Rationale:** Matches the accepted `model_dispatch` deny-by-default fallback posture and prevents fallback from widening authority without policy.

## Risks / Trade-offs

- A ledger per route can grow with attempted models. Cap the tried-model set at `max_candidates` and fail the route if exhausted.
- Repair-vs-fallback precedence can be policy-sensitive. Default to repair-then-fallback and let policy override; document the default.
- This overlaps `negotiate-router-capability-eligibility` on eligibility. Keep this change to budget/exhaustion/chain-consumption; capability eligibility of the fallback model is delegated to that sibling cairn's `resolve_role_eligibility`.
