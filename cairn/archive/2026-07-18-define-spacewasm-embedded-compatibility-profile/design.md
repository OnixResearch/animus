## Context

Animus's supported Wasm surface is compile-only and intentionally excludes runtime and shell crates. SpaceWasm supports a conservative core-module subset and can host primitive imported functions, but ordinary `wasm32-unknown-unknown` compiler defaults may emit unsupported proposals. The compatibility lane therefore needs an explicit producer profile and a small executable fixture rather than attempting to run library crates directly.

The profile is useful only if it preserves the existing no-std functional core and injected-service architecture. Runtime construction, file reads, process invocation, and evidence persistence belong in a diagnostic shell that is not linked into default or reusable profiles.

## Decisions

### 1. Generic Wasm support and SpaceWasm compatibility remain distinct

The existing `wasm32-unknown-unknown` rail continues to prove only compilation of named crates. `spacewasm-embedded-compat` adds a narrower artifact/runtime observation and cannot replace, weaken, or silently constrain the generic rail.

### 2. One typed profile names the exact crate and export surface

A Nickel profile will bind selected Animus crates/features, producer target/toolchain/link flags, guest-wrapper source identity, exported ABI functions, imported host functions, artifact limits, Mantle runtime bundle, Octet report, inputs, expected outcomes, and non-claims. Generated runtime input must remain fresh.

### 3. The producer uses a conservative target without opaque lowering

The first candidate is `wasm32v1-none` under the pinned toolchain when all selected crates support it. If the target or `alloc` surface is unavailable, the profile is blocked. The lane will not rewrite arbitrary modern modules into MVP or treat a lowering tool as semantics-preserving without a separate change.

### 4. A test-only guest wrapper exposes pure operations

A dedicated fixture crate will export a minimal pointer/length/result ABI over bounded byte DTOs for selected core state transition, safe receipt, observability, or routing operations. It will import no ambient services. The wrapper is test material, not a public plugin SDK or runtime facade.

### 5. Static admission precedes execution

The shell will remeasure the Mantle bundle and guest bytes, verify the Octet profile/cohort report, reject undeclared imports or unsupported features, and validate input/resource bounds before instantiating SpaceWasm.

### 6. Runtime behavior stays behind an injected diagnostic service

Reusable crates receive only owned compatibility DTOs and pure decisions. A non-default std host test/service reads bundles, invokes the runtime, captures normalized observations, and returns facts for core classification. Missing service or evidence fails closed; no ambient discovery occurs.

### 7. Receipts are bounded and secret-free

Compatibility receipts bind Animus source/crate/feature profile, producer/toolchain, exact guest bytes, SpaceWasm bundle/cohort/configuration, Octet report, input/output refs, result/trap/resource class, and non-claims. They must not contain credentials, raw environment, arbitrary paths, provider payloads, or unrestricted logs.

### 8. Negative evidence protects authority boundaries

Fixtures will prove that default features do not pull SpaceWasm, runtime execution is absent from descriptor parsing and reusable crates, unsupported producer features fail, undeclared imports fail, and passing compatibility cannot activate plugins or tools.

## Functional core / imperative shell split

- **Pure core**: profile/export DTO validation, supported-surface/dependency admission, evidence identity matching, input/output/resource classification, receipt projection, and deterministic diagnostics.
- **Imperative shell**: fixture compilation, bundle/artifact/report reads, SpaceWasm instantiation, input/output memory transfer, runtime execution, observation capture, and evidence writes.

## Risks / Trade-offs

- `wasm32v1-none` may expose dependency incompatibilities not present in the generic rail. That is profile-specific diagnostic evidence, not a reason to weaken no-std boundaries.
- A tiny wrapper proves only exported fixture behavior and may miss code paths in the selected crates. Coverage must be explicit.
- Runtime integration can accidentally become plugin infrastructure. Default dependency-tree and forbidden-runtime checks must fail on such drift.
- SpaceWasm's maturity and unsafe code prevent production promotion from this compatibility lane alone.

## Non-Goals

- No default SpaceWasm dependency, host, plugin loader, or product feature.
- No WIT, Component Model, WASI, provider, tool, persistence, process, filesystem, network, credential, clock, terminal, Steel, or browser behavior.
- No opaque module lowering or arbitrary `wasm32-unknown-unknown` compatibility claim.
- No sandbox-completeness, runtime-correctness, embedded-readiness, or release claim.
