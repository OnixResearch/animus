# Agent Harness Runtime

## ADDED Requirements

### Requirement: Universal Weave remains a pinned optional boundary

r[agent_harness_runtime.universal_weave.boundary] Animus MUST adapt `universal-weave` branching through a non-default adapter crate pinned to an exact reviewed crate version with default Cargo features disabled, and reusable, default, no-std, and Wasm-supported surfaces MUST NOT acquire `universal-weave`, `rkyv`, `loro`, `serde`-lane, filesystem, process, network, credential, clock, terminal, provider, or persistence dependencies.

#### Scenario: Exact pinned cohort is selected

r[agent_harness_runtime.universal_weave.boundary.exact_cohort]
- GIVEN a host enables the reviewed `animus-adapter-weave` crate pinned to `universal-weave` `=0.4.1` with default features disabled
- WHEN the optional adapter is composed
- THEN Animus MAY admit DAG weave operations behind Animus-owned DTOs
- AND default or reusable builds MUST NOT require the upstream crate.

#### Scenario: Excluded upstream lane is requested

r[agent_harness_runtime.universal_weave.boundary.excluded_lane]
- GIVEN a caller requests the tree weave, the loro CRDT lane, rkyv or serde serialization, the dedup wrapper, or the undo wrapper
- WHEN adapter admission runs
- THEN Animus MUST return a typed unsupported-surface outcome or fail to compile
- AND it MUST NOT silently widen authority, serialization, or compatibility claims.

### Requirement: Weave adapter core is pure and mutable-model only

r[agent_harness_runtime.universal_weave.adapter_core] The adapter MUST wrap the DAG `IndependentWeave` over an Animus-owned text content type with validated add, edit, split, merge-with-parent, move, bookmark, activation, and removal operations, and reusable logic MUST NOT read files, inspect environment, spawn processes, access clocks, perform network I/O, mutate the neutral session ledger, or expose upstream types in its public API.

#### Scenario: Valid operation mutates the weave

r[agent_harness_runtime.universal_weave.adapter_core.valid_operation]
- GIVEN a constructed adapter and a request that passes policy admission with existing parent identifiers and fresh node identifiers
- WHEN the operation runs
- THEN the adapter MUST apply it through the upstream weave and report the resulting bounded counts
- AND the operation MUST NOT touch any session ledger, persistence, provider, filesystem, or clock.

#### Scenario: Operation targets missing or stale identifiers

r[agent_harness_runtime.universal_weave.adapter_core.stale_target]
- GIVEN a request that names a missing node, a duplicate node identifier, a missing parent, or a split point outside the content
- WHEN the operation runs
- THEN the adapter MUST fail closed with a typed error
- AND the weave MUST remain unchanged.

### Requirement: Weave bounds are named policy data enforced before mutation

r[agent_harness_runtime.universal_weave.bounds] The adapter MUST define a typed policy with named bounds for maximum node count, maximum content bytes, and maximum parents per node, MUST validate the policy at construction, and MUST revalidate every request against the policy before any weave mutation.

#### Scenario: Bounded request is admitted

r[agent_harness_runtime.universal_weave.bounds.valid]
- GIVEN a policy with finite positive bounds and a request within every bound
- WHEN admission runs
- THEN the adapter MUST admit the request deterministically
- AND admission MUST NOT perform any weave mutation by itself.

#### Scenario: Request or policy exceeds a bound

r[agent_harness_runtime.universal_weave.bounds.exceeded]
- GIVEN a zero policy bound, content longer than the byte bound, more parents than the fan-in bound, or a weave already at the node bound
- WHEN construction or admission runs
- THEN validation MUST fail closed with a typed error naming the violated bound
- AND no weave state may change.

### Requirement: Weave evidence is metadata-only with BLAKE3 identity

r[agent_harness_runtime.universal_weave.evidence] Adapter receipts and snapshots MUST bind operation class, target identifiers, bounded node, root, active, and bookmark counts, policy identity, content lengths, content BLAKE3 digests, and a domain-separated BLAKE3 receipt identity, and MUST exclude raw node text, raw prompts, provider payloads, credentials, full paths, and host store material.

#### Scenario: Receipt proves a bounded operation fact

r[agent_harness_runtime.universal_weave.evidence.receipt]
- GIVEN a completed admitted operation
- WHEN receipt projection runs
- THEN the receipt MUST contain only safe operation metadata, bounded counts, and a lowercase BLAKE3 hex identity
- AND equal operation sequences MUST produce equal receipt identities.

#### Scenario: Evidence leaks raw content

r[agent_harness_runtime.universal_weave.evidence.leak]
- GIVEN any attempted receipt or snapshot containing raw node text, prompt bodies, provider payloads, credentials, or full paths
- WHEN evidence validation runs
- THEN validation MUST reject the material
- AND no durable evidence may be emitted.

### Requirement: Weave adapter verification covers positive and negative paths

r[agent_harness_runtime.universal_weave.verification] The adapter MUST ship paired positive and negative tests for every operation class, bound, and evidence rule, a focused verification rail, component catalog registration, and documentation with explicit non-claims.

#### Scenario: Focused rail validates the boundary

r[agent_harness_runtime.universal_weave.verification.rail]
- GIVEN the adapter crate, its tests, catalog row, and docs
- WHEN the focused rail runs
- THEN it MUST verify the exact pin, disabled default features, no_std and unsafe-forbid markers, forbidden-authority absence, and registration metadata
- AND it MUST fail closed on drift.

#### Scenario: Default and Wasm surfaces stay clean

r[agent_harness_runtime.universal_weave.verification.isolation]
- GIVEN a default workspace build or a Wasm-supported surface check
- WHEN dependencies resolve
- THEN `universal-weave` MUST appear only behind the opt-in crate
- AND the supported Wasm surface MUST remain unchanged unless separate evidence promotes the adapter crate.
