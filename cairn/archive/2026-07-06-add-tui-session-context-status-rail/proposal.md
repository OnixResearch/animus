# Proposal: TUI session context status rail

## Why

A pi-style harness surfaces session and context state in a persistent rail: which session/model is active, how much context budget remains, status, cost, git/branch state, environment, and todos. This change adds that rail to `animus-tui`, consuming existing session-tree-ledger, model-dispatch/provider-registry, retrieval-budget, cost-model-routing, process-job-profile, and status-authority DTOs downstream, without owning session/model contracts or credential storage.

## What Changes

- Add a session picker that lists and switches sessions from session-tree-ledger DTOs.
- Add a model picker that selects models from provider-registry/model-dispatch DTOs without credential leakage.
- Add a context/retrieval-budget gauge from retrieval-budget DTOs.
- Add a status bar and cost overlay from status-authority and cost-model-routing DTOs.
- Add git-status/branch, environment, and todo panels from process-job-profile and existing status surfaces.
- Compose on the conversation render model; add no new reusable-crate contracts.

## Impact

- **Files**: new `animus-tui` session/model/context/status/cost/git/env/todo panel modules and tests.
- **Testing**: positive tests for session listing, model selection, gauge, status, cost, and panel rendering; negative tests for missing sessions, credential leakage, oversize context, unknown models, and reusable-crate contract leakage.
- **Non-claims**: this change does not own session/model contracts, does not read credentials, does not persist state, and does not add reusable-crate contracts.
