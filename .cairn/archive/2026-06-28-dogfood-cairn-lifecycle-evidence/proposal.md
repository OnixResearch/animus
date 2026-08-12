# Proposal: Dogfood Cairn lifecycle evidence in Animus

## Why

Animus references Cairn as its lifecycle engine and already stores native Cairn changes, but it lacks an explicit dogfood plan for accepted specs, change gates, sync/archive discipline, release evidence, and policy-bound validation. Without this, Cairn artifacts can become planning notes rather than durable lifecycle evidence.

This change makes Cairn use in Animus intentional and auditable.

## What Changes

- Establish the accepted spec tree as the canonical baseline for lifecycle decisions.
- Define how active changes move through proposal, design, tasks, validation, sync, archive, and evidence collection.
- Add lifecycle evidence expectations for validation, proposal/design/tasks gates, sync plans, archive receipts, and release-readiness summaries when supported.
- Define how Cairn policy identity and hashes are recorded in evidence.
- Add docs for maintainers explaining when to create a Cairn change, when direct edits are acceptable, and how to close out changes.

## Impact

- **Files**: lifecycle documentation, possible evidence output directory conventions, validation scripts/checks, accepted specs, Cairn artifacts.
- **Testing**: Cairn validation, proposal/design/tasks gates, sync/archive dry runs, evidence verification when available.
- **Non-claims**: this does not require every typo fix to open a Cairn change, does not replace ordinary Rust/Nix checks, and does not push or open pull requests.
