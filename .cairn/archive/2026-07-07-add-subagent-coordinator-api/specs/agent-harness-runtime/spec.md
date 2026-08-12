# agent-harness-runtime Change Spec: Subagent coordinator API

## Requirements

### Requirement: Subagent execution uses injected services

r[agent_harness_runtime.subagent_coordinator.service] Animus runtime MUST coordinate subagent execution through injected host services for enqueue, status, cancel, and collect operations, and MUST NOT construct concrete queues, provider clients, process runners, external-agent clients, filesystem stores, network transports, or credential readers in reusable runtime code.

#### Scenario: Coordinator enqueues through an injected service

- GIVEN a validated child-run admission and an injected service for that child-run kind
- WHEN the coordinator enqueues the child run
- THEN the service receives the validated admission and returns a typed handle
- AND runtime does not discover or construct a fallback executor.

### Requirement: Coordinator transitions are deterministic

r[agent_harness_runtime.subagent_coordinator.state_machine] Subagent coordinator state transitions MUST be pure over the current child-run state, validated admission, requested operation, and injected service outcome.

#### Scenario: Running child cancellation is explicit

- GIVEN a child run is running with a valid service handle
- WHEN cancellation is requested and the injected service accepts cancellation
- THEN the coordinator moves the child run to a cancelling or cancelled state according to the typed outcome
- AND no completed receipt is emitted until a terminal collect or cancellation outcome is validated.

### Requirement: Lifecycle events are safe projections

r[agent_harness_runtime.subagent_coordinator.lifecycle] Subagent coordinator lifecycle events MUST include safe child identity, parent identity, child-run kind, status, failure class, budget summary when available, and bounded evidence references, and MUST NOT include raw transcripts, provider payloads, credentials, full path dumps, or unbounded tool output.

#### Scenario: Malformed child report is visible without leaking raw output

- GIVEN an injected service returns a malformed child report
- WHEN the coordinator classifies the outcome
- THEN the lifecycle event records a malformed-report failure class and safe evidence reference
- AND raw child output is not inserted into the event or parent context.

### Requirement: Coordinator failures fail closed

r[agent_harness_runtime.subagent_coordinator.fail_closed] The subagent coordinator MUST fail closed when required services are missing, handles are stale, service calls are denied, unavailable, timed out, cancelled, malformed, or would require ambient fallback.

#### Scenario: Missing service does not spawn fallback work

- GIVEN a valid child-run admission requires a scheduler or worker service
- WHEN no matching service is injected
- THEN the coordinator returns a service-unavailable diagnostic
- AND no shell command, background thread, external CLI, model call, or network request is attempted.

### Requirement: Coordinator closeout is evidenced

r[agent_harness_runtime.subagent_coordinator.verification] The subagent coordinator adoption MUST include positive enqueue/status/cancel/collect tests, negative fail-closed service tests, safe lifecycle event tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Coordinator evidence is complete

- GIVEN the coordinator API change is ready to archive
- WHEN closeout validation runs
- THEN evidence names state-machine tests, service fake tests, failure classification tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
