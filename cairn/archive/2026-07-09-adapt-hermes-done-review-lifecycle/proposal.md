# Proposal: Adapt Hermes done-review lifecycle contracts

## Why

Animus already has a Hermes-style review report parser, but it does not yet model the surrounding lifecycle that decides when a host can ask for an independent post-completion review, which bounded evidence is supplied, how completion rails are represented, how reviewer attempt policy is selected, or how review outcomes feed safe continuation.

Animus should adapt the lifecycle as optional SDK contracts. Embedders can build their own review gates, reviewer services, and evidence collectors on top of typed DTOs without Animus baking in Hermes commands, default reviewers, global state, or automatic model dispatch.

## What Changes

- Define typed review lifecycle requests, evidence bundles, completion rail descriptors/results, reviewer attempt plans, and review outcome receipts.
- Keep diff collection, VCS inspection, file reads, reviewer model calls, timers, persistence, UI prompts, and metrics writes host-owned.
- Treat PASS/WARN/FAIL as structured evidence that can request host continuation or block a completion claim without automatically fixing, retrying, mutating files, or opening lifecycle changes.
- Add positive and negative fixtures for valid review requests, bounded evidence, failed rails, missing reviewer service, malformed reviewer output, unsafe receipts, and no default behavior.

## Impact

- **Files**: future adapter/runtime DTOs, pure lifecycle reducers, tests, docs, and focused rail updates.
- **Testing**: positive lifecycle and receipt cases plus negative missing-service, unsafe-evidence, malformed-report, and automatic-dispatch fixtures.
- **Non-claims**: this does not add a built-in `done-review` command, require Hermes, select a default reviewer, call providers, inspect VCS state, persist metrics, enforce project-specific completion rails, or make post-completion review mandatory for Animus embedders.
