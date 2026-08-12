# Proposal: Introduce the animus-router crate barrier

## Why

Animus already owns a substantial router module inside `animus-adapters::router`: recipe DTOs, route families, roles, output contracts, bounds, request-signal projection, recipe selection, catalog/budget projections, route results, and BLAKE3 receipt DTOs. That module has grown past an "adapter contract" boundary and now carries routing *decision* logic (`select_recipe`, `preferred_family`, `select_entries`) that does not belong next to implementation-free model/tool/port contracts. Promoting it to a dedicated workspace crate gives the routing decision its own barrier, no-std purity rail, facade entry, and conformance surface, matching how `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, `animus-runtime`, `animus-std`, and `animus-tui` each own one boundary.

## What Changes

- Add a new `crates/animus-router` workspace crate, `#![no_std]` with `alloc` only, owning the routing decision: recipe/role/output-contract DTOs, signal projection, recipe selection, execution-plan materialization, route-result aggregation, and safe receipt assembly.
- Move `crates/animus-adapters/src/router/` into `animus-router` and re-export the public surface from `animus-adapters::router` for a transition window so existing dependents compile without churn.
- Keep `animus-adapters` owning implementation-free model/tool/port contracts while `animus-router` owns the router-specific validation helpers; `animus-adapters` depends on `animus-router` only for the transition re-export shim, never for runtime authority.
- Add the crate to the workspace `Cargo.toml`, root `animus` compatibility facade (gated behind a `router` feature), flake packages, `check-wasm-surface` allowlist, runtime facade inventory, and adapter conformance index.
- Add a router conformance-index fixture and Nickel policy hook so router recipe/role/bounds contracts stay policy-reviewed.

## Impact

- **Files**: new `crates/animus-router` crate, `animus-adapters` re-export shim, workspace `Cargo.toml`, root `Cargo.toml` feature, `flake.nix`, `scripts/cairn/check-wasm-surface`, `cairn/runtime-facade-inventory.ncl`, `cairn/adapter-conformance-index.ncl`, facade inventory docs, and Cairn evidence.
- **Testing**: positive no-std compile, Wasm surface, and parity tests proving the re-exported surface is byte-identical to the previous in-adapters module; negative tests for purity smoke (no `std::fs`, `std::env`, `println!`, async, or process access in `animus-router`).
- **Non-claims**: this does not add live provider clients, router daemons, network transport, credential lookup, Steel bindings, runtime execution wiring, or new routing behavior beyond what the module already owns.
