# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Cross-consumer inventory precedes extraction

r[agent_harness_runtime.host_control.repository_extraction.inventory] Animus MUST record a reviewed inventory of host-control semantics in Animus and Tile before selecting a standalone protocol boundary.

#### Scenario: Inventory separates semantics from product authority

r[agent_harness_runtime.host_control.repository_extraction.inventory.boundary]
- GIVEN Animus and Tile both expose topology, observation, status, event, and wait behavior through different product types
- WHEN the extraction inventory is reviewed
- THEN it MUST identify shared semantic DTOs and validation decisions
- AND it MUST identify sockets, PTYs, processes, persistence, clocks, policy, rendering, daemon state, and product-native IDs that remain consumer-owned.

### Requirement: Standalone repository owns the protocol contract

r[agent_harness_runtime.host_control.repository_extraction.repository] Animus MUST move canonical transport-neutral host-control contract ownership to an independent `OnixResearch/host-control` repository with its own lifecycle, release profile, documentation, and source identity.

#### Scenario: Standalone package is product-neutral

r[agent_harness_runtime.host_control.repository_extraction.repository.independent]
- GIVEN the standalone package is prepared for release
- WHEN its package graph and public vocabulary are inspected
- THEN it MUST NOT depend on Animus runtime or UI crates, Tile core/protocol/daemon crates, a terminal library, a transport runtime, or workspace-relative sibling paths
- AND its release metadata MUST identify canonical source and verification entrypoints.

### Requirement: Protocol decisions use a functional core

r[agent_harness_runtime.host_control.repository_extraction.functional_core] The standalone repository MUST provide a deterministic `#![no_std]`/`alloc` functional core and MUST keep filesystem, process, environment, network, clock, terminal, daemon, UI, and persistence effects outside that core.

#### Scenario: Wait resolution needs no running host

r[agent_harness_runtime.host_control.repository_extraction.functional_core.in_memory]
- GIVEN a validated wait request and a finite supplied event sequence
- WHEN the core resolves the wait
- THEN it MUST return a deterministic matched, timeout, cancelled, or host-error outcome without opening a socket, reading a clock, or inspecting a process.

#### Scenario: Ambient authority in the core is rejected

r[agent_harness_runtime.host_control.repository_extraction.functional_core.purity]
- GIVEN a core change introduces host I/O, clock access, terminal rendering, printing, async runtime state, or hidden global state
- WHEN the core purity rail runs
- THEN validation MUST fail before release acceptance.

### Requirement: Canonical protocol has a small feature-negotiated base

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract] The standalone core MUST define versioned contracts for opaque selectors, explicit lifetime and revision, topology snapshots, bounded observations, semantic events and waits, typed errors, and feature negotiation, while optional layout and snapshot-first stream behavior MUST require declared features.

#### Scenario: Unsupported feature fails before execution

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract.feature_gate]
- GIVEN a command requests layout application or snapshot-first streaming from a host that did not advertise that feature
- WHEN command admission runs
- THEN it MUST return a typed unsupported-feature result
- AND no consumer mutation or stream registration may be inferred.

#### Scenario: Positional reference is revision-scoped

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract.scoped_index]
- GIVEN a positional pane index was produced for one session, tab, and topology revision
- WHEN it is reused under a different scope or revision
- THEN validation MUST reject it as stale or cross-scope
- AND it MUST NOT treat the index as durable identity.

### Requirement: Consumer mappings are explicit and loss-aware

r[agent_harness_runtime.host_control.repository_extraction.mapping_profiles] The standalone repository MUST define reviewed Animus and Tile mapping profiles for native IDs, resource kinds, selector lifetime, status vocabulary, observation modes, timeout units, stream features, errors, and unsupported operations.

#### Scenario: Status mismatch is not silently coerced

r[agent_harness_runtime.host_control.repository_extraction.mapping_profiles.status_loss]
- GIVEN one consumer supplies `waiting` or `failed` and the target profile has only `blocked`, `working`, `done`, `idle`, or `unknown`
- WHEN status mapping runs
- THEN the mapping MUST preserve the distinction, return a declared lossy-mapping diagnostic, or reject the value
- AND it MUST NOT silently turn failure, waiting, or blocked into successful completion.

### Requirement: Bounded observation stays explicit

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract.bounded_observation] Host-control observation MUST carry finite line and byte budgets, source class, truncation status, redaction status, measured counts, and optional BLAKE3 identity metadata rather than exposing unbounded terminal transcripts.

#### Scenario: Raw or oversized observation fails closed

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract.bounded_observation.rejects_unbounded]
- GIVEN a request has no finite budget, exceeds the supported profile, requests unsafe raw PTY access, or contains an unsafe output reference
- WHEN observation admission runs
- THEN it MUST fail with a typed bounded-observation or unsafe-output diagnostic
- AND it MUST NOT return an accepted observation DTO.

### Requirement: Snapshot-first streams preserve ordering

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract.snapshot_first] A host advertising snapshot-first streams MUST provide one accepted redaction-safe initial snapshot before incremental updates for each stream identity.

#### Scenario: Incremental event before snapshot is rejected

r[agent_harness_runtime.host_control.repository_extraction.canonical_contract.snapshot_first.order]
- GIVEN a stream has not accepted an initial snapshot
- WHEN an incremental update is validated for that stream
- THEN validation MUST reject the update with a deterministic order diagnostic
- AND it MUST NOT fabricate local state or claim raw PTY coverage.

### Requirement: Serialization remains an adapter

r[agent_harness_runtime.host_control.repository_extraction.serialization] Serde and Preserves support MAY be provided as optional adapters, but decoded payloads MUST be revalidated by the semantic core and serialization code MUST NOT introduce socket, daemon, process, terminal, or persistence authority.

#### Scenario: Malformed decoded DTO is rejected

r[agent_harness_runtime.host_control.repository_extraction.serialization.revalidate]
- GIVEN JSON or Preserves bytes decode structurally but contain an unsupported version, unknown required feature, invalid selector, unbounded read, or invalid timeout
- WHEN adapter revalidation runs
- THEN the payload MUST be rejected before a consumer command is admitted.

### Requirement: Conformance includes both consumers and failure cases

r[agent_harness_runtime.host_control.repository_extraction.conformance] The standalone repository MUST maintain a fixture-backed conformance corpus for Animus and Tile mappings with positive and negative protocol behavior.

#### Scenario: Supported consumer profiles pass

r[agent_harness_runtime.host_control.repository_extraction.conformance.positive]
- GIVEN reviewed Animus topology, observation, layout, event, wait, and envelope fixtures plus Tile stable-ID, scoped-index, pane-read, status, and stream fixtures
- WHEN standalone conformance validation runs
- THEN each supported mapping and expected canonical result MUST pass deterministically.

#### Scenario: Invalid or unsafe behavior fails closed

r[agent_harness_runtime.host_control.repository_extraction.conformance.negative]
- GIVEN a stale or cross-host selector, unscoped index, duplicate label, unbounded read, unsafe output, invalid timeout, lossy status coercion, incremental-before-snapshot update, unsupported version or feature, raw PTY request, or malformed wire payload
- WHEN standalone conformance validation runs
- THEN it MUST fail with deterministic diagnostics naming the applicable failure classes
- AND it MUST NOT emit an admitted command or successful wait result.

### Requirement: Animus pins one canonical source

r[agent_harness_runtime.host_control.repository_extraction.exact_source] Animus MUST bind dependency and cutover evidence to one immutable standalone source coordinate and exact revision, and its dependency manifest and lockfile MUST agree on that identity.

#### Scenario: Floating or duplicate canonical source fails

r[agent_harness_runtime.host_control.repository_extraction.exact_source.rejects_drift]
- GIVEN the dependency is branch-only, tag-only, a release-local override, lock-drifted, or duplicated under the canonical package identity from a different source
- WHEN Animus validates cutover readiness
- THEN cutover MUST fail and name the conflicting or non-immutable source identity.

### Requirement: Animus delegates through a compatibility facade

r[agent_harness_runtime.host_control.repository_extraction.animus_cutover] Animus MUST delegate canonical selector, topology, bounded-observation, layout, event, wait, error, and envelope semantics to the pinned standalone core while preserving current public imports during the migration window.

#### Scenario: Existing public import remains compatible

r[agent_harness_runtime.host_control.repository_extraction.animus_cutover.public_path]
- GIVEN an existing consumer imports host-control items from `animus-adapters`
- WHEN the standalone dependency is introduced
- THEN representative existing imports MUST continue to compile through explicit aliases, conversions, or re-exports
- AND new code SHOULD be directed to the smallest standalone package it needs.

### Requirement: Compatibility facade contains no semantic fork

r[agent_harness_runtime.host_control.repository_extraction.compatibility_facade] The Animus compatibility facade MUST NOT retain independently evolving selector, topology, observation, layout, event, wait, error, feature, or envelope business logic.

#### Scenario: Duplicate core implementation fails ownership checks

r[agent_harness_runtime.host_control.repository_extraction.compatibility_facade.no_fork]
- GIVEN semantic logic is added back to the Animus facade instead of the standalone core or an Animus-specific mapping adapter
- WHEN ownership and dependency-isolation checks run
- THEN validation MUST fail and identify the duplicated semantic surface.

### Requirement: Existing Animus splits remain separate

r[agent_harness_runtime.host_control.repository_extraction.active_change_boundary] The extraction MUST keep authority-scoped std host adapters and Preserves bridge-family work separate from standalone semantic protocol ownership.

#### Scenario: Extraction does not absorb active adapter work

r[agent_harness_runtime.host_control.repository_extraction.active_change_boundary.no_overlap]
- GIVEN std host-adapter or Preserves bridge changes are active
- WHEN the host-control extraction inventory and dependency graph are reviewed
- THEN filesystem, provider, credential, network, persistence, terminal, and encoding authority MUST remain in those adapter boundaries
- AND the standalone core MUST contain only semantic DTOs and deterministic validation.

### Requirement: Tile adoption remains Tile-owned

r[agent_harness_runtime.host_control.repository_extraction.tile_handoff] Tile MUST adopt the standalone contract only through a Tile-owned Cairn change that preserves Tile native IDs, daemon policy, sockets, PTYs, persistence, rendering, workspace state, and rollback behavior.

#### Scenario: Standalone extraction does not move Tile authority

r[agent_harness_runtime.host_control.repository_extraction.tile_handoff.separate_change]
- GIVEN the standalone package and Animus cutover are ready
- WHEN Tile adoption is planned
- THEN the Tile repository MUST own its mapping adapter, exact pin, compatibility evidence, and rollback
- AND no Tile daemon or host authority may move into the standalone repository as an implicit dependency.

### Requirement: Extraction validation remains bounded

r[agent_harness_runtime.host_control.repository_extraction.validation] Extraction validation MUST prove deterministic DTO validation, exact source ownership, selected Animus and Tile mapping compatibility, bounded observation, and declared stream ordering only.

#### Scenario: Passing conformance avoids overclaims

r[agent_harness_runtime.host_control.repository_extraction.validation.non_claims]
- GIVEN standalone and Animus cutover validation passes
- WHEN an operator reviews the result
- THEN it MUST state that conformance does not prove daemon correctness, transport confidentiality or authentication, process or PTY preservation, terminal-output truth, policy authorization, arbitrary host compatibility, or whole-product readiness.
