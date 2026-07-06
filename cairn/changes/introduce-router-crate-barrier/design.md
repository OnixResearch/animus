# Design: Introduce the animus-router crate barrier

## Context

`animus-adapters::router` currently mixes two concerns: implementation-free adapter contracts (the shared field-validation helpers) and routing *decision* logic (`select_recipe`, `preferred_family`, `select_entries`, BLAKE3 `hash_parts`). Animus keeps each reusable boundary in its own crate so embedders depend on the smallest surface and so purity rails can target one crate. The router module has crossed the threshold where it deserves its own barrier.

## Decisions

### 1. animus-router owns the routing decision, not adapter contracts

**Choice:** The new `crates/animus-router` crate owns recipe/role/output-contract DTOs, signal projection, recipe selection, execution-plan materialization, route-result aggregation, and safe receipt assembly. `animus-adapters` keeps the shared `validation` helpers and model/tool/port contracts the router consumes.

**Rationale:** Routing decisions are a separable pure core that embedders may want without pulling all adapter contracts. Keeping the validation helpers in adapters preserves a one-way dependency: `animus-router` depends on `animus-adapters`, never the reverse.

### 2. no_std + alloc, matching animus-core and animus-adapters

**Choice:** `animus-router` is `#![no_std]` with `alloc` only and reuses `animus-adapters`' BLAKE3 dependency through a shared workspace crate.

**Rationale:** The router must compile to the Wasm boundary alongside core/adapters and stay testable without standing up providers, shells, or async runtimes.

### 3. Transition re-export preserves the existing surface

**Choice:** `animus-adapters::router` becomes a re-export shim of `animus-router` public items for one transition window so existing call sites and tests compile unchanged.

**Rationale:** Moving the module must not force a concurrent rewrite of every dependent. The shim lets later cairns (runtime wiring, signal adapter) migrate call sites incrementally and lets the shim be removed once no direct consumers remain.

### 4. The root facade gates animus-router behind a feature

**Choice:** The root `animus` crate exposes `animus-router` behind a `router` feature, off by default, mirroring how `preserves`, `steel`, `runtime`, and `tui` are gated.

**Rationale:** The root compatibility facade should stay minimal by default; embedders opt into the router surface explicitly.

## Risks / Trade-offs

- A re-export shim can hide accidental API drift. Mitigate with a parity test asserting the re-exported surface matches the moved module item-for-item, and remove the shim once direct consumers migrate.
- Adding a crate expands the Wasm surface rail. Mitigate by adding `animus-router` to the `check-wasm-surface` allowlist in the same change and proving `cargo check -p animus-router --target wasm32-unknown-unknown --locked` passes.
- Nickel policy hooks for router recipes could overlap future policy-contracts cairn work. Keep this change to a conformance-index fixture stub and a policy hook reference; defer richer policy to `add-router-policy-contracts`.
