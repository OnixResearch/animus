## Goals

- Add a `no_std` retention decision core to the Animus core crate.
- Add a bounded store in the Animus std shell.
- Key every entry by BLAKE3 digest.
- Keep every bound explicit.
- Release results only after the synchronization boundary.
- Leave tool semantics with the tool adapters.

## Core contract

The core accepts supplied values: capacity, active count, LRU and free-list
state, generation values, promotion threshold, and watermarks.

It returns deterministic decisions: eviction target, promotion disposition,
trim plan, free-list transition, and deferred-release plan.

It reads no clock, memory, or scheduler state.

## Key projection

The cache key is the BLAKE3 digest of a canonical projection over the tool
request identity, the normalized arguments, and the capability-context
projection when the adapter supplies one. The projection excludes payload
bytes that do not change the result.

## Retention policy

- Strict LRU at threshold 0; FIFO at threshold 100; lazy promotion in between.
- Promotion and eviction are deterministic over supplied values.
- Watermark trimming uses explicit high and low watermarks.
- The core returns a release plan. The shell releases after the guard.

## Shell contract

The shell owns the bounded store, synchronization, counters, and release
execution. It never decides policy.

The store can later adopt a published `bounded-cache` revision without changing
the core contract.

## Verification

Positive coverage includes projection stability, bounded retention, eviction
order, promotion at and below threshold, and deferred-release ordering.

Negative coverage includes missing explicit bounds, reversed watermarks, and
boundary over-claims.

Boundary coverage includes zero capacity, single slot, threshold 0 and 100.
