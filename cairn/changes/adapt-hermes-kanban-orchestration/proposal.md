# Proposal: Adapt Hermes Kanban orchestration for Animus

## Why

Hermes' Kanban worker model captures durable multi-agent orchestration patterns that are useful for long-lived agent work: task graphs, parent/child dependencies, worker heartbeats, block/complete handoffs, workspace scopes, and tenant isolation. Animus should adapt those concepts as an optional orchestration adapter and DTO set while keeping core agent turns independent from any specific board database or dispatcher.

## What Changes

- Define mode-neutral task graph DTOs for tasks, dependencies, lifecycle state, worker assignment, workspace scope, tenant label, comments, heartbeats, block reasons, and completion metadata.
- Add deterministic dependency promotion rules for fan-out/fan-in workflows.
- Add workspace isolation contracts for scratch, directory, and worktree-style execution contexts.
- Treat Kanban dispatch as optional host orchestration, not a required runtime loop dependency.
- Add positive and negative fixtures for task lifecycle, dependency ordering, workspace validation, tenant scoping, and handoff metadata.

## Impact

- **Files**: future orchestration DTOs, adapter traits, task graph validation, workspace scope validation, lifecycle event tests, and docs.
- **Testing**: valid graph promotion plus cycle, missing parent, invalid workspace, cross-tenant leakage, stale worker completion, malformed metadata, and unauthorized workspace mutation rejection.
- **Non-claims**: this change does not adopt Hermes' SQLite board, dispatcher, profile roster, CLI, or automatic process spawning in reusable runtime crates.
