# Tasks

## Phase 1: Baseline and prerequisites

- [ ] [serial] Record plugin-descriptor, backend-selection, runtime-authority-seal, tool-stage, safe-receipt, no-std, and Wasm-isolation baselines before core changes. r[animus.wasm_plugin.fixtures]
- [ ] [serial] Record exact shared-runtime candidate, Mantle verifier, and matching materialization-bundle prerequisites and keep product support blocked until producer evidence is complete. r[animus.wasm_plugin.host] r[animus.wasm_plugin.nonclaims]
- [ ] [serial] Define component, WIT, bundle, runtime, resource, envelope, effect-intent, outcome, limit, and receipt DTOs in a focused `no_std + alloc` adapter family. r[animus.wasm_plugin.contract] r[animus.wasm_plugin.receipts]
- [ ] [serial] Implement pure descriptor-to-component admission, identity construction, outcome normalization, effect-intent validation, fallback denial, and receipt payload construction. r[animus.wasm_plugin.contract] r[animus.wasm_plugin.admission] r[animus.wasm_plugin.effects] r[animus.wasm_plugin.fallback]

## Phase 2: World and host shell

- [ ] [serial] Define the import-free WIT request and result world with exact codec, schema, method, and bound identities. r[animus.wasm_plugin.world]
- [ ] [serial] Add the separate non-default component host with injected roots and services, Mantle file verification, shared-runtime execution, deadlines, cancellation, cleanup, and bounded observations. r[animus.wasm_plugin.host]
- [ ] [serial] Bind every invocation to an embedded-component backend plan and matching current runtime-authority seal with a final pre-effect recheck. r[animus.wasm_plugin.admission]
- [ ] [serial] Route returned effect intents into existing tool stages as untrusted proposals and prohibit direct guest effects. r[animus.wasm_plugin.effects]
- [ ] [serial] Enforce no fallback after component denial, trap, deadline, cancellation, invalid output, resource failure, or cleanup failure. r[animus.wasm_plugin.fallback]

## Phase 3: Evidence and isolation

- [ ] [parallel] Add positive exact-bundle invocation, stable decision, valid output, effect-intent proposal, cancellation, and cleanup fixtures. r[animus.wasm_plugin.fixtures]
- [ ] [parallel] Add negative missing dependency, descriptor drift, wrong world, import, stale plan, stale seal, resource, trap, deadline, invalid output, effect bypass, receipt leak, fallback, and overclaim fixtures. r[animus.wasm_plugin.fixtures] r[animus.wasm_plugin.receipts] r[animus.wasm_plugin.nonclaims]
- [ ] [parallel] Add dependency and feature-isolation gates that keep the host out of default, reusable-core, descriptor-only, and compile-only Wasm graphs. r[animus.wasm_plugin.host]
- [ ] [serial] Update the component catalog, plugin docs, backend docs, authority-seal docs, safety contract, and README with exact support and non-claims. r[animus.wasm_plugin.contract] r[animus.wasm_plugin.nonclaims]
- [ ] [serial] Run focused tests before and after changes, host and no-std builds, compile-only Wasm isolation, Clippy with warnings denied, Octet, Cairn validation and gates, and relevant Nix checks. r[animus.wasm_plugin.fixtures]
