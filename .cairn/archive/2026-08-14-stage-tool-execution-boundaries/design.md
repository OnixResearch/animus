# Design: Stage tool-execution boundaries

## Context

`tool_executor_kit` is the authoritative final tool gate and bounded execution wrapper. The coding-session host owns durable call and result ordering. Lifecycle hooks cover turns, not each tool boundary.

A staged contract can support host-specific review, redaction, wrappers, and observation only if every stage remains subordinate to these owners.

## Success Contract

One immutable stage plan identifies every selected handler and phase. Each handler receives only its phase-specific typed input. Every candidate returns to Rust validation. Denial remains sticky. Provider continuation occurs only after the exact final result commits durably.

## Decisions

### 1. Define a fixed phase order

The phase order is:

```text
pre_admission
  -> final_admission
  -> pre_execution
  -> execution
  -> output_narrowing
  -> final_validation
  -> durable_commit
  -> post_commit_observation
```

Profiles can select zero or more declared handlers inside an extensible phase. They cannot add phases, move final admission, move final validation, or move durable commit.

### 2. Bind one immutable stage plan

The plan identity binds ordered handler identities, handler cohorts, tool and schema identity, exact request identity, executor binding, capability and authority identities, mutation posture, output policy, cancellation and deadline posture, and required commit owner.

Missing, duplicate, stale, substituted, or undeclared handlers fail before execution.

### 3. Preserve the existing final admission owner

`plan_tool_execution` remains the final admission gate. Pre-admission handlers can add safe diagnostics, narrow a request posture, or deny. They cannot create a catalog entry, capability, approval, executor, mutation target, or fallback.

Pre-execution handlers can request shell-owned wrappers or deny. The host revalidates each wrapper against the admitted effect scope.

### 4. Make monotonic transitions structural

A stage candidate cannot:

- reverse a denial;
- add a capability or authority source;
- change tool, schema, arguments, correlation, executor, or target;
- widen a mutation posture;
- increase byte, character, line, time, or other bounds;
- turn cancellation, timeout, failure, or truncation into success;
- remove a required redaction or commit barrier.

The reducer returns typed widening or substitution issues.

### 5. Limit output changes to narrowing

Output-narrowing handlers can redact, truncate, classify, or replace unsafe visible text with a bounded safe marker. They preserve the original execution outcome class and bind both input and narrowed output identities.

Final validation remains in the executor kit. Rejected output cannot produce completed-effect evidence.

### 6. Keep durable ordering with the coding session

The stage shell returns one final validated result to the coding-session host. The host commits the exact correlated tool result. A matching durable observation advances the stage plan to post-commit observation and provider continuation.

Post-commit handlers receive metadata only. They cannot edit the committed result, execute another tool, call a provider, or change completion state.

### 7. Keep handlers host-injected and bounded

Reusable code defines handler ports and validates their outputs. Concrete handlers remain selected at visible composition roots. There is no plugin discovery, dynamic code loading, global middleware list, or handler-owned authority.

## Alternatives Considered

### Treat arbitrary lifecycle hooks as tool stages

Rejected because turn hooks do not bind exact tool request, executor, output, and commit identities.

### Permit handlers to rewrite tool arguments

Rejected because a rewritten request is a new request that must pass model/tool admission and durable ordering from the start.

### Observe before durable commit

Rejected because observers could report a result that the session did not accept.

## Risks and Controls

- Many handlers can obscure behavior. Generated activation and stage-plan inspection must show exact ordered identities.
- Redaction can alter model behavior. Receipts record narrowing identity and class without retaining raw output.
- A host handler can lie. Rust revalidation checks supplied candidates but does not prove handler implementation correctness.

## Non-Claims

This design does not prove tool correctness, effect rollback, handler honesty, sandboxing, task success, provider continuation correctness, production readiness, or release eligibility.
