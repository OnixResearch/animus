# agent-harness-runtime Change Spec: Optional Syndicate host dataspace adapter

## Requirements

### Requirement: Optional Syndicate host adapter

r[agent_harness_runtime.syndicate_host_adapter.optional] Animus MUST keep any Syndicate-compatible live dataspace integration behind an optional host-owned adapter or non-default feature, and reusable core/runtime/adapter crates MUST NOT construct live Syndicate backends by default.

#### Scenario: Runtime publishes through injected host service

r[agent_harness_runtime.syndicate_host_adapter.optional.injected]
- GIVEN a host enables Syndicate-compatible dataspace coordination
- WHEN runtime or host shell code wants to publish or observe dataspace state
- THEN it communicates through an injected `animus_adapters::dataspace::AdapterService` implementation
- AND default `animus-core`, `animus-runtime`, and `animus-adapters` builds do not require Syndicate crates, daemon clients, network transports, credentials, or live daemon availability.

### Requirement: Redacted coordination projection

r[agent_harness_runtime.syndicate_host_adapter.projection] The optional host adapter MUST publish only bounded redacted assertions derived from approved runtime, headless, tool lifecycle, background-work, session/provenance, and agent/subagent observations.

#### Scenario: Runtime observation becomes safe assertion

r[agent_harness_runtime.syndicate_host_adapter.projection.redacted]
- GIVEN an observation contains prompt text, provider payloads, credentials, full private paths, file bodies, tool input/output, model output, or unbounded logs
- WHEN projection builds a dataspace assertion draft
- THEN the assertion contains only stable IDs, normalized scope labels, status classes, assertion kinds, counts, BLAKE3 hashes, and explicit redaction markers
- AND raw sensitive material is omitted or rejected before publication.

### Requirement: Scoped subscription and orchestration handling

r[agent_harness_runtime.syndicate_host_adapter.orchestration] The optional host adapter MUST validate safe subscription patterns and host orchestration events for cancellation, expiry, publish acknowledgement, and ownership transfer before forwarding them to a live or fake dataspace backend.

#### Scenario: Cancellation is scoped to one owner

r[agent_harness_runtime.syndicate_host_adapter.orchestration.scoped_cancellation]
- GIVEN a cancellation event targets one tenant, workspace, branch, session, facet, actor, or assertion owner
- WHEN the host adapter handles the event
- THEN only matching scoped assertions are cancelled, expired, or acknowledged
- AND unrelated tenants, workspaces, branches, sessions, facets, actors, and owners remain unchanged.

### Requirement: Safe unavailable-backend behavior

r[agent_harness_runtime.syndicate_host_adapter.failure] The optional host adapter MUST fail safely when live transport configuration is absent, a daemon is unavailable, credentials are missing, subscriptions are unsupported, leases are stale, or ownership-transfer identities do not match.

#### Scenario: Missing live backend does not affect default runtime

r[agent_harness_runtime.syndicate_host_adapter.failure.unavailable_backend]
- GIVEN live Syndicate transport is not configured or the configured backend is unavailable
- WHEN default runtime tests, fake-backend tests, or host startup without the optional feature run
- THEN no live connection is attempted by default
- AND any explicit live-adapter failure is reported as a typed safe diagnostic without raw credentials, transport payloads, or private paths.

### Requirement: Syndicate dependency isolation

r[agent_harness_runtime.syndicate_host_adapter.dependency_isolation] Animus MUST check that Syndicate crates, daemon clients, live transport code, and host credential plumbing remain outside default reusable core/runtime/adapter dependency graphs.

#### Scenario: Live transport dependency leaks into default graph

r[agent_harness_runtime.syndicate_host_adapter.dependency_isolation.rejects_leak]
- GIVEN a Syndicate crate, daemon client, network transport, or host credential dependency is added to a default reusable core/runtime/adapter crate
- WHEN dependency-isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to the optional host dataspace adapter boundary.

### Requirement: Syndicate host adapter closeout evidence

r[agent_harness_runtime.syndicate_host_adapter.closeout] Syndicate host adapter work MUST document host configuration, fake-backend release evidence, optional live smoke-test gating, dependency isolation, focused tests, workspace checks, and Cairn gates before archive.

#### Scenario: Host adapter evidence is reviewable

r[agent_harness_runtime.syndicate_host_adapter.closeout.evidence]
- GIVEN optional Syndicate host adapter work is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN evidence names redacted projection tests, safe subscription/orchestration tests, unavailable-backend tests, dependency-isolation checks, fake-backend tests, optional live smoke-test status, workspace checks, and Cairn gates
- AND any skipped live transport rail includes a concrete blocker, explicit opt-in requirement, and next-best fake-backend validation.
