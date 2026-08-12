# Tasks

- [x] [serial] Document the supported Wasm target profile, included crates, excluded crates/features, and non-claims in README and the safety contract. r[agent_harness_runtime.wasm_compile.surface]
- [x] [depends:wasm_compile.surface] Feature-gate the root facade so `animus` can expose a no-std-compatible surface without pulling `animus-std`, terminal, provider, process, filesystem, or native persistence dependencies. r[agent_harness_runtime.wasm_compile.root_facade]
- [x] [depends:wasm_compile.root_facade] Add a credential-free Wasm compile rail for `wasm32-unknown-unknown` covering `animus-core`, `animus-adapters`, and the no-std root facade profile. r[agent_harness_runtime.wasm_compile.rail]
- [x] [depends:wasm_compile.rail] Add a dependency-drift guard or focused check proving std shell, terminal, Steel, provider, native persistence, and host execution dependencies are not required by the Wasm-supported surface. r[agent_harness_runtime.wasm_compile.shell_isolation]
- [x] [depends:wasm_compile.rail,wasm_compile.shell_isolation] Decide whether `animus-preserves` participates in the initial Wasm rail; document blockers or add it with positive compile coverage and malformed-payload tests still routed through adapter DTOs. r[agent_harness_runtime.wasm_compile.optional_bridges]
- [x] [depends:wasm_compile.optional_bridges] Run focused Wasm rail, relevant crate tests, workspace check/clippy as applicable, Cairn validation, and Cairn gates with reviewable evidence before archive. r[agent_harness_runtime.wasm_compile.closeout]
