## Phase 1: Profile and pure compatibility core

- [x] [serial] Define the typed Nickel `spacewasm-embedded-compat` profile, exact crate/feature/export surface, conservative producer target/toolchain, Mantle/Octet cohort, limits, expected outcomes, and non-claims. r[agent_harness_runtime.spacewasm_compat.profile] r[agent_harness_runtime.spacewasm_compat.surface]
- [x] [serial] Implement pure profile/export freshness, supported-surface, dependency, evidence-identity, resource/outcome, and receipt-projection validation with deterministic diagnostics. r[agent_harness_runtime.spacewasm_compat.functional_core] r[agent_harness_runtime.spacewasm_compat.evidence]
- [x] [serial] Extend the focused dependency guard to prove default/no-default reusable surfaces do not pull SpaceWasm, Wasmtime, WASI, runtime loaders, or std shell authority. r[agent_harness_runtime.spacewasm_compat.isolation]

## Phase 2: Test guest and injected diagnostic shell

- [x] [depends:spacewasm-compat-profile] Add a test-only conservative-target guest wrapper exposing bounded byte ABI operations over selected deterministic core/adapter behavior with no ambient imports. r[agent_harness_runtime.spacewasm_compat.guest]
- [x] [depends:spacewasm-compat-guest] Add producer artifact inspection and Octet profile/cohort verification before runtime invocation; reject opaque lowering and unsupported features. r[agent_harness_runtime.spacewasm_compat.artifact]
- [x] [depends:spacewasm-compat-artifact] Add a non-default injected diagnostic shell that remeasures the Mantle bundle, executes the fixture under SpaceWasm, captures normalized facts, and returns them to the pure classifier. r[agent_harness_runtime.spacewasm_compat.execution]
- [x] [depends:spacewasm-compat-execution] Emit bounded secret-free compatibility receipts binding source/crates/features, producer, guest bytes, runtime bundle/config, Octet report, inputs, outputs, resource/result class, and non-claims. r[agent_harness_runtime.spacewasm_compat.evidence]

## Phase 3: Positive and negative evidence

- [x] [parallel] Add positive profile/export freshness, conservative-target compile, exact artifact/report/bundle match, pure-operation execution, deterministic output, expected trap, and bounded resource fixtures. r[agent_harness_runtime.spacewasm_compat.verification]
- [x] [parallel] Add negative disabled-profile, unsupported target/dependency/proposal, undeclared import, stale/tampered bundle/report/artifact, malformed input/output, resource exhaustion, missing injected service, runtime failure, default dependency leak, plugin activation, receipt leak, and overclaim fixtures. r[agent_harness_runtime.spacewasm_compat.verification]
- [x] [parallel] Update safety/Wasm surface docs, component catalog, focused verification rails, and non-claims while keeping generic compile support independent. r[agent_harness_runtime.spacewasm_compat.docs]

## Phase 4: Closeout

- [x] [serial] Run generic Wasm surface checks plus conservative-target guest checks, focused core/adapter/receipt tests, dependency isolation, positive/negative runtime smoke when the exact bundle is available, formatting, clippy, and config freshness. r[agent_harness_runtime.spacewasm_compat.verification]
- [x] [serial] Run Cairn validation and proposal/design/tasks gates; sync and archive only with exact evidence or recorded blockers for unavailable runtime/target rails. r[agent_harness_runtime.spacewasm_compat.verification]
