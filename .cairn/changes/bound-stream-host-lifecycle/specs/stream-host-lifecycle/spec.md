# Bounded stream host lifecycle

## ADDED Requirements

### Requirement: Reserve capacity before dispatch

r[animus.stream_host.admission]

The selected host MUST admit explicit task, queue, byte, and lifetime bounds before dispatch without changing default product activation.

#### Scenario: Admitted stream

GIVEN an authorized stream and available capacity
WHEN the host acquires a permit
THEN it reserves retained state and output within the declared budget.

#### Scenario: Saturated host

GIVEN exhausted task or output capacity
WHEN a request seeks new effect dispatch
THEN the host rejects or waits within bounds without starting that effect.

### Requirement: Fence and drain notifications

r[animus.stream_host.wakeups]

The selected host MUST publish state before notification, use generation-fenced wakeups, and reschedule bounded drains when pending work remains.

#### Scenario: Signal precedes wait

GIVEN published events and a signal before the consumer waits
WHEN the consumer observes state
THEN it drains the queued events without requiring another producer signal.

#### Scenario: Stale handle signals

GIVEN a retired subscription generation
WHEN its old handle signals
THEN a replacement subscription remains unchanged.

### Requirement: Preserve deadline and cleanup ownership

r[animus.stream_host.teardown]

The selected host MUST preserve the admitted deadline and perform logical cleanup once only after dependent access ends, with unresolved teardown explicit.

#### Scenario: Slow output completes

GIVEN a retained output buffer and a pending write
WHEN terminal write completion arrives
THEN the host releases the buffer and cleanup ownership once.

#### Scenario: Shutdown lacks acknowledgement

GIVEN cancellation without terminal access acknowledgement
WHEN the shutdown deadline expires
THEN the host records unresolved teardown and prohibits slot reuse.

### Requirement: Reuse replay and authorization fences

r[animus.stream_host.replay]

The selected host MUST recheck current authorization before protected delivery and use existing replay fences without adding a second semantic ledger.

#### Scenario: Valid reconnect

GIVEN a current authorized replay cursor
WHEN the transport requests retained events
THEN the existing planner supplies the admitted durable projection.

#### Scenario: Revoked or stale reconnect

GIVEN revoked authority or drift in generation, branch, or redaction policy
WHEN protected delivery resumes
THEN the host denies delivery or selects the existing explicit rebuild outcome.

### Requirement: Never replay effects because output is blocked

r[animus.stream_host.effects]

The host MUST preserve existing effect reservations and distinguish publication, transport observations, and authoritative completion without replay after output errors.

#### Scenario: Draft fails before publication

GIVEN an unpublished response draft and no dispatched effect
WHEN bounded output fails
THEN the host discards the draft and reports a bounded error.

#### Scenario: Output fails after an effect

GIVEN a dispatched effect and a failed subscriber stream
WHEN the host processes the error
THEN it retains the effect observation and does not dispatch the effect again.
