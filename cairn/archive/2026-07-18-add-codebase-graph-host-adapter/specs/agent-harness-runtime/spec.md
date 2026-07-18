# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Codebase graph access uses an injected host service

r[agent_harness_runtime.codebase_graph_host.boundary] Animus MUST represent codebase-graph indexing and queries through explicit transport-neutral service contracts and bounded DTOs, while filesystem reads, watchers, parser dependencies, thread pools, mmap, cache files, and service lifecycle remain in an optional host-owned adapter.

#### Scenario: Runtime requests a definition query

- GIVEN a host injected an admitted codebase-graph service and runtime has a validated bounded query
- WHEN runtime dispatches the request
- THEN it MUST receive a typed bounded result or failure through the service contract
- AND reusable no-std/runtime code MUST NOT discover a repository, read files, start watchers, load caches, or construct parser workers.

#### Scenario: Service is absent

- GIVEN a context reference or tool requires codebase-graph resolution
- WHEN no admitted graph service is injected
- THEN dispatch MUST fail with a typed unavailable-service result
- AND it MUST NOT fall back to ambient filesystem scanning or an undeclared external command.

### Requirement: Index behavior is controlled by a typed profile

r[agent_harness_runtime.codebase_graph_host.profile] Codebase-graph hosts MUST consume a validated typed profile declaring language and query versions, capability-root labels, include/exclude policy, binary and special-file handling, finite file/count/byte/result budgets, event-overflow behavior, cache posture, redaction, and a canonical domain-separated BLAKE3 profile identity.

#### Scenario: Rust profile is accepted

- GIVEN a generated Rust index profile matches the reviewed Nickel source and all named bounds are finite and valid
- WHEN Rust revalidates the profile and recomputes its identity
- THEN the host MAY advertise the exact Rust language/query cohort
- AND it MUST NOT advertise another language or query version from parser availability alone.

#### Scenario: Profile is stale or unsafe

- GIVEN a profile has a stale hash, duplicate rule, unknown required language/query version, unsafe root label, missing bound, contradictory cache posture, or unsupported required field
- WHEN profile validation runs
- THEN it MUST fail closed before repository access or parser construction
- AND no compatible profile may be inferred from source order or ambient configuration.

### Requirement: Graph logic has a deterministic functional core

r[agent_harness_runtime.codebase_graph_host.pure_core] File-observation admission, parsed contribution projection, graph-delta application, stale-state decisions, query planning, stable result ordering, truncation, and safe projection MUST be deterministic functions over already-loaded bytes, validated profiles, prior graph state, and explicit events.

#### Scenario: Identical snapshot repeats

- GIVEN identical admitted relative file observations, parser/query profile, prior graph, and event sequence
- WHEN the graph core runs repeatedly
- THEN it MUST produce the same graph identity, ordered deltas, index status, and query results
- AND it MUST perform no filesystem, environment, process, network, clock, watcher, cache, or terminal effect.

### Requirement: Incremental indexing fails closed when state is incomplete

r[agent_harness_runtime.codebase_graph_host.incremental_index] The host adapter MUST support bounded initial indexing and explicit create, modify, delete, and rename reconciliation, and MUST mark the index stale and block trusted queries after watcher overflow, event gaps, ambiguous rename, required-read failure, root replacement, or unsupported event classes until a bounded rescan succeeds.

#### Scenario: File is modified

- GIVEN a fresh index and an admitted modify event with a newly loaded bounded file observation
- WHEN incremental reconciliation runs
- THEN the prior file contribution MUST be replaced deterministically
- AND subsequent queries MUST bind the new source and graph identities.

#### Scenario: Watcher loses events

- GIVEN the watcher reports overflow or a sequence gap
- WHEN the host processes that observation
- THEN the index MUST enter a stale state and trusted definition/reference queries MUST fail closed
- AND the host MAY return to fresh state only after a bounded rescan establishes a new source snapshot identity.

### Requirement: Codebase graph caches are derived and rebuildable

r[agent_harness_runtime.codebase_graph_host.cache] Any persisted codebase-graph cache MUST be treated as a disposable derived artifact bound by BLAKE3 to safe workspace, source snapshot, profile, parser, query, schema, and graph identities, and MUST NOT become authoritative source history or mutate repository files.

#### Scenario: Matching cache is loaded

- GIVEN a cache passes framing, schema, identity, budget, and safe-path validation for the current admitted inputs
- WHEN the host opens the graph service
- THEN it MAY load the derived graph and confirm freshness against current source evidence
- AND cache presence alone MUST NOT authorize repository access or a query.

#### Scenario: Cache is corrupt or stale

- GIVEN cache bytes are malformed, over budget, identity-mismatched, version-incompatible, path-unsafe, or stale for the current source snapshot
- WHEN cache validation runs
- THEN the host MUST quarantine/rebuild or return a typed unavailable result
- AND it MUST NOT answer from the rejected graph or modify source files.

### Requirement: Graph retrieval composes with existing admission

r[agent_harness_runtime.codebase_graph_host.retrieval] Definition, reference, symbol, and scope queries MUST pass existing context-reference or read-only tool admission, retrieval budgets, workspace capability bounds, result bounds, and redaction policy, and graph access MUST NOT grant file mutation, VCS, process, network, or tool authority.

#### Scenario: Bounded context expansion succeeds

- GIVEN an admitted symbol reference, a fresh graph, and a retrieval budget that permits bounded source locations
- WHEN the graph resolver answers the query
- THEN it MUST return stable repository-relative locations with language/query/profile/source/index identities and truncation metadata
- AND any content read for expansion MUST pass the existing context resolver separately.

#### Scenario: Result would exceed budget

- GIVEN a symbol has more matching references than the admitted result budget
- WHEN query projection runs
- THEN the result MUST be deterministically truncated or rejected according to profile
- AND receipts MUST retain total/visible counts and graph identity without embedding omitted source content.

### Requirement: Graph evidence is safe and syntactic

r[agent_harness_runtime.codebase_graph_host.receipts] Codebase-graph receipts MUST contain only safe service/profile/language/query/status labels, workspace/source/index/cache identities, bounded file/symbol/result counts, freshness and truncation state, redaction markers, syntactic-evidence non-claims, and a domain-separated BLAKE3 receipt identity.

#### Scenario: Query receipt is emitted

- GIVEN a fresh graph answers a bounded definition or reference query
- WHEN receipt projection runs
- THEN it MUST identify the exact profile, parser/query cohort, source snapshot, graph, status, and bounded counts
- AND it MUST omit source bodies, prompts, credentials, physical paths, raw cache bytes, parser diagnostics, tool payloads, provider payloads, and unbounded spans.

### Requirement: Codebase graph behavior has conformance evidence

r[agent_harness_runtime.codebase_graph_host.verification] The codebase-graph host change MUST include positive and negative fixtures for profile validation, Rust parsing/navigation, initial and incremental indexing, stale recovery, cache identity, capability-root safety, retrieval/tool integration, dependency isolation, bounded results, syntactic non-claims, and receipt redaction.

#### Scenario: Change is ready to archive

- GIVEN maintainers intend to sync and archive the codebase-graph host change
- WHEN closeout validation runs
- THEN focused contract, profile, parser, graph, event, cache, context, tool, receipt, Cairn, and relevant Nix checks MUST pass
- AND unsupported languages and unavailable optional live watcher checks MUST remain explicit non-claims or blockers.
