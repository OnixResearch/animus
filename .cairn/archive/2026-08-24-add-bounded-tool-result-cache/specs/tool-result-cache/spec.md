# Animus Tool Result Cache

## ADDED Requirements

### Requirement: Canonical key projection

r[animus.cache.projection] The cache MUST key every entry by the BLAKE3 digest of a canonical projection over tool request identity, normalized arguments, and optional capability context. The projection MUST exclude payload bytes that do not change the result.

#### Scenario: Same request projects to the same key

r[animus.cache.projection.equal]
- GIVEN two requests with equal tool identity, normalized arguments, and context
- WHEN the cache projects both keys
- THEN the keys MUST be equal

#### Scenario: Distinct requests project to distinct keys

r[animus.cache.projection.distinct]
- GIVEN two requests whose tool identity or arguments differ
- WHEN the cache projects both keys
- THEN the keys MUST differ

### Requirement: Explicit capacity

r[animus.cache.bound] The cache MUST require an explicit capacity. The cache MUST NOT exceed that capacity and MUST NOT assume an implicit default bound.

#### Scenario: Operator configures a capacity

r[animus.cache.bound.configured]
- GIVEN an operator supplies an explicit capacity
- WHEN the cache is built
- THEN the cache MUST retain at most that many entries

#### Scenario: Insertion at capacity

r[animus.cache.bound.full]
- GIVEN the cache is at capacity
- WHEN an insertion requires a slot
- THEN the cache MUST evict by policy
- AND the entry count MUST stay at or below capacity

### Requirement: Deterministic retention decisions

r[animus.cache.decision] The `no_std` decision core MUST decide eviction and promotion from supplied values only. It MUST read no clock, memory, or scheduler state. The promotion decision MUST honor an explicit threshold where 0 means strict LRU and 100 means FIFO.

#### Scenario: Strict LRU retention

r[animus.cache.decision.strict_lru]
- GIVEN a threshold of 0 and an entry now qualifies as most recently used
- WHEN the core evaluates promotion
- THEN the entry MUST promote

#### Scenario: FIFO retention

r[animus.cache.decision.fifo]
- GIVEN a threshold of 100 and any entry age
- WHEN the core evaluates promotion
- THEN the entry MUST NOT promote

### Requirement: Deferred release ordering

r[animus.cache.deferral] The core MUST return release plans so the shell drops results after it releases the guard. The shell MUST NOT run user destructors inside the synchronization boundary.

#### Scenario: Result evicted under contention

r[animus.cache.deferral.evicted]
- GIVEN a result is evicted while other threads wait
- WHEN the shell processes the release plan
- THEN the guard MUST be released first
- AND the result MUST drop only after release

### Requirement: Adapter authority

r[animus.cache.boundary] A cache hit MUST NOT change tool semantics. The cache MUST NOT claim that a hit proves tool correctness or context freshness. The cross-repo retention contract MUST be recorded in repo docs.

#### Scenario: Over-claim of cache authority

r[animus.cache.boundary.overclaim]
- GIVEN a consumer claims that a cache hit proves tool correctness
- WHEN the claim boundary is verified
- THEN the claim MUST fail unless the adapter owns it

### Requirement: Fixture coverage

r[animus.cache.verification] Positive, negative, and boundary fixtures MUST cover projection stability, bounded retention, retention decisions, deferred release, and the claim boundary.

#### Scenario: Complete focused matrix passes

r[animus.cache.verification.matrix]
- GIVEN decisions, fixtures, documentation, and dependency evidence are complete
- WHEN focused workspace, Clippy, Cairn, and Nix verification runs
- THEN in-bounds inputs MUST behave as declared
- AND every missing-bound, reversed-watermark, boundary, or over-claim input MUST fail as declared
