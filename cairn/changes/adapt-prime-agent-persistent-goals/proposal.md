## Why

Prime Agent's persistent goals keep one objective active across turns until it is completed, paused, budget-limited, errored, or cleared. Goal state records token usage, elapsed time, continuation count, and an optional explicit token budget. Only an explicit completion marks success. Animus has `ralph-loop` completion promises and `controller-continuation-policy`, but no durable goal object: neither carries cross-turn accounting, and neither distinguishes "the loop stopped" from "the objective is done".

Without a goal object, long autonomous work conflates stopping conditions with success. Animus needs a bounded goal contract: durable state, explicit lifecycle, accounting, and the rule that exhaustion is an outcome, not a completion.

## What Changes

- Add a durable goal object with objective, status, creation provenance, token and elapsed accounting, continuation count, and optional explicit budget. r[agent_harness_runtime.persistent_goals.state]
- Add the continuation rule: an active goal keeps presenting the objective after ordinary assistant turns, and only explicit completion marks success. r[agent_harness_runtime.persistent_goals.continuation]
- Add budget enforcement: exhaustion of token, time, or continuation budgets stops continuation with a typed budget outcome that is never reported as success. r[agent_harness_runtime.persistent_goals.budget]
- Add ownership rules: goal creation, pause, resume, and clear are explicit user or host actions; the agent never infers a goal from a task. r[agent_harness_runtime.persistent_goals.ownership]
- Add a Nickel-authored goal policy for budgets, accounting fields, lifecycle transitions, and non-claims. r[agent_harness_runtime.persistent_goals.profile]
- Add metadata-only goal receipts for lifecycle and accounting events. r[agent_harness_runtime.persistent_goals.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.persistent_goals.verification]

## Impact

- **Files**: new focused goal contract module in the runtime family, Nickel policy, fixtures, focused check script, docs; composition seams in `controller-continuation-policy` and `ralph-loop`.
- **Reusable boundary**: goal state is pure data; clocks, token counters, and persistence stay host-injected.
- **Runtime composition**: `controller-continuation-policy` consumes goal state when deciding post-turn continuation; ralph-loop profiles may bind a goal but keep their own completion-promise semantics.
- **Testing**: positive fixtures for lifecycle, accounting, and explicit completion; negative fixtures for inferred goals, budget-as-success confusion, and invalid transitions; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Inferring goals from ordinary tasks, or auto-creating goals without an explicit action.
- Treating budget exhaustion, compaction, or session detach as completion.
- Scheduling or heartbeats, which the schedule-reliability change owns.
- Proving objective achievement, work quality, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: persistent-goal boundary, policy, state, continuation, budget, ownership, evidence, and verification.
