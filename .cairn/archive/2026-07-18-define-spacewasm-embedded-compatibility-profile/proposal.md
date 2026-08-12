## Why

Animus currently proves that selected deterministic reusable crates compile for `wasm32-unknown-unknown`; it explicitly does not claim a Wasm host, plugin runtime, WASI behavior, or ambient capabilities. SpaceWasm creates a useful narrower question for embedded deployments: can a reviewed fixture exposing selected Animus core/adapter behavior be emitted within a conservative WebAssembly 1.0 feature profile and execute under one exact no-std interpreter cohort?

That compatibility must remain opt-in and diagnostic. SpaceWasm is unreleased, lacks the Component Model/WIT/WASI and several post-MVP operators, and should not become Animus's default plugin loader or host shell. A dedicated profile can detect producer-feature drift and validate the reusable boundary without broadening Animus authority.

## What Changes

- Add a typed non-default `spacewasm-embedded-compat` profile for a named subset of Animus no-std crates and fixture exports.
- Preserve the existing `wasm32-unknown-unknown` compile rail and add a separate conservative producer target/profile, preferring `wasm32v1-none` when the pinned Rust toolchain supports the required `core`/`alloc` surface.
- Build a small test-only guest wrapper around selected deterministic core and adapter DTO operations; do not expose provider, tool execution, persistence, process, filesystem, network, credentials, clocks, terminal, Steel, or host shell behavior.
- Consume and remeasure an exact Mantle SpaceWasm reference bundle and matching Octet artifact-profile report.
- Execute only through an injected diagnostic shell and emit bounded compatibility receipts for artifact, profile, input, output, trap/resource class, and non-claims.
- Add positive and negative producer-feature, dependency-isolation, import, ABI, resource, malformed-input/output, stale-evidence, and overclaim fixtures.
- Keep compatibility evidence outside default features and production/plugin activation.

## Impact

- **Surfaces**: Wasm/no-std support policy, typed compatibility profiles, test guest crate/fixture, producer checks, injected diagnostic runner, receipts, component catalog, focused verification rails, and docs.
- **Dependencies**: exact Mantle SpaceWasm bundle and Octet profile report; no direct runtime dependency enters reusable Animus crates.
- **Compatibility**: the existing Wasm compile boundary remains supported independently. Failing SpaceWasm compatibility does not revoke generic compile support; it blocks only the new profile.
- **Claims**: passing proves one exact guest fixture executes under one exact runtime/profile. It does not prove a general Animus Wasm runtime, plugin loading, arbitrary crate compatibility, sandbox completeness, or embedded production readiness.
