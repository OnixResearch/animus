# Design: Syndicate dataspace adapters

## Context

Syndicate offers useful patterns for assertion-based state and conversational concurrency. Animus can adapt those patterns at the host boundary while keeping its internal runtime loop, ledger, receipts, and adapter contracts independent.

## Decisions

### 1. Dataspace is an optional adapter

A dataspace backend is a host service that receives and emits typed DTOs. Reusable runtime code publishes events through injected services and does not depend on Syndicate crates or daemon state.

### 2. Publish redacted assertions, not raw transcripts

Runtime events, tool states, background work, subagent states, ledger summaries, and provenance facts are projected to bounded assertion shapes with redaction classes, hashes, status labels, and stable IDs. Raw prompts, provider payloads, full paths, file bodies, credentials, and unbounded outputs are not assertions by default.

### 3. Scope concurrency explicitly

Facets, sessions, tenants, workspaces, branches, and subagents are represented as explicit scopes. Cancellation, expiry/leases, stale assertions, and ownership transfer are typed events rather than implicit backend behavior.

### 4. Test against fake dataspaces first

Default tests use an in-memory fake dataspace with deterministic ordering. Live Syndicate transports, if added later, are opt-in smoke tests with explicit host configuration.

### 5. Preserve ledger/provenance authority

Dataspace assertions are projections or orchestration hints. The session ledger and provenance contracts remain the source of durable replay and evidence unless a future change explicitly changes that boundary.

## Risks

- Assertion projections can become a second source of truth. Keep them derived and bounded.
- Concurrency semantics can hide races. Model cancellation, expiry, and ownership transfer explicitly.
