# Proposal: Adapt Clankers checkpoint rollback UX for Animus

## Why

Clankers pairs mutating tools with local checkpoints and an explicit rollback workflow. Animus already plans VCS-neutral provenance and mutating-tool observations, but it also needs the operator-facing rollback contract: create/list checkpoints, require confirmation before restore, verify current state before rollback, and emit safe receipts.

Animus should adopt the UX and receipt pattern without adopting Clankers' git-only checkpoint store. Backends such as jj, git, in-memory fixtures, or future workspace-isolation adapters belong behind injected version-control or checkpoint services.

## What Changes

- Define VCS-neutral checkpoint and rollback DTOs for checkpoint identity, safe repository/workspace label, before/after content identities, changed-file counts, session/tool attribution, and backend status.
- Add a checkpoint service boundary that can create, list, inspect, and rollback checkpoints without reusable runtime code invoking git, jj, shell commands, or filesystem mutation directly.
- Require explicit operator confirmation and current-state verification before rollback mutates the workspace.
- Connect mutating-tool dispatch policy to pre-mutation checkpoint requests when checkpoint policy requires protection.
- Add safe rollback receipts and negative tests for missing backend, checkpoint failure, stale current state, unsupported backend, unconfirmed rollback, and metadata leak attempts.

## Impact

- **Files**: future checkpoint DTOs, injected checkpoint service traits, mutating-tool observer hooks, confirmation integration, tests, docs, and Cairn artifacts.
- **Testing**: positive checkpoint/list/rollback preflight fixtures and negative missing-backend/stale-state/unconfirmed/leak fixtures.
- **Non-claims**: this does not make Git the only backend, does not implement remote checkpoint storage, and does not auto-rollback without explicit host confirmation.
