# Design: Compare-and-swap coding-session queue edits

## Context

Queued steering and follow-up items are mutable session state outside the transcript until injection. A client can display one version while another client edits or injects it.

The reducer needs an explicit precondition instead of position-only mutation.

## Success Contract

An edit or removal succeeds only when entry identity, queue generation, and expected text identity match current state. Success advances the queue generation exactly once.

A stale command returns a bounded conflict with the current safe snapshot identity. It does not change queue, transcript, active run, or durable cursor state.

A position-only remote edit, last-writer-wins update, or conflict response that leaks queued text is false completion.

## Decisions

### 1. Bind stable entry and state identities

Each queued item has a stable entry identity. Each queue state has a monotonic generation and a domain-separated BLAKE3 state identity.

Mutation commands bind the entry identity, expected generation, and BLAKE3 identity of the expected private text.

### 2. Keep comparison in the functional core

The pure reducer compares supplied command facts with current queue state. It returns changed state, a conflict, or a typed denial.

The shell persists an accepted transition and publishes events. It does not decide conflict semantics.

### 3. Keep conflict output safe

A conflict returns current generation, queue identity, safe entry identities, positions, and mutation status. It does not return queued text by default.

A client can request a fresh private snapshot through the existing authorized session API.

### 4. Limit compatibility behavior

A local single-owner convenience method can derive preconditions from a snapshot held under the same exclusive owner.

Headless, daemon, SDK, and TUI commands that cross an ownership boundary must supply explicit preconditions.

## Alternatives Considered

### Compare only the queue position

Rejected because insertions and removals can move another item into that position.

### Compare raw expected text in safe events

Rejected because queued input can contain private prompt material.

### Merge concurrent edits automatically

Rejected because the core cannot infer user intent safely.

## Risks and Controls

- Generations can wrap if poorly typed. Use a checked monotonic type and overflow denial.
- Text hashes can expose equality. Treat the identity as private command data and keep safe receipts domain-separated.
- Compatibility wrappers can hide races. Restrict them to one explicit exclusive owner.

## Non-Claims

This design does not establish distributed consensus, replicated queues, semantic merge correctness, availability, exactly-once delivery, or production readiness.
