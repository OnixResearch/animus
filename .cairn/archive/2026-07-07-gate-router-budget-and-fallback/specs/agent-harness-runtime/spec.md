# agent-harness-runtime Change Spec: Gate router budget and fallback

## Requirements

### Requirement: Pure router budget tracking

r[agent_harness_runtime.model_router.budget_fallback] Animus MUST track router budget as an immutable `BudgetLedger` initialized from `Bounds` with pure decrement helpers for candidate, parallelism, synthesis, and repair consumption, and MUST report exhaustion as typed `BudgetExhausted` denials rather than allowing caps to be exceeded.

#### Scenario: Repair budget exhaustion stops retry

r[agent_harness_runtime.model_router.budget_fallback.repair_exhausted]
- GIVEN a route has consumed all `max_repair_attempts`
- WHEN a step fails and `evaluate_fallback` runs
- THEN it returns `Deny` or `FailRoute`, never `Repair`
- AND no further repair attempt is permitted.

### Requirement: Fallback evaluation is pure and deny-by-default

r[agent_harness_runtime.model_router.budget_fallback.evaluation] Fallback MUST be resolved by a pure `evaluate_fallback` function over a step failure, ledger, fallback chain, and `FallbackPolicy` that returns `Deny`, `Repair(attempt)`, `FallbackTo(model)`, or `FailRoute(reason)`, and the default `FallbackPolicy` MUST deny fallback unless policy explicitly authorizes the allowed fallback role/backend and exhaustion class.

#### Scenario: Unauthorized fallback denied

r[agent_harness_runtime.model_router.budget_fallback.evaluation.unauthorized]
- GIVEN a step fails and the `FallbackPolicy` does not name an allowed fallback for its class
- WHEN `evaluate_fallback` runs
- THEN it returns `Deny`
- AND no rerouting to another model occurs.

### Requirement: Fallback chain is acyclic and budget-bound

r[agent_harness_runtime.model_router.budget_fallback.chain] Fallback MUST consume the chain in order, MUST NOT revisit a model already tried in the route, MUST stay within remaining candidate and parallelism budget, and MUST check backend and privacy eligibility before authorizing `FallbackTo`.

#### Scenario: Previously tried model not revisited

r[agent_harness_runtime.model_router.budget_fallback.chain.no_revisit]
- GIVEN a fallback model has already been tried and failed in the route
- WHEN `evaluate_fallback` runs for a later failure
- THEN it does not return `FallbackTo` for that model
- AND if no unconsumed eligible entry remains, it returns `FailRoute`.

### Requirement: Budget and fallback verification

r[agent_harness_runtime.model_router.budget_fallback.verification] The change MUST include focused positive and negative tests for budget tracking, authorized repair, authorized fallback, fail-route, exhaustion, policy deny, cyclic fallback, ineligible fallback, cap-exceeding retry, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers budget and fallback

r[agent_harness_runtime.model_router.budget_fallback.verification.closeout]
- GIVEN maintainers intend to archive the budget-and-fallback change
- WHEN closeout validation runs
- THEN evidence names focused ledger, evaluation, chain, default-deny, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
