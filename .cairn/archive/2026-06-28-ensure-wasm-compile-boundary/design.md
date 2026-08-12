# Design: Wasm compile boundary

## Context

`animus-core` and `animus-adapters` already use `#![no_std]` and are shaped as reusable, deterministic crates. The root facade currently re-exports std-shell APIs, while `animus-std`, `animus-steel`, `animus-preserves`, and `animus-tui` carry different host and dependency assumptions. Without a named Wasm boundary, future work can accidentally pull std, process, filesystem, terminal, or native persistence dependencies into the reusable surface.

The goal is not to claim every Animus crate is Wasm-ready. The goal is to keep a small supported surface continuously compile-checked for Wasm while host shells remain explicit opt-ins.

## Decisions

### 1. Start with `wasm32-unknown-unknown`

**Choice:** The first supported Wasm rail targets `wasm32-unknown-unknown` for the deterministic reusable surface.

**Rationale:** This target exercises the strictest embeddable shape: no ambient process, filesystem, terminal, network, or native persistence assumptions. A future `wasm32-wasip1` profile can be added separately if Animus accepts WASI host services.

### 2. Define a narrow supported surface

**Choice:** The initial Wasm-supported surface includes `animus-core`, `animus-adapters`, and a root facade profile that does not pull in std shell crates. Optional bridges such as Preserves may join the rail only after their target compatibility and dependency behavior are verified. `animus-std`, `animus-steel`, and `animus-tui` remain excluded unless later changes define target-specific host contracts.

**Rationale:** This preserves modular embedder choice and prevents host-runtime dependencies from becoming part of the portable core contract.

### 3. Feature-gate the root shell surface

**Choice:** Root `animus` should expose a no-std-compatible facade when built without the std shell feature, while preserving the existing std-backed convenience exports under the default or explicit std-shell feature.

**Rationale:** Embedders often prefer one crate name, but the root facade must not force std shell dependencies into Wasm builds.

### 4. Make the rail credential-free and deterministic

**Choice:** The Wasm compile rail should use Cargo/Nix target checks only. It must not require providers, credentials, network calls, browser runtimes, plugin loading, filesystem mutation beyond normal build outputs, or live host services.

**Rationale:** Wasm support is a compile boundary claim. Runtime host behavior belongs behind injected adapters and separate acceptance criteria.

### 5. Exclusions are part of the contract

**Choice:** Documentation and closeout evidence must name excluded crates/features and explain that Wasm compile support does not grant sandbox, browser, plugin, or provider execution behavior.

**Rationale:** Compile compatibility can otherwise be mistaken for a larger runtime support promise.

## Risks / Trade-offs

- Root feature gating can affect downstream imports; keep default behavior compatible and document the no-std profile clearly.
- Optional bridge crates may compile for Wasm today but still be excluded until the dependency and schema contracts are explicitly validated.
- `wasm32-unknown-unknown` target availability in local toolchains can vary; Nix wiring should pin the target once the local rail is stable.
