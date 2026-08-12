# Evidence: adapt-clankers-prompt-assembly-kit

r[verify agent_harness_runtime.clankers_prompt_assembly.verification.closeout]

## Implementation evidence

- Added pure `front::prompt_assembly` DTOs and assembly logic in `crates/animus-adapters/src/front/prompt_assembly.rs`.
- Exported the prompt assembly facade through `crates/animus-adapters/src/front/mod.rs`.
- Added positive and negative focused tests for host-context-only assembly, deterministic conflict ordering, over-budget truncation, denied/redacted/unsupported ledger states, missing discovery-adapter behavior, and receipt leak rejection.
- Added `docs/prompt-assembly-kit.md` documenting the source, policy, ledger, receipt, and non-claim boundaries.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-adapters prompt::tests` — passed, 10 tests.
- Baseline before edits: `nix develop -c cargo test -p animus-adapters front::guidance::tests` — passed, 6 tests.
- After implementation: `nix develop -c cargo fmt --check` — passed.
- After implementation: `nix develop -c cargo test -p animus-adapters front::prompt_assembly::tests` — passed, 5 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters prompt::tests` — passed, 10 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters front::guidance::tests` — passed, 6 tests.
- After implementation: `nix develop -c cargo check -p animus-adapters --all-targets` — passed.
- Focused clippy: `nix develop -c cargo clippy -p animus-adapters --lib --no-deps -- -D warnings` — globally blocked by pre-existing lint failures outside `front/guidance.rs`, `front/mod.rs`, and `front/prompt_assembly.rs`; recaptured log had no touched-module diagnostics.
- Cairn proposal/design/tasks gates and validation are run during closeout.

## Next-best validation for blocked rails

- Fix the existing `animus-adapters` clippy backlog outside the touched front modules before using crate-wide clippy as closeout evidence for this change.
