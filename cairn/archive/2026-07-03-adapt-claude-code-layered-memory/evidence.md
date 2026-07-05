# Evidence: adapt-claude-code-layered-memory

r[verify agent_harness_runtime.claude_code_memory.scope.compat_import]

## Implementation evidence

- Added the pure no-std layered-memory facade in `crates/animus-adapters/src/front/memory.rs`.
- Exported the memory facade through `crates/animus-adapters/src/front/mod.rs`.
- Implemented typed instruction and learned-memory descriptors, memory prompt policy, pure in-memory assembly through the reusable prompt assembly core, host-admitted candidate resolution, safe memory influence receipts, and swappable source/store/importer/extraction/admission adapter traits.
- Added positive and negative tests for descriptor validation, deterministic memory prompt assembly, conflict diagnostics with descriptor IDs, stale/denied/unavailable handling, backend-swap equivalence, missing-adapter ledgering, admission denial, persistence prevention, and receipt/admission leak rejection.
- Added `docs/claude-code-layered-memory.md` documenting the bounded reference, descriptor boundary, pure assembly, adapter boundary, receipt safety, and non-claims.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-adapters prompt_assembly --locked` — passed, 7 tests.
- After implementation: `nix develop -c cargo fmt --check` — passed.
- After implementation: `nix develop -c cargo test -p animus-adapters memory --locked` — passed, 5 tests.
- After implementation: `nix develop -c cargo check -p animus-adapters --all-targets --locked` — passed.
- After implementation: `nix develop -c cargo test -p animus-adapters prompt_assembly --locked` — passed, 8 tests.
- Focused clippy: `nix develop -c cargo clippy -p animus-adapters --lib --no-deps -- -D warnings` remains globally blocked by pre-existing `animus-adapters` lint debt; recaptured log at `target/animus-adapters-clippy-memory.log` contains no diagnostics for `crates/animus-adapters/src/front/memory.rs` or `crates/animus-adapters/src/front/mod.rs`.

## Next-best validation for blocked rails

- Fix the existing crate-wide clippy backlog outside the touched memory/front module before using crate-wide clippy as closeout evidence for this change.
