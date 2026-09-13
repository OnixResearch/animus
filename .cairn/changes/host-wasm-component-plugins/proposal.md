# Proposal: Host capability-scoped Wasm component plugins

## Why

Animus validates data-only plugin descriptors and can select an embedded component backend. It does not load or execute WebAssembly components. The current Wasm rail proves compile portability only, and the SpaceWasm profile is a diagnostic fixture.

A non-default component host can turn the existing descriptor, backend-selection, runtime-authority, tool-stage, and safe-receipt contracts into one real plugin path. The host must remain separate from descriptor parsing and reusable cores.

## What Changes

- Add an authority-free `no_std + alloc` component-plugin contract for exact descriptors, WIT world, bundle, runtime, resource, input, output, and effect-intent facts. r[animus.wasm_plugin.contract]
- Add a separate non-default standard-library host that consumes the shared hardened Wasmtime runtime and Mantle consumer verifier. r[animus.wasm_plugin.host]
- Bind every invocation to the selected embedded-component plan and one current runtime-authority seal before instantiation. r[animus.wasm_plugin.admission]
- Use an import-free first world. The component returns bounded output and optional typed effect intents for existing Animus tool stages to revalidate. r[animus.wasm_plugin.world] r[animus.wasm_plugin.effects]
- Deny silent fallback to stdio, host, process, Steel, or another execution backend. r[animus.wasm_plugin.fallback]
- Add positive and negative fixtures for exact execution, drift, authority, resources, output, effect intents, cleanup, and receipt safety. r[animus.wasm_plugin.fixtures]

## Lifecycle Prerequisites

The pilot requires an immutable shared-runtime candidate with passing producer gates, a published Mantle consumer verifier, and one matching materialization bundle. Production support remains blocked until the shared runtime archives its independent-consumer evidence.

## Impact

- **Core**: a focused component-plugin family owns deterministic admission and metadata-only receipt construction.
- **Shell**: a separate host owns bundle reads, remeasurement, instantiation, bounded execution, cancellation, cleanup, and runtime observations.
- **Existing owners**: plugin descriptors remain data-only. Tool stages retain effect execution. Runtime authority seals retain dispatch admission.

## Out of Scope

- WASI, ambient host imports, arbitrary component registries, automatic discovery, or default plugin loading.
- Direct model, tool, file, process, network, credential, clock, terminal, or persistence authority in the guest.
- Claims of sandbox completeness, plugin correctness, production readiness, or release eligibility.

## Affected Specs

- `wasm-component-plugin-host`: contract, host, world, admission, effects, fallback, fixtures, and non-claims.
