## Context

`run_agent_loop` is synchronous and advances planned turns until completion. `UserControlEvent::Pause` classifies queued work, but no live wait port or pause-resume state exists.

`CodingSessionCommand::Resume` means durable session recovery. The new live command therefore uses `ResumeRun` and does not alter recovery semantics.

The reusable runtime cannot own sockets, clocks, persistence, process control, or ambient executors. The host must own synchronization and external transport bridges.

## Decisions

### Decision: model control as a revisioned desired-state latch

**Choice:** Add `Running`, `PauseRequested`, `Paused`, `Cancelling`, and `Terminal` phases. Commands carry an exact run identity, command identity, and expected revision.

A pure reducer applies commands and checkpoint observations. `Pause` changes `Running` to `PauseRequested`. A safe checkpoint changes `PauseRequested` to `Paused`. `ResumeRun` changes either pause phase to `Running`. `Cancel` changes every non-terminal phase to `Cancelling`.

**Rationale:** Pause and resume express current desired state. A command queue can replay stale intent or overflow before the worker observes it. A revisioned latch prevents this ambiguity.

### Decision: separate pure transitions from blocking

**Choice:** Put transport-neutral DTOs and the `AgentControlWait` port in `animus-adapters`. Put the deterministic reducer in `animus-runtime`. Put `Mutex`, `Condvar`, and thread blocking in `animus-std-control`.

**Rationale:** Reducer tests need no threads or host services. The standard host remains a thin imperative shell over the same reducer.

### Decision: block only at safe checkpoints

**Choice:** The runtime calls the injected wait port before loop-fabric evaluation and immediately before model or tool dispatch. The call returns `Continue` or `Cancel`.

The waiter blocks while the phase is `Paused`. It emits the paused transition before sleeping. It never holds runtime, model, tool, middleware, or persistence locks while it sleeps.

**Rationale:** This prevents new external work while preserving completed in-flight observations. It avoids unsafe thread suspension and partial tool effects.

### Decision: cancellation wins

**Choice:** `Cancel` is monotonic until terminal state. `Pause` and `ResumeRun` cannot leave `Cancelling`. A cancellation publication wakes every waiter.

**Rationale:** Concurrent pause and cancellation need one deterministic safety outcome.

### Decision: use predicate-based condition-variable waiting

**Choice:** The standard host stores the full control state under one mutex. Waiters re-evaluate the phase after every condition-variable wake.

Publishers mutate state before notification. Resume, cancellation, terminal closure, and host failure notify all waiters.

**Rationale:** The state predicate prevents lost wakeups and tolerates spurious wakeups.

### Decision: make subscriptions bounded and non-blocking

**Choice:** The host keeps a bounded revisioned event journal. Subscribers read with a cursor and can block on the same condition variable.

A subscriber behind the retained range gets an explicit `Lagged` outcome with the oldest available revision. Slow subscribers never block publication.

**Rationale:** Control state and event delivery have different purposes. The state latch preserves current intent, while the journal provides ordered operator evidence.

### Decision: keep transport authority outside the feature

**Choice:** External bridges resolve authentication, selectors, and run ownership before they call the publisher. The core accepts only exact run identities.

**Rationale:** Unix sockets, ACP, Iroh, and Syndicate have different authority and delivery contracts. Animus must not choose an ambient transport.

## State Flow

```text
publisher command
      |
      v
pure command reducer ---- stale or duplicate ----> typed rejection
      |
      v
revisioned state + event
      |
      +----> bounded subscribers
      |
agent reaches safe checkpoint
      |
      v
pure checkpoint reducer
      |
      +---- Running ------> continue dispatch
      +---- Paused -------> Condvar wait
      +---- Cancelling ---> return cancel permit
```

## In-Flight Policy

The first version is cooperative. A provider or tool call that already started can finish. Its result enters the normal runtime transition before the next gate.

The host can add provider-owned cancellation later. Such cancellation needs explicit interrupted-call repair and cannot be inferred from a pause request.

## Adversarial Cases

- Resume arrives before the worker enters the condition-variable wait.
- Cancel races with pause or resume.
- A duplicate command identity arrives after a successful command.
- A command targets a prior run identity.
- A publisher uses a stale expected revision.
- A subscriber falls behind the retained journal.
- A condition variable wakes without a state change.
- The control host closes while a worker is paused.
- A runtime has no injected control port.

Each case must return one deterministic transition, no-op, permit, lag result, or typed error.

## Risks and Trade-offs

- Cooperative pause cannot reclaim tokens from a request that already crossed the provider boundary.
- An in-memory paused run does not survive process restart.
- A blocked operating-system thread remains allocated, but it performs no polling and consumes no model tokens.
- Strict revision admission requires external controllers to refresh state after a conflict.

## Non-Claims

This design does not prove external transport delivery, controller authorization, provider cancellation, tool rollback, process durability, exactly-once effects, fairness between publishers, production readiness, or release eligibility.
