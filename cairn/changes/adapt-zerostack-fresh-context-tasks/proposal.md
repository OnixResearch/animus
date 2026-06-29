# Proposal: Adapt Zerostack fresh-context task agents for Animus

## Why

Zerostack exposes a `task` tool that runs a fresh-context subagent for cross-file investigation and a `/btw` side-question path that can answer read-only questions without interrupting the main run. Animus already has future subagent/thread and host-control surfaces; this Cairn captures the fresh-context task and read-only side-question contracts worth adapting.

The goal is bounded parallel investigation with clear permissions, resource limits, and result attribution, not process-global subagent state or uncontrolled concurrent tool use.

## Scope

- Define fresh-context task request descriptors for investigation goals, admitted context, allowed tools, budgets, and expected result shape.
- Define read-only side-question requests that cannot mutate state or enter the main ledger unless admitted by policy.
- Define task result attribution with child-run identity, source context, tool budget usage, uncertainty markers, and redacted evidence.
- Define host-owned scheduling and cancellation semantics without process-global mutable configuration.
- Add positive and negative tests for read-only enforcement, budget exhaustion, cancelled tasks, malformed results, and unsafe result injection.

## Impact

- **Files**: subagent/task DTOs, runtime planning core, host scheduling interfaces, receipts, tests, and docs.
- **Testing**: positive fresh-context investigation and side-question tests; negative mutation attempt, missing scheduler, budget overflow, cancelled task, and raw-output leak tests.
- **Non-claims**: this does not copy Zerostack's subagent implementation, use global statics, require a specific provider, or make side-question output durable by default.
