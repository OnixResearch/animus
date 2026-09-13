# Animus Ranked Retrieval

## ADDED Requirements

### Requirement: Ranked retrieval is authority-free

r[animus.ranked_retrieval.boundary] The ranked-retrieval family MUST validate ranked-query, route, fusion, index-state, budget, limit, candidate, evidence, and receipt facts without model, tool, process, filesystem, network, clock, terminal, provider, persistence, or product mutation effects.

#### Scenario: Supplied ranked facts are validated

- GIVEN a consumer supplies in-memory postings, term, route, fusion, budget, and index-state values
- WHEN the family validates them
- THEN it MUST return deterministic structural facts or typed issues
- AND it MUST perform no host effect.

### Requirement: Ranked retrieval uses typed bounded values

r[animus.ranked_retrieval.model] The family MUST define opaque ranked-query, route, fusion-policy, index-state, budget, limit, ranked-candidate, evidence, and metadata-only receipt values.

#### Scenario: Complete ranked query is represented

- GIVEN a bounded ranked query binds one route, one fusion policy, finite limits, and bounded candidate facts
- WHEN model validation runs
- THEN the family MUST admit the typed structure for evaluation
- AND no value may contain a service handle or product payload.

#### Scenario: Duplicate candidate is rejected

- GIVEN a ranked candidate set repeats one source location
- WHEN model validation runs
- THEN the family MUST reject the set
- OR MUST bind the duplicate under an explicit rule.

### Requirement: Ranking is deterministic and bounded

r[animus.ranked_retrieval.retrieval] BM25 scoring and reciprocal rank fusion MUST be deterministic over supplied safe values. Missing postings, missing terms, and over-limit candidate sets MUST fail closed.

#### Scenario: Scoring is deterministic

- GIVEN the same postings, term, route, and fusion facts
- WHEN scoring and fusion run twice
- THEN the family MUST return identical ranked candidates and identities.

#### Scenario: Over-limit candidate set is rejected

- GIVEN a candidate set exceeds the declared limit
- WHEN ranking runs
- THEN the family MUST reject the set
- AND it MUST NOT silently truncate without a typed result.

### Requirement: Routes fail closed

r[animus.ranked_retrieval.routes] Route selection MUST accept only exact, ranked-lexical, vector, and hybrid routes. A hybrid route MUST declare an explicit fusion policy. An unknown or undeclared route MUST fail closed.

#### Scenario: Hybrid route declares fusion policy

- GIVEN a hybrid route cites an explicit fusion policy and bounded candidate groups
- WHEN route validation runs
- THEN the family MUST admit the fusion plan.

#### Scenario: Unknown route is denied

- GIVEN a request names no known route and no fallback
- WHEN route validation runs
- THEN the family MUST fail closed
- AND it MUST NOT assume an implicit route.

### Requirement: Stale index facts block ranked answers

r[animus.ranked_retrieval.freshness] The family MUST admit only explicit current index-state facts. A possibly-stale verdict MUST block ranked answers and MAY name an exact-route fallback or a bounded rebuild.

#### Scenario: Fresh index admits ranked query

- GIVEN a request carries a fresh index-state verdict
- WHEN the family evaluates the request
- THEN the family MUST admit the ranked query.

#### Scenario: Stale index blocks ranked answers

- GIVEN a request carries a possibly-stale index-state verdict
- WHEN the family evaluates the request
- THEN the family MUST block ranked answers
- AND it MAY name the exact-route fallback or a bounded rebuild.

### Requirement: Index admission is profile-policy driven

r[animus.ranked_retrieval.admission] Type-aware admission limits and binary detection MUST be typed profile inputs. Facts outside the profile limits MUST be rejected without implicit defaults.

#### Scenario: Oversize fact is rejected

- GIVEN a fact exceeds the declared type-aware limit in the profile
- WHEN admission runs
- THEN the family MUST reject the fact
- AND it MUST NOT apply a hidden default limit.

#### Scenario: Profile is missing

- GIVEN no admission profile is supplied
- WHEN admission runs
- THEN the family MUST fail closed.

### Requirement: Agent evidence is compact and source-linked

r[animus.ranked_retrieval.evidence] Evidence output MUST be file-oriented and source-linked. Previews MUST be omitted unless requested. Receipts MUST remain metadata-only and MUST NOT embed unbounded source bodies.

#### Scenario: Compact evidence without preview

- GIVEN a consumer requests ranked evidence without a preview
- WHEN the evidence projection runs
- THEN the output MUST contain source-linked locations and no source bodies.

#### Scenario: Receipt omits raw bodies

- GIVEN a ranked query completes
- WHEN the metadata-only receipt is formed
- THEN the receipt MUST contain counts, safe labels, and BLAKE3 identities
- AND it MUST NOT embed the source bodies.

### Requirement: Exact path remains authoritative

r[animus.ranked_retrieval.exact] Ranked results MUST NOT suppress or reorder exact-route matches for known symbols. The exact codebase-graph and managed-ripgrep paths remain unchanged and authoritative.

#### Scenario: Exact match is not suppressed

- GIVEN an exact-route match exists for a known symbol
- WHEN ranked output is formed
- THEN the exact match MUST remain present and source-linked.

### Requirement: Consumers retain product ownership

r[animus.ranked_retrieval.consumers] Consumers MUST retain index construction, storage, refresh, embedder choice, execution, and effect authority. The family MUST gain no search-side runtime authority.

#### Scenario: Codebase tool host admits ranked query

- GIVEN a read-only codebase tool host supplies bounded ranked-query facts
- WHEN the mapping fixture runs
- THEN structural admission MAY succeed
- AND the family MUST gain no execution, mutation, or storage authority.

#### Scenario: Retrieval budgets apply unchanged

- GIVEN a retrieval-capable resource carries a validated budget
- WHEN ranked retrieval is dispatched
- THEN the budget MUST apply before graph or ranked dispatch
- AND stale indexes MUST NOT spend additional retrieval budget implicitly.

### Requirement: Ranked retrieval has positive and negative verification

r[animus.ranked_retrieval.verification] The focused rail MUST cover valid ranked queries, exact-route passthrough, hybrid fusion, fresh index state, codecs, and consumer mappings, plus negative missing-postings, unknown-route, missing-fusion, stale, over-limit, malformed, and authority-overclaim cases.

#### Scenario: Ranked-retrieval rail passes

- GIVEN all required no-std, Wasm, codec, consumer, positive, and negative fixtures are present
- WHEN the focused rail runs
- THEN structural and ranking decisions MUST be deterministic
- AND invalid facts MUST fail without effects or promoted claims.
