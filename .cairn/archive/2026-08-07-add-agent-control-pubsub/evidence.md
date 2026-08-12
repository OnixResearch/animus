# Agent-control implementation evidence

## Outcome

The external agent-control feature is implemented. It pauses at safe checkpoints, blocks without polling, resumes through a revisioned state latch, and exposes bounded event subscriptions.

The archived `add-durable-active-turn-recovery` contract now supplies the parked-state composition. Exact parked checkpoint entries recover without provider continuation.

A later exact running entry restores eligibility for the other recovery gates. Consumer hosts retain checkpoint storage and transaction durability.

## Baseline

Before core implementation, the adapter, runtime, and standard-host baseline command passed:

```text
nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-std
```

The direct host `cargo` command was unavailable outside the Nix development shell. The Nix command established the usable baseline.

## Implemented boundaries

- `animus-adapters` owns typed commands, identities, phases, checkpoints, permits, metadata-only events, and the injected wait port.
- `animus-runtime` owns pure command, checkpoint, duplicate, stale, cancellation, and terminal reduction.
- The runtime shell gates loop-fabric, model, and tool dispatch.
- `CodingSessionAgentControlGate` lets prerequisite commits continue and gates provider, tool, and summarizer effects.
- `animus-std-control` owns explicit `Mutex` and `Condvar` synchronization plus a bounded revisioned event journal.
- The root `agent-control` feature re-exports the focused host crate.
- Active-turn control entries bind the turn, run, revision, and phase with a domain-separated BLAKE3 identity.
- Recovery returns `Parked` for the latest parked entry and resumes only after a later exact running entry.
- External transport, authentication, selector resolution, checkpoint storage, and durability remain consumer-owned.

## Passing evidence

- Focused adapter, runtime, standard-host, coding-session, active-turn, root-feature, and Wasm rail: passed.
- Agent-control rail source identity: `ac81a6a44439603d017752e965d2a6b63f5dcb0a0ac7cd2f2160d6a732e0ba79`.
- Active-turn recovery rail source identity: `71a91a47c1b76cd93323a084fcf2d0c063c33cb2f557c6e84525e0127ec8de0e`.
- Positive recovery tests prove that a parked entry blocks continuation until an exact running entry exists.
- Negative recovery tests reject cancellation as a parked or runnable checkpoint entry.
- `nix build .#checks.x86_64-linux.agent-control --no-link -L`: passed for the earlier in-memory implementation.
- Focused Clippy for `animus-std-control`, `animus-runtime`, and coding-session `animus-std`: passed with warnings denied.
- `cargo fmt --all -- --check`: passed.
- `nix run .#cargo-tigerstyle -- check`: passed after explicit assertion, searchable-path, parameter, and unit-name repairs.
- `scripts/cairn/check-component-catalog --root .`: passed with positive and negative fixtures.
- `scripts/cairn/check-request-lifecycle --root .`: passed with positive and negative fixtures.
- Cairn repository validation and proposal, design, and tasks gates: passed.

## Bounded repository blockers

The runtime facade inventory rail still reports 47 unclassified exports from pre-existing worktree changes. No agent-control export remains in that diagnostic set.

The roadmap rail classifies `add-agent-control-pubsub`, but other active changes remain unclassified. This change does not modify those packages.

The final sync and archive results appear in the closeout history for this change.

## Concurrency audit

The implementation uses one mutex for the state predicate and event journal. Publishers mutate state before `notify_all`. Waiters evaluate state in a loop after every wake.

Tests cover resume-before-wait, blocked resume, cancellation wake, stale run and revision rejection, duplicate command identity, journal lag, wrong-run checkpoints, and explicit host closure.

An advisory secondary review suggested separate locks, timeout polling, and reduced notification scope. Those suggestions were rejected because they weaken the shared predicate or conflict with indefinite token-free pause semantics.

## Non-claims

This evidence does not prove external transport delivery, controller authorization, provider cancellation, tool rollback, store durability, exactly-once effects, task completion, production readiness, or release eligibility.
