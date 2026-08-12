# Proposal: Add an Animus component catalog

## Why

Animus already has many reusable contracts, host-injection seams, optional bridges, and shell adapters, but embedders must currently assemble the Lego map from `README.md`, `docs/contracts-index.md`, `docs/runtime-facade-inventory.md`, crate features, and focused rail scripts. That makes the architecture modular in code but harder to consume as small documented building blocks.

A typed component catalog makes the block map executable: each component declares its owner, authority posture, dependencies, composition links, feature gate, and focused validation rail in one Nickel source of truth.

## What Changes

- Add a Nickel-authored component catalog that records reusable contracts, host-injection surfaces, shell adapters, optional serialization bridges, runtime preludes, and crate/feature ownership.
- Generate or check a human-readable component catalog document from the Nickel data so `README.md`, `contracts-index`, and runtime facade guidance stay aligned.
- Add positive and negative fixtures for catalog validation: valid reusable blocks, invalid authority labels, missing checks, unknown dependencies, unsafe evidence labels, and cycles.
- Wire the catalog into focused Cairn validation so future crate splits cannot leave undocumented blocks behind.

## Impact

- **Surfaces**: `cairn/` Nickel data, docs, scripts/cairn focused rail, README pointers, runtime-facade inventory cross-checks.
- **Testing**: component catalog checker with positive and negative fixtures, Cairn validate/gates, and targeted docs drift checks.
- **Non-claims**: this does not split crates, change public Rust APIs, add host authority, or implement concrete providers/tools/storage. It only creates the executable map those future splits must satisfy.
