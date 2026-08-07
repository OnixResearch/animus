# Agent Harness Runtime

## ADDED Requirements

### Requirement: Agent control uses pure revisioned state

r[agent_harness_runtime.agent_control.state] Animus MUST model pause, resume, cancellation, checkpoint, and terminal transitions as deterministic reduction over supplied typed facts.

#### Scenario: Pause reaches a safe checkpoint

r[agent_harness_runtime.agent_control.state.pause]
- GIVEN a running exact run identity and a pause command at the current revision
- WHEN the pure reducer accepts the command and later accepts a safe checkpoint
- THEN the state MUST change through `PauseRequested` to `Paused`
- AND the transition MUST emit ordered metadata-only events with monotonic revisions.

#### Scenario: Stale state is supplied

r[agent_harness_runtime.agent_control.state.stale]
- GIVEN a command has a different run identity, duplicate command identity, or stale expected revision
- WHEN command reduction runs
- THEN it MUST return a deterministic rejection or idempotent duplicate outcome
- AND it MUST NOT mutate the current control state.

### Requirement: Agent control waiting is an injected port

r[agent_harness_runtime.agent_control.port] Reusable runtime code MUST use an injected control-wait port and MUST NOT construct ambient synchronization, transport, clock, process, network, or persistence services.

#### Scenario: Host injects a control waiter

r[agent_harness_runtime.agent_control.port.injected]
- GIVEN a host supplies a waiter for one exact run
- WHEN the runtime reaches a declared safe checkpoint
- THEN it MUST request a typed `Continue` or `Cancel` permit
- AND host blocking behavior MUST remain outside the reusable runtime.

#### Scenario: No waiter is supplied

r[agent_harness_runtime.agent_control.port.absent]
- GIVEN a runtime uses the compatibility adapter bundle without a control waiter
- WHEN it reaches the same checkpoint
- THEN it MUST continue with existing runtime behavior
- AND it MUST NOT discover or create an ambient controller.

### Requirement: Paused runs start no new dispatch

r[agent_harness_runtime.agent_control.runtime] A paused runtime MUST start no new loop-fabric, model, tool, or summarizer dispatch until an exact resume permit is observed.

#### Scenario: Pause precedes dispatch

r[agent_harness_runtime.agent_control.runtime.gate]
- GIVEN a pause request becomes active before a safe dispatch checkpoint
- WHEN the runtime reaches that checkpoint
- THEN it MUST block before the external dispatch
- AND the wait MUST NOT consume a planning turn, model token, or polling iteration.

#### Scenario: Pause arrives during an external call

r[agent_harness_runtime.agent_control.runtime.in_flight]
- GIVEN a provider or tool call already crossed its adapter boundary
- WHEN a pause command becomes active
- THEN the first version MUST allow the in-flight call to return through normal validation
- AND it MUST block before the next declared external dispatch.

#### Scenario: Parked control survives active-turn recovery

r[agent_harness_runtime.agent_control.runtime.recovery]
- GIVEN an exact parked control entry exists in the current active-turn checkpoint
- WHEN recovery plans provider continuation
- THEN it MUST return a parked action without provider or tool dispatch
- AND only a later exact running entry MAY restore eligibility for the other recovery gates.

### Requirement: The standard host waits without lost wakeups

r[agent_harness_runtime.agent_control.host] The standard agent-control host MUST use predicate-based synchronized waiting over explicit state and MUST expose bounded event subscriptions.

#### Scenario: Resume precedes condition-variable sleep

r[agent_harness_runtime.agent_control.host.resume_race]
- GIVEN a pause command is accepted and a resume command is accepted before the worker sleeps
- WHEN the worker evaluates the synchronized state predicate
- THEN it MUST observe `Running` and continue without sleeping
- AND the resume notification MUST NOT be lost.

#### Scenario: Subscriber exceeds retention

r[agent_harness_runtime.agent_control.host.lag]
- GIVEN a subscriber cursor is older than the retained bounded event journal
- WHEN the subscriber requests the next event
- THEN the host MUST return an explicit lag outcome with the oldest available revision
- AND the subscriber MUST NOT block publication or agent execution.

### Requirement: Concurrent control has deterministic precedence

r[agent_harness_runtime.agent_control.concurrency] Cancellation and terminal transitions MUST have monotonic precedence over pause and resume.

#### Scenario: Cancel races with resume

r[agent_harness_runtime.agent_control.concurrency.cancel]
- GIVEN pause, resume, and cancel commands target the same exact run
- WHEN accepted commands are reduced in host-observed order
- THEN cancellation MUST prevent every later pause or resume from returning the run to `Running`
- AND every waiter MUST wake with a `Cancel` permit.

#### Scenario: Control host closes

r[agent_harness_runtime.agent_control.concurrency.close]
- GIVEN a worker is paused and the explicit control host closes
- WHEN the waiter wakes
- THEN it MUST return a typed closed-host error or cancellation outcome
- AND it MUST NOT continue dispatch through an implicit fallback.

### Requirement: Agent-control evidence is metadata-only

r[agent_harness_runtime.agent_control.evidence] Control commands, events, acknowledgements, and receipts MUST contain bounded identities, revisions, phases, checkpoints, outcomes, and non-claim metadata only.

#### Scenario: Safe event is published

r[agent_harness_runtime.agent_control.evidence.safe]
- GIVEN an admitted control transition
- WHEN the host publishes its event
- THEN the event MUST bind the run identity, command identity when present, revision, phase, and checkpoint when present
- AND it MUST omit prompts, messages, reasoning, tool bodies, provider payloads, credentials, paths, and raw errors.

#### Scenario: Private evidence is requested

r[agent_harness_runtime.agent_control.evidence.rejects]
- GIVEN a caller expects control evidence to include private runtime content or unsupported delivery and durability claims
- WHEN evidence projection runs
- THEN Animus MUST reject or omit those fields without echoing private values
- AND the result MUST NOT establish transport delivery, provider cancellation, durability, completion, or release eligibility.

### Requirement: Agent control has positive and negative race evidence

r[agent_harness_runtime.agent_control.verification] Agent control MUST include deterministic positive and negative tests for reducers, runtime gates, condition-variable races, cancellation precedence, subscriber lag, closure, and absent-control compatibility.

#### Scenario: Focused control evidence passes

r[agent_harness_runtime.agent_control.verification.positive]
- GIVEN tests cover pause, resume, cancellation, dispatch gating, wake-before-wait, event order, and compatibility mode
- WHEN the focused control rail runs
- THEN production reducer, runtime, and standard-host entrypoints MUST return the expected outcomes
- AND repeated pure reductions over equal inputs MUST return equal states and events.

#### Scenario: Adversarial control attempt fails

r[agent_harness_runtime.agent_control.verification.negative]
- GIVEN tests supply stale runs, stale revisions, duplicate commands, terminal mutations, lagging cursors, spurious wakes, host closure, or private evidence
- WHEN the focused control rail runs
- THEN each attempt MUST fail or return its declared bounded outcome before prohibited dispatch or disclosure
- AND reusable authority, cancellation, recovery, and event boundaries MUST remain intact.
