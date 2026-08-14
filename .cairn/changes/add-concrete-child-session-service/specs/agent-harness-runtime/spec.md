# Agent Harness Runtime

## ADDED Requirements

### Requirement: Child lifecycle uses normal coding sessions

r[agent_harness_runtime.child_session_service.lifecycle] Animus MUST provide a non-default host implementation of enqueue, status, cancel, and collect that operates admitted child work through normal coding-session instances and generation-fenced handles.

#### Scenario: An admitted child is enqueued

r[agent_harness_runtime.child_session_service.lifecycle.enqueue]
- GIVEN a child draft, context packet, authority intersection, and required services pass admission
- WHEN the child service enqueues the request
- THEN it MUST create one child coding session and return one stable generation-bound handle
- AND provider, tool, store, event, cancellation, and recovery ordering MUST follow the accepted coding-session contracts.

#### Scenario: A stale handle is used

r[agent_harness_runtime.child_session_service.lifecycle.stale]
- GIVEN a status, cancel, or collect request carries a stale generation or foreign parent
- WHEN the child service validates the handle
- THEN it MUST fail closed before child mutation
- AND it MUST NOT discover or substitute another child session.

### Requirement: Child construction uses an explicit factory

r[agent_harness_runtime.child_session_service.factory] The child host MUST receive an explicit session factory and explicit provider, tool, persistence, context, timer, cancellation, event, kernel, and skill services without ambient discovery or process-global registration.

#### Scenario: A provider service is absent

r[agent_harness_runtime.child_session_service.factory.missing]
- GIVEN an admitted child kind requires provider dispatch and no provider service is supplied
- WHEN the factory prepares the child session
- THEN construction MUST return a typed missing-service outcome
- AND it MUST NOT inspect ambient credentials, provider files, global registries, or environment values.

### Requirement: Child authority is a monotonic intersection

r[agent_harness_runtime.child_session_service.authority] Before construction, effective child authority and budgets MUST intersect parent policy, child profile, request ceilings, workspace scope, model and tool grants, depth, fan-out, time, token, turn, and result limits.

#### Scenario: A child requests a wider workspace

r[agent_harness_runtime.child_session_service.authority.workspace]
- GIVEN a child request names a workspace scope wider than its parent grant
- WHEN child admission runs
- THEN admission MUST deny creation with the widening source identified
- AND no session, store, provider, tool, or workspace effect may occur.

### Requirement: Child lifecycle and lineage are durable safe facts

r[agent_harness_runtime.child_session_service.persistence] The host MUST persist safe child identity, parent lineage, generation, lifecycle state, session head, result identity, and failure class through explicit store services while private child bodies remain in child-owned private channels.

#### Scenario: A non-terminal child is restored

r[agent_harness_runtime.child_session_service.persistence.restore]
- GIVEN restart finds a valid non-terminal child record and matching current services
- WHEN child restore admission runs
- THEN it MUST reopen the child through coding-session recovery using the recorded generation and session head
- AND it MUST not repeat committed provider or tool effects.

### Requirement: Parent results are bounded and typed

r[agent_harness_runtime.child_session_service.results] Collection MUST return a typed bounded child result with terminal class, safe lineage, content identity, and admitted model-visible body, without importing the full child transcript by default.

#### Scenario: A child result exceeds its limit

r[agent_harness_runtime.child_session_service.results.oversized]
- GIVEN a completed child result exceeds the admitted parent result limit
- WHEN collection projects the result
- THEN it MUST return a typed bound outcome or approved bounded summary
- AND it MUST NOT expose the excess body through receipts, events, or diagnostics.

### Requirement: Concrete child service evidence is adversarial

r[agent_harness_runtime.child_session_service.verification] The child service MUST include production-entrypoint tests for lifecycle success, cancellation, restart, lineage, result projection, missing services, stale handles, foreign parents, authority widening, bounds, malformed context, race outcomes, leak rejection, and dependency isolation.

#### Scenario: The child service is ready to archive

r[agent_harness_runtime.child_session_service.verification.closeout]
- GIVEN maintainers intend to sync and archive the child service
- WHEN focused and lifecycle validation runs
- THEN subagent, coding-session, store, recovery, RLM, receipt, dependency, Cairn, and relevant Nix checks MUST pass
- AND automatic delegation, remote workers, sandboxing, child correctness, task success, and production readiness MUST remain non-claims.
