# Proposal: Adapt Syndicate dataspace adapters

## Why

Syndicate is listed as a reference for assertion-based state replication, facets, dataspaces, and conversational concurrency, but the runtime-loop plan explicitly defers Syndicate integration. Animus needs a dedicated optional-adapter plan so dataspace ideas can be explored without becoming implicit core architecture or a hidden runtime dependency.

This change defines Syndicate-inspired integration as a host adapter and event projection boundary.

## What Changes

- Define optional dataspace adapter contracts for publishing bounded runtime assertions, subscribing to safe query patterns, and receiving host orchestration events.
- Map Animus runtime events, session/provenance summaries, tool states, background work, and agent/subagent lifecycle states into redacted assertion shapes.
- Require facet/session scoping, cancellation, lease/expiry behavior, and tenant/workspace isolation before concurrent orchestration uses a dataspace backend.
- Add deterministic fake-dataspace tests for publish/subscribe, ordering, cancellation, stale assertions, and malformed subscriptions.
- Keep Syndicate crates and live dataspace transports out of default reusable runtime/core dependencies.

## Impact

- **Files**: optional dataspace adapter DTOs, fake-dataspace fixtures, docs, dependency-isolation checks, and Cairn artifacts.
- **Testing**: deterministic fake-dataspace positive/negative tests; optional live integration smoke tests only with explicit host config.
- **Non-claims**: this does not make Syndicate the internal runtime state store, require a dataspace daemon, add network transport by default, or replace the session ledger/provenance contracts.
