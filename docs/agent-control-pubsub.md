# External agent control and subscriptions

Animus supports external pause, resume, and cancellation through an injected control port. A paused agent blocks without polling and starts no new model, tool, summarizer, or loop-fabric work.

The root compatibility feature is `agent-control`. The focused standard host is `animus-std-control`.

## Ownership

| Concern | Owner |
| --- | --- |
| Command, checkpoint, permit, event, and wait contracts | `animus-adapters` |
| Pure revisioned state transitions | `animus-runtime` |
| Mutex, condition variable, blocked thread, and bounded event journal | `animus-std-control` |
| Socket, ACP, Iroh, Syndicate, or TUI command source | Consumer host |
| Authentication, selector resolution, and controller policy | Consumer host |
| Durable paused state after restart | Future active-turn recovery composition |

Reusable crates do not open sockets, read clocks, send signals, discover controllers, or create global registries.

## Control phases

The pure reducer uses these phases:

1. `Running`
2. `PauseRequested`
3. `Paused`
4. `Cancelling`
5. `Terminal`

`Pause` changes `Running` to `PauseRequested`. The next safe checkpoint changes it to `Paused`.

`ResumeRun` changes `PauseRequested` or `Paused` to `Running`. The name is distinct from coding-session recovery `Resume`.

`Cancel` changes every non-terminal phase to `Cancelling`. Pause and resume cannot reverse cancellation.

## Safe checkpoints

The reusable runtime gates these boundaries:

- before loop-fabric evaluation;
- immediately before model dispatch;
- immediately before tool dispatch.

`CodingSessionAgentControlGate` also gates provider, tool, and summarizer effects. It lets prerequisite commits, recovery, and cancellation effects continue.

A pause request does not preempt a call that already crossed its adapter boundary. The call can finish through normal validation. The next safe checkpoint blocks new work.

## Blocking behavior

`AgentControlHub` stores the complete state under one mutex. `AgentControlWaiter` evaluates that state before every condition-variable wait.

The publisher mutates state before it notifies waiters. Thus, a resume that arrives before the worker sleeps cannot be lost.

Spurious wakeups are safe because the waiter evaluates the state again. Host closure wakes all waiters and returns a typed error. The runtime then stops without a running fallback.

The blocked operating-system thread remains allocated. It performs no polling and consumes no model tokens.

## Event subscriptions

`AgentControlSubscriber` reads a bounded revisioned event journal. It supports a non-blocking read and a blocking wait after one revision cursor.

A slow subscriber never blocks the publisher or agent. If its cursor falls behind retention, it receives `Lagged` with the oldest available revision.

Events contain only these fields:

- run identity;
- command identity when present;
- revision;
- phase;
- event class;
- safe checkpoint class when present.

Events do not contain prompts, messages, reasoning, tool bodies, provider payloads, credentials, paths, or raw errors.

## Host use

```rust
use animus::agent_control::{AgentControlHub, AgentControlSubscriptionOutcome};
use animus_adapters::{
    AgentControlAction, AgentControlCommand, AgentControlCommandId, AgentControlRunId,
};
use std::num::NonZeroUsize;

const EVENT_CAPACITY: usize = 64;

let run_id = AgentControlRunId::new("run.example")?;
let hub = AgentControlHub::new(
    run_id.clone(),
    NonZeroUsize::new(EVENT_CAPACITY).expect("capacity is non-zero"),
);
let publisher = hub.publisher();
let snapshot = publisher.snapshot()?;
let command = AgentControlCommand::new(
    run_id,
    AgentControlCommandId::new("command.pause")?,
    snapshot.revision(),
    AgentControlAction::Pause,
);
publisher.publish(&command)?;

let event = hub.subscriber().wait_after(snapshot.revision())?;
assert!(matches!(event, AgentControlSubscriptionOutcome::Event(_)));
```

A transport bridge must resolve authorization and the exact run before it calls the publisher. The bridge must refresh its snapshot after a stale revision response.

## Non-claims

This feature does not prove transport delivery, publisher authorization, provider cancellation, tool rollback, process durability, fairness, exactly-once effects, task completion, production readiness, or release eligibility.

## Focused validation

```sh
cargo test -p animus-adapters control --locked
cargo test -p animus-runtime control --locked
cargo test -p animus-std-control --locked
cargo test -p animus-std --features coding-session-host agent_control --locked
cargo check -p animus --features agent-control --locked
nix build .#checks.x86_64-linux.agent-control --no-link -L
```
