# Proposal: Gate router budget and fallback

## Why

`Bounds`, `FallbackPolicy`, `Budget`, and the `limit.rs` module define hard caps (max candidates, parallelism, quorum, synthesis attempts, repair attempts, timeout class) and fallback policy labels, but the *execution semantics* are unenforced: nothing tracks budget exhaustion across a route's lifetime, nothing decides deny-vs-fallback-vs-repair when a step fails, nothing enforces that the fallback chain is consumed acyclically and within remaining budget, and nothing prevents a route from exceeding its synthesis or repair caps through retry. Without enforcement, `Bounds` are advisory and fallback can widen authority silently — exactly the failure the accepted `vllm_semantic_router_recipes.bounds` requirement forbids.

## What Changes

- Add a pure `BudgetLedger` initialized from `Bounds` that tracks remaining candidate, parallelism, synthesis, and repair budget as immutable decremented copies, and reports exhaustion as typed `BudgetExhausted` denials.
- Add a pure `evaluate_fallback(failure: &StepFailure, ledger: &BudgetLedger, chain: &FallbackChain, policy: &FallbackPolicy) -> FallbackDecision` returning `Deny`, `Repair(attempt)`, `FallbackTo(model)`, or `FailRoute(reason)`.
- Enforce that fallback consumes the chain in order, stays within remaining budget, respects privacy/backend eligibility, and never revisits a model already tried in the route.
- Add positive tests for budget tracking, authorized repair, authorized fallback, and fail-route; negative tests for budget exhaustion, fallback policy deny, cyclic fallback, ineligible fallback model, and cap-exceeding retry.

## Impact

- **Files**: `crates/animus-router` budget/fallback module reusing `limit.rs`, router tests, facade inventory, and Cairn evidence.
- **Testing**: positive ledger/fallback fixtures; negative tests for each exhaustion and denial path.
- **Non-claims**: this does not execute steps, call providers, persist ledger state, or wire the runtime; the ledger is a pure value the runtime consults between steps.
