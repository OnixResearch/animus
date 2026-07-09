# Proposal: Adapt Hermes VCS snapshot ledger contracts

## Why

Hermes' checkpoint and shadow-autocommit ideas are useful for recovery and audit, but Git plumbing, shadow branches, automatic snapshots, and accept/squash/drop commands are product behavior. Animus should adapt the underlying pattern as VCS-neutral snapshot ledger and checkpoint plan contracts.

The SDK should let host shells expose recovery workflows over jj, Git, in-memory stores, BranchFS-style workspaces, or other backends without Animus mutating VCS state by default.

## What Changes

- Define snapshot intent, snapshot record, snapshot lineage, accept/drop/squash/restore plan, freshness proof, confirmation binding, and safe receipt DTOs.
- Compose with existing checkpoint rollback and provenance ledger contracts.
- Keep VCS commands, shadow branches, stash refs, worktree writes, cleanup, UI prompts, and background snapshot scheduling host-owned.
- Require explicit host policy and confirmation for protected restore/accept/drop/squash mutations.

## Impact

- **Files**: future workspace/provenance DTOs, pure ledger reducers, tests, docs, and focused fixtures.
- **Testing**: positive VCS-neutral snapshot lineage and restore-plan tests plus negative stale snapshot, missing service, backend-specific command, no-confirmation, auto-snapshot, and receipt leak fixtures.
- **Non-claims**: this does not add autocommit, Git shadow branches, jj commands, automatic checkpoints, restore UI, or VCS mutation to reusable Animus crates.
