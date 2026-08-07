## Why

Animus agents can receive queued pause interrupts, but the synchronous runtime has no live resume gate. Hosts must poll or keep advancing model turns while they wait for outside work.

Animus needs a transport-neutral control contract that lets an external publisher pause, resume, or cancel one exact run. A standard-library host must block without polling and wake without losing state changes.

## What Changes

- Add pure revisioned agent-control state and command reducers. r[agent_harness_runtime.agent_control.state]
- Add an implementation-free safe-checkpoint wait port. r[agent_harness_runtime.agent_control.port]
- Gate model, tool, and loop-fabric dispatch at host-selected safe checkpoints. r[agent_harness_runtime.agent_control.runtime]
- Add a standard-library control hub with a condition-variable state latch and bounded event subscriptions. r[agent_harness_runtime.agent_control.host]
- Define deterministic stale-run, stale-revision, duplicate-command, lag, shutdown, and cancellation behavior. r[agent_harness_runtime.agent_control.concurrency]
- Add metadata-only events and positive and negative control evidence. r[agent_harness_runtime.agent_control.evidence] r[agent_harness_runtime.agent_control.verification]

## Success Criteria

- A paused agent starts no new model, tool, or loop-fabric dispatch.
- The waiting runtime consumes no model tokens and performs no polling.
- Resume and cancellation wake a paused runtime without lost wakeups.
- Stale run identities and stale revisions fail before state mutation.
- Slow subscribers do not block command publication or agent execution.
- Existing runtimes without an injected control waiter keep their current behavior.

## Impact

- **Adapter contracts**: transport-neutral commands, checkpoints, permits, events, and wait port.
- **Runtime**: pure reducer plus optional safe-checkpoint gates.
- **Standard host**: explicit synchronized in-memory control hub and event journal.
- **Consumers**: TUI, ACP, Unix socket, Iroh, or Syndicate bridges can publish through the same handle.
- **Testing**: reducer, race, lag, dispatch-gate, no-default-control, and lifecycle tests.

## Out of Scope

- Provider or tool preemption after an external call starts.
- Ambient sockets, network clients, global registries, authentication, or process signals in reusable crates.
- Claims about checkpoint storage durability, power-loss behavior, or exact recovery after a host storage failure.
- Claims for transport delivery, provider cancellation, exactly-once effects, or production readiness.

## Affected Specs

- `agent-harness-runtime`: agent-control state, wait port, runtime gates, host synchronization, evidence, and verification.
