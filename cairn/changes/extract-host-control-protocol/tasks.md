## Phase 1: Boundary and mapping inventory

- [x] [serial] Inventory Animus and Tile topology, identity, revision, bounded-read, status, layout, event, stream, wait, error, feature, wire, and retained-authority surfaces. r[agent_harness_runtime.host_control.repository_extraction.inventory]
- [x] [serial] Define explicit Animus and Tile mapping tables for resource kinds, native IDs, selector lifetime, status vocabulary, observation modes, timeout units, stream features, errors, and unsupported operations. r[agent_harness_runtime.host_control.repository_extraction.mapping_profiles]
- [x] [serial] Freeze positive and negative cross-consumer fixtures before moving semantic ownership. r[agent_harness_runtime.host_control.repository_extraction.conformance]

## Phase 2: Standalone repository

- [ ] [serial] Create `OnixResearch/host-control` with independent README, license, typed Nickel repository configuration, native Cairn lifecycle, release profile, and no Animus, Tile, or sibling-path dependency. r[agent_harness_runtime.host_control.repository_extraction.repository]
- [x] [serial] Extract `host-control-core` as a deterministic `#![no_std]`/`alloc` functional core with no filesystem, process, environment, network, clock, terminal, daemon, UI, or persistence authority. r[agent_harness_runtime.host_control.repository_extraction.functional_core]
- [x] [depends:agent_harness_runtime.host_control.repository_extraction.functional_core] Implement the mandatory selector/topology/bounded-observation/event/wait/error/feature core and optional layout and snapshot-first stream profiles. r[agent_harness_runtime.host_control.repository_extraction.canonical_contract]
- [ ] [depends:agent_harness_runtime.host_control.repository_extraction.functional_core] Add optional serde and Preserves adapters that revalidate decoded DTOs and do not introduce transport or daemon authority. r[agent_harness_runtime.host_control.repository_extraction.serialization]
- [x] [parallel] Add positive fixtures for Animus topology, bounded reads, layouts, events/waits, envelopes, and Tile stable IDs, revision-scoped indices, bounded pane observation, agent status, and snapshot-first streams. r[agent_harness_runtime.host_control.repository_extraction.conformance.positive]
- [x] [parallel] Add negative fixtures for stale/cross-host selectors, unscoped indices, duplicate labels, unbounded reads, unsafe output, zero or excessive timeouts, lossy status coercion, incremental-before-snapshot, unsupported versions/features, raw PTY requests, and malformed wire payloads. r[agent_harness_runtime.host_control.repository_extraction.conformance.negative]

## Phase 3: Animus cutover

- [ ] [serial] Pin Animus to one immutable standalone revision and validate source/lock identity, package uniqueness, and the absence of workspace-relative release dependencies. r[agent_harness_runtime.host_control.repository_extraction.exact_source]
- [ ] [depends:agent_harness_runtime.host_control.repository_extraction.exact_source] Delegate current `animus-adapters` host-control semantics to the standalone core while preserving existing public imports through a compatibility facade. r[agent_harness_runtime.host_control.repository_extraction.animus_cutover]
- [ ] [depends:agent_harness_runtime.host_control.repository_extraction.animus_cutover] Prove the compatibility facade contains no forked selector, topology, bounded-output, layout, event, wait, or envelope semantics. r[agent_harness_runtime.host_control.repository_extraction.compatibility_facade]
- [ ] [parallel] Keep std host-adapter authority crates and Preserves bridge-family work separate, updating their catalogs only to point at the new semantic owner. r[agent_harness_runtime.host_control.repository_extraction.active_change_boundary]

## Phase 4: Tile handoff and verification

- [ ] [serial] Publish a Tile mapping/conformance profile and create a Tile-owned Cairn dependency for adapter implementation; do not move Tile daemon, PTY, socket, persistence, policy, rendering, or native workspace state into the standalone repository. r[agent_harness_runtime.host_control.repository_extraction.tile_handoff]
- [ ] [serial] Run standalone host/wasm tests, serialization adapter tests, positive and negative cross-consumer fixtures, mapping-loss checks, stream-order tests, package metadata checks, and source-pin checks. r[agent_harness_runtime.host_control.repository_extraction.validation]
- [ ] [serial] Run focused Animus host-control and TUI facade tests, adapter dependency-isolation checks, active-change boundary checks, workspace checks, `nix flake check`, Cairn validation, and proposal/design/tasks gates before sync and archive. r[agent_harness_runtime.host_control.repository_extraction.validation]
