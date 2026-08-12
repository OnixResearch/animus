## ADDED Requirements

### Requirement: Standard evaluation host is non-default and explicit

r[agent_harness_runtime.native_evaluation_host.boundary] Animus MUST provide native evaluation execution through a separate non-default standard-library host that consumes checked plans and explicitly supplied services, and the default runtime MUST NOT discover or gain filesystem, process, socket, provider, tool, credential, clock, container, scheduler, network, persistence, or evaluation-host authority.

#### Scenario: Host is constructed explicitly

r[agent_harness_runtime.native_evaluation_host.boundary.selected]
- GIVEN a caller selects the evaluation-host feature, loads one explicit checked profile, and supplies an admitted service catalog
- WHEN host preflight runs
- THEN the host binds the selected services and authority to the exact evaluation plan
- AND no additional service or capability is discovered from ambient state.

#### Scenario: Default Animus runs without evaluation host

r[agent_harness_runtime.native_evaluation_host.boundary.default]
- GIVEN the root package uses default features
- WHEN dependency, authority, and facade checks run
- THEN no evaluation host, gateway, provider, process, container, credential, or persistence service activates
- AND reusable evaluation contracts remain effect-free.

### Requirement: Host execution is bounded and cancellation-aware

r[agent_harness_runtime.native_evaluation_host.execution] Evaluation host execution MUST use explicit injected services, separate positive limits for concurrent episodes and agent runs within each episode, exact cancellation and deadline identities, typed terminal outcomes, and cleanup obligations, and it MUST fail before effects when any required service or bound is unavailable, denied, incompatible, stale, or exceeded.

#### Scenario: Multi-agent episode runs within bounds

r[agent_harness_runtime.native_evaluation_host.execution.bounded]
- GIVEN an admitted recipe fits both concurrency limits and every required service is available
- WHEN the host executes the plan
- THEN active calls never exceed either selected limit
- AND completed observations are reduced through the native evaluation core in deterministic plan order.

#### Scenario: Required service is unavailable

r[agent_harness_runtime.native_evaluation_host.execution.unavailable]
- GIVEN preflight finds a missing provider, tool, verifier, artifact, gateway, timer, cancellation, cleanup, or storage service
- WHEN execution is requested
- THEN the host returns the matching typed blocked outcome before partial dispatch
- AND it does not select a local process, provider, container, or persistence fallback.

#### Scenario: Cancellation races an active call

r[agent_harness_runtime.native_evaluation_host.execution.cancelled]
- GIVEN cancellation becomes current while an admitted service call is active
- WHEN the host observes cancellation and service termination evidence
- THEN it stops new dispatch, records a bounded cancelled or cleanup-required outcome, and releases permits
- AND late results cannot overwrite terminal episode state.

### Requirement: Local model gateway preserves durable turn ordering

r[agent_harness_runtime.native_evaluation_host.gateway] An opt-in evaluation model gateway MUST accept only configured loopback or caller-admitted binds and reviewed protocol dialects, use separate scoped model and task-state capabilities, bind exact request and attempt identities, gate model dispatch on durable accepted-turn evidence, coalesce only exact identical non-streaming retries, and withhold terminal stream delivery until matching durable turn-commit evidence exists.

#### Scenario: External harness call completes safely

r[agent_harness_runtime.native_evaluation_host.gateway.complete]
- GIVEN an admitted harness sends a bounded supported request with a valid model capability and matching durable accepted-turn evidence
- WHEN the selected provider returns a valid response and the turn commit completes
- THEN the gateway records bounded canonical call metadata and releases the terminal response event
- AND raw provider bodies do not enter safe records or receipts.

#### Scenario: Retry request matches exactly

r[agent_harness_runtime.native_evaluation_host.gateway.retry]
- GIVEN a non-streaming retry matches canonical request, run, model, sampling, policy, active-turn, and attempt-lineage identities for an in-flight or completed call
- WHEN retry admission runs
- THEN the host can await or reuse the first bounded result without a second provider dispatch
- AND a changed field forces a distinct attempt or typed rejection.

#### Scenario: Gateway posture is unsafe

r[agent_harness_runtime.native_evaluation_host.gateway.denied]
- GIVEN configuration requests public bind, wildcard network, shared model and state capabilities, response or tool-result rewriting, content-only retry matching, unbounded bodies or streams, dispatch before acceptance, or terminal delivery before commit
- WHEN gateway preflight runs
- THEN the request is denied before transport or provider dispatch
- AND no successful capture, trace, or completion claim is emitted.

#### Scenario: Run ends with straggler traffic

r[agent_harness_runtime.native_evaluation_host.gateway.sealed]
- GIVEN a run is cancelled, timed out, failed, or terminal while a model, state, or stream operation remains active
- WHEN teardown seals the run
- THEN late operations are rejected and cannot mutate ledger, episode, reward, resume, or receipt state
- AND unresolved cleanup remains explicit.

### Requirement: Verifier execution and artifact transfer preserve isolation

r[agent_harness_runtime.native_evaluation_host.verifier] Authoritative-capable evaluation profiles MUST use a separately identified verifier service and environment with immutable subject, rubric, policy, and mutation-lineage bindings, and artifact transfer MUST be allowlisted, bounded, collision-checked, no-follow, and identity-attributed, while shared-runtime judging MUST remain explicit and advisory.

#### Scenario: Isolated verifier receives artifacts

r[agent_harness_runtime.native_evaluation_host.verifier.isolated]
- GIVEN a solver reaches its handoff boundary and an allowlisted collision-free artifact manifest passes policy
- WHEN the host invokes a distinct verifier service
- THEN only admitted artifacts and bounded ledger-derived context reach the verifier
- AND hidden verifier material remains outside the solver environment.

#### Scenario: Artifact path is hostile

r[agent_harness_runtime.native_evaluation_host.verifier.artifact_denied]
- GIVEN an artifact is missing, oversized, duplicated, colliding, escaping its capability root, a planted link, or includes hidden tests, credentials, logs, environment, or undeclared trajectory content
- WHEN collection or restore preflight runs
- THEN transfer fails before verifier execution
- AND no partial artifact set is treated as valid evidence.

#### Scenario: Shared judge passes

r[agent_harness_runtime.native_evaluation_host.verifier.shared]
- GIVEN a judge runs in the solver-controlled runtime and returns a passing result
- WHEN host reduction classifies it
- THEN the result remains shared and advisory
- AND it cannot satisfy isolation, verifier-soundness, correctness, or authoritative completion requirements.

### Requirement: Evaluation persistence is explicit, atomic, and resume-safe

r[agent_harness_runtime.native_evaluation_host.persistence] Evaluation output persistence MUST use explicit capability-rooted destinations, complete bounded byte preparation, selected synchronization, atomic replacement for rewritten records, strict reopen validation, and native content-addressed resume decisions, and torn, malformed, failed, missing, stale, conflicting, or mismatched records MUST NOT suppress rerun.

#### Scenario: Safe output is replaced atomically

r[agent_harness_runtime.native_evaluation_host.persistence.atomic]
- GIVEN a complete checked output and admitted destination capability are ready
- WHEN the host persists and reopens the record
- THEN either the prior complete record or the new complete record is visible according to transaction evidence
- AND the receipt binds the selected durability class without claiming stronger host behavior.

#### Scenario: Output contains a torn record

r[agent_harness_runtime.native_evaluation_host.persistence.torn]
- GIVEN an interrupted output contains a truncated, malformed, uncorrelated, failed, or identity-mismatched record
- WHEN reopen and resume evaluation run
- THEN the record is excluded from completion evidence and its work remains owed
- AND valid unrelated records remain available according to explicit policy.

#### Scenario: Cleanup fails after terminal persistence

r[agent_harness_runtime.native_evaluation_host.persistence.cleanup]
- GIVEN terminal output is durable under the selected class but resource cleanup fails
- WHEN the host finalizes the run
- THEN terminal evidence remains distinguishable from cleanup-required state
- AND cleanup failure is not hidden or converted into an exactly-once claim.

### Requirement: Host evidence is metadata-only and bounded

r[agent_harness_runtime.native_evaluation_host.evidence] Evaluation host receipts MUST bind safe host profile, service catalog, execution, concurrency, gateway, verifier, artifact, persistence, cleanup, status, redaction, count, and BLAKE3 identity metadata and MUST exclude raw task or transcript content, hidden tests, reasoning, tool or provider bodies, credentials, environment values, physical paths, unrestricted logs, and unsupported mechanism claims.

#### Scenario: Host run emits safe receipt

r[agent_harness_runtime.native_evaluation_host.evidence.safe]
- GIVEN host execution reaches a typed terminal or blocked outcome with validated observations
- WHEN receipt projection runs
- THEN it emits only admitted metadata, identities, counts, failure classes, redaction, and non-claims
- AND it does not promote evaluation success into completion or release authority.

#### Scenario: Receipt overstates host guarantees

r[agent_harness_runtime.native_evaluation_host.evidence.overclaim]
- GIVEN a draft claims confinement, network enforcement, provider correctness, persistence durability, exactly-once effects, verifier soundness, benchmark validity, production readiness, completion, or release eligibility beyond checked evidence
- WHEN receipt validation runs
- THEN validation rejects the overclaim
- AND no success identity is emitted.

### Requirement: Evaluation host has a public-entrypoint vertical slice

r[agent_harness_runtime.native_evaluation_host.verification] The standard evaluation host MUST include one deterministic public-entrypoint vertical slice with fake task, agent, model, tool, verifier, artifact, gateway, timer, cancellation, cleanup, and storage services plus positive and negative lifecycle, gateway, verifier, persistence, receipt, dependency, and authority fixtures before archive.

#### Scenario: Host change is ready to archive

r[agent_harness_runtime.native_evaluation_host.verification.closeout]
- GIVEN maintainers intend to sync and archive the evaluation host change
- WHEN closeout checks run
- THEN policy, host, vertical-slice, failure-injection, gateway, verifier, persistence, receipt, dependency, Cairn, and Nix checks pass or record an exact blocker
- AND no live provider, tool, process, container, cloud, benchmark, training, production, or release behavior is claimed.
