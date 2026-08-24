# Bounded tool-result cache

Animus provides an advisory cache for repeated tool requests. The cache does not change tool meaning or grant tool authority.

## Core and shell boundary

`animus-core::retention` owns deterministic policy:

- BLAKE3 keys over a tagged canonical request projection
- explicit capacity and watermarks
- insertion and eviction decisions
- strict LRU, FIFO, and lazy promotion decisions
- deferred-release plans

The core reads no clock, process state, scheduler state, or memory pressure.

`animus_std::retentionhost` owns synchronization, values, counters, and release execution. It drops evicted values only after it releases the synchronization guard.

## Key projection

Adapters must supply these fields:

1. stable tool identity
2. normalized arguments
3. optional capability-context projection

Adapters must exclude payload bytes that do not change the result. The cache does not normalize arguments or decide which capability facts are relevant.

## Explicit bounds

Construction requires `Some(capacity)`. There is no default capacity.

An explicit capacity of zero creates a pass-through cache. Misses still use the existing adapter path, and no result is retained.

The low watermark must not exceed the high watermark. The high watermark must not exceed capacity. When entry count exceeds the high watermark, the core plans removal down to the low watermark.

## Retention modes

A promotion threshold of zero gives strict LRU behavior. Every hit promotes the entry.

A threshold of 100 gives FIFO behavior. Hits do not change insertion order.

Intermediate thresholds use the supplied relative age. A hit promotes at or above the threshold.

## Claim boundary

A cache hit proves only that this process retained a value under the configured policy and canonical key.

A cache hit does not prove:

- tool correctness
- result freshness
- capability validity
- authorization
- exactly-once execution
- durable retention
- production readiness
- release eligibility

The tool adapter still owns request semantics, execution authority, result validation, and freshness policy.

## Cross-repository contract

The policy follows the proposed OnixResearch `bounded-cache` retention contract. No published immutable revision was available during this change. Animus therefore keeps the contract local and adds no sibling-path dependency.

The retention design also considers the Apache-2.0 `adanil-code/LRUHashTable` project as a design reference. Animus does not copy its implementation.
