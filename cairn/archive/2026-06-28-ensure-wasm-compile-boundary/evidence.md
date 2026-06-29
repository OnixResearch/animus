# Evidence: ensure-wasm-compile-boundary

## Baseline

- `nix run .#cairn -- gate proposal/design/tasks ensure-wasm-compile-boundary --root .`: PASS before edits; tasks gate receipt hash `ba091f0b1edd6bcfdef5724bfe5b2018056ac035fd097e8638530a96b9ccee95`.
- `cargo check -p animus-core --target wasm32-unknown-unknown --locked`: PASS before edits.
- `cargo check -p animus-adapters --target wasm32-unknown-unknown --locked`: PASS before edits.
- `cargo check -p animus --no-default-features --target wasm32-unknown-unknown --locked`: PASS before edits.
- `cargo test -p animus-core --locked`: PASS before edits.
- `cargo test -p animus-adapters --locked`: PASS before edits.
- Baseline gap: the root facade still required `animus-std` as a normal dependency, and no dependency guard proved the no-default Wasm surface stayed shell-free.

## Implementation

- Feature-gated the root `animus` facade with default `std` compatibility and a no-std `--no-default-features` profile over `animus-core` and `animus-adapters`.
- Added `scripts/cairn/check-wasm-surface` and `scripts/cairn/check-wasm-surface.rs` to run credential-free Wasm compile checks plus a cargo-tree dependency guard.
- Added positive and negative cargo-tree fixtures under `cairn/wasm-surface-fixtures/`.
- Added `checks.x86_64-linux.wasm-surface` and documented the Wasm profile in README and `docs/safety-contract.md`.
- Updated roadmap data/docs during implementation so `ensure-wasm-compile-boundary` was classified while active, removed it again after archive, and kept the previously archived Clankers rejection change out of the active roadmap.
- Optional bridge decision: `animus-preserves` is excluded from the initial Wasm surface. It can join only after a later change supplies target compatibility evidence and positive/negative adapter DTO revalidation tests.

## Verification

- `nix develop -c scripts/cairn/check-wasm-surface --root .`: PASS; core/adapters/root Wasm checks passed, dependency guard passed, positive fixture passed, negative fixture reported `WASM_SURFACE_EXCLUDED_PACKAGE`.
- `nix build .#checks.x86_64-linux.wasm-surface --no-link -L`: PASS.
- `nix develop -c cargo test -p animus-core --locked`: PASS after edits.
- `nix develop -c cargo test -p animus-adapters --locked`: PASS after edits.
- `nix develop -c cargo check -p animus --locked`: PASS after edits.
- `nix develop -c cargo check -p animus --no-default-features --locked`: PASS after edits.
- `nix develop -c cargo check -p animus --no-default-features --target wasm32-unknown-unknown --locked`: PASS after edits.
- `nix develop -c cargo check -p animus --features preserves --locked`: PASS after edits.
- `nix develop -c cargo check -p animus --features steel --locked`: PASS after edits.
- `nix develop -c cargo check -p animus --features tui --locked`: PASS after edits.
- `nix develop -c cargo clippy -p animus --all-targets -- -D warnings`: PASS after edits.
- `nix develop -c scripts/cairn/check-roadmap --root .`: PASS after roadmap cleanup; remaining blocked entries are dependency-order reports, not validation failures.
- `nix run .#cairn -- validate --root .`: PASS after edits; 69 active changes and 70 specs validated.
- `nix run .#cairn -- gate proposal/design/tasks ensure-wasm-compile-boundary --root .`: PASS after edits; tasks gate receipt hash `e34a22924c2d02fe63a8cf43f5a29d166a1bec57908c9b5821ac278443db7fef`.
- `nix run .#cairn -- sync ensure-wasm-compile-boundary --root .`: dry-run PASS with empty reasons, plan hash `819410bb8f0aa09c3b3352eb451aea47ea2884137f0f692119f8be0a9f03d4a8`, receipt hash `a9ed32bf2cc11d0034f0bc673d81a8ac275e74ff7b5f8029505d1bf7994217f0`.
- `nix run .#cairn -- sync ensure-wasm-compile-boundary --root . --execute`: PASS with empty reasons and receipt hash `9b95f4fea872d82f9ae0b1150d52cba01aec0a5e034723d70afd586e03320fef`.
- Accepted-spec repair: the sync command succeeded but did not promote `r[agent_harness_runtime.wasm_compile.*]` IDs into `cairn/specs/agent-harness-runtime/spec.md`; the delta was manually merged and verified with a focused requirement-ID search.
- `nix run .#cairn -- validate --root .` after accepted-spec repair: PASS; 69 active changes and 70 specs validated.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive ensure-wasm-compile-boundary --root .`: dry-run PASS with `blocked: false`, empty reasons, plan hash `7c31fefd0bf808b06bc62efd993ea96fd998731135d7f1042e52a5aefe8c0885`, and receipt hash `1dea165f430c4b240ece2112d8e712d0cc789f057356c1f8a66c20f79b610d30`.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive ensure-wasm-compile-boundary --root . --execute`: PASS with empty reasons and receipt hash `c0c91e7f38392d670accb2c79699acd229f79350c4ffdaa1d3f2b7173330db94`.
- `nix run .#cairn -- validate --root .` after archive execution: PASS; 68 active changes and 69 specs validated.
- Archive location: `cairn/archive/2026-06-28-ensure-wasm-compile-boundary/`.
