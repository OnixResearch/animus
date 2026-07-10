# Proposal: Split adapter families into focused crates

## Why

`animus-adapters` is the largest crate and currently acts as a single bin for many independent Lego blocks: model, tool, context, memory, policy, workspace, observability, orchestration, safe receipts, plugin manifests, transport DTOs, and more. The crate is implementation-free and no-std, but embedders still pull the whole adapter surface when they need one family.

Splitting high-value families into focused crates makes adapter contracts easier to understand, test, publish, and compose while preserving `animus-adapters` as the compatibility facade.

## What Changes

- Introduce a staged adapter-family crate split plan backed by the component catalog.
- Extract low-risk families first, such as safe receipts and observability, before larger context/tool/memory/policy/workspace families.
- Keep existing public `animus-adapters` item names and import paths as facade re-exports during migration.
- Add parity tests, no-std/Wasm checks where applicable, conformance-index entries, and positive/negative fixtures for each extracted family.

## Impact

- **Surfaces**: workspace crate graph, `animus-adapters` facade exports, adapter conformance index, runtime facade inventory, component catalog, docs, Wasm surface rail.
- **Testing**: per-family DTO tests, re-export parity tests, no-std purity checks, adapter conformance index, catalog checks, and Cairn gates.
- **Non-claims**: this does not add concrete providers, tools, memory stores, workspaces, networks, plugin runtimes, credentials, persistence, or new adapter behavior. It is a boundary split with compatibility shims.
