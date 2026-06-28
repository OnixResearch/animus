# agent-harness-runtime Change Spec: Syndicate dataspace adapters

## Requirements

### Requirement: Optional dataspace adapter contract

r[agent_harness_runtime.syndicate_dataspace.adapter_contract] Animus MAY provide optional dataspace adapter contracts for publishing bounded runtime assertions, subscribing to safe query patterns, and receiving host orchestration events, but reusable runtime/core crates MUST NOT depend on Syndicate crates or live dataspace transports by default.

#### Scenario: Runtime uses injected dataspace service

r[agent_harness_runtime.syndicate_dataspace.adapter_contract.injected]
- GIVEN runtime wants to publish or observe dataspace state
- WHEN a dataspace backend is configured
- THEN runtime communicates through an injected typed adapter contract
- AND no default runtime/core crate constructs a Syndicate backend or network transport.

### Requirement: Redacted assertion projection

r[agent_harness_runtime.syndicate_dataspace.assertion_projection] Dataspace projections MUST map runtime events, tool states, background work, session/provenance summaries, and agent/subagent lifecycle states into bounded redacted assertion shapes.

#### Scenario: Raw sensitive material is not asserted

r[agent_harness_runtime.syndicate_dataspace.assertion_projection.redacted]
- GIVEN an event or state contains prompt text, provider payloads, credentials, full paths, file bodies, tool input/output, model output, or unbounded logs
- WHEN it is projected to a dataspace assertion
- THEN the assertion includes only approved redaction markers, hashes, counts, stable IDs, and safe status classes
- AND raw sensitive material is omitted by default.

### Requirement: Explicit scoping and lifecycle

r[agent_harness_runtime.syndicate_dataspace.scoping] Dataspace adapter DTOs MUST represent facet, session, tenant, workspace, branch, agent/subagent, cancellation, lease/expiry, stale assertion, and ownership-transfer semantics explicitly before being used for concurrent orchestration.

#### Scenario: Scoped cancellation removes assertions

r[agent_harness_runtime.syndicate_dataspace.scoping.cancellation]
- GIVEN a scoped agent, subagent, tool, or background task is cancelled
- WHEN the dataspace adapter processes the cancellation
- THEN it emits or updates bounded cancellation assertions for the affected scope
- AND unrelated tenant, workspace, branch, or session assertions are not modified.

### Requirement: Fake dataspace backend

r[agent_harness_runtime.syndicate_dataspace.fake_backend] Animus MUST provide deterministic fake-dataspace fixtures before live dataspace transports are treated as release evidence.

#### Scenario: Fake backend exercises ordering and stale cleanup

r[agent_harness_runtime.syndicate_dataspace.fake_backend.deterministic]
- GIVEN publish, subscribe, expiry, cancellation, and malformed-subscription fixtures
- WHEN fake-dataspace tests run
- THEN ordering and cleanup outcomes are deterministic
- AND no live network or daemon is required.

### Requirement: Dataspace tests

r[agent_harness_runtime.syndicate_dataspace.tests] Dataspace adapter work MUST include positive and negative tests for bounded projections, scoped publish/subscribe, cancellation, stale assertions, malformed subscriptions, denied queries, dependency isolation, and safe failure evidence.

#### Scenario: Malformed subscription fails safely

r[agent_harness_runtime.syndicate_dataspace.tests.malformed_subscription]
- GIVEN a subscription requests raw private payloads, crosses tenant/workspace scope, or uses an unsupported query shape
- WHEN the dataspace adapter validates it
- THEN validation fails with a typed issue code
- AND no raw private material is returned.

### Requirement: Dependency isolation

r[agent_harness_runtime.syndicate_dataspace.dependency_isolation] Animus MUST check that Syndicate crates, daemon clients, and live transport dependencies remain out of default reusable runtime/core dependency graphs unless a future accepted change makes them default.

#### Scenario: Syndicate dependency leaks into core

r[agent_harness_runtime.syndicate_dataspace.dependency_isolation.rejects_leak]
- GIVEN a Syndicate or live transport dependency is added to a default reusable runtime/core crate
- WHEN dependency isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to use the optional dataspace adapter boundary.

### Requirement: Dataspace closeout evidence

r[agent_harness_runtime.syndicate_dataspace.closeout] Syndicate dataspace adapter work MUST document optional live integration configuration and run fake-dataspace tests, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Dataspace evidence is reviewable

r[agent_harness_runtime.syndicate_dataspace.closeout.evidence]
- GIVEN dataspace adapter work is ready to archive
- WHEN reviewers inspect evidence
- THEN fake-dataspace tests, dependency isolation, workspace checks, and Cairn gates are named with status
- AND live integration is either explicitly configured or marked unavailable with a blocker.
