# Evidence: adapt-zerostack-prompt-mode-profiles

## Baseline

- `nix develop -c cargo test -p animus-adapters` — passed before implementation; 404 tests passed.
- `nix run .#cairn -- validate --root .` — passed before implementation; 15 changes, 16 specs validated.
- `scripts/cairn/check-roadmap --root .` — passed before implementation; reported active prerequisite blockers only for unrelated dependent changes.
- `nix run .#cairn -- gate proposal|design|tasks adapt-zerostack-prompt-mode-profiles --root .` — passed before implementation as part of all-change gate sweep.

## Implementation evidence

- Added no-std prompt profile DTOs, directive parsing, policy-bound mode binding, chain planning, and hashed safe receipts in `crates/animus-adapters/src/prompt/profiles.rs`.
- Re-exported prompt profile contracts from `animus-adapters`.
- Added `docs/prompt-mode-profiles.md` and linked it from `README.md`.

## Verification

- `nix develop -c cargo fmt --check && nix develop -c cargo test -p animus-adapters prompt::profiles::tests` — passed; 15 prompt-profile tests passed.
- `nix develop -c cargo test -p animus-adapters` — passed after implementation; 419 tests passed.

## Blocked broader checks

- `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` — failed on pre-existing unrelated Clippy warnings in existing adapter test/support code before reaching a clean all-targets result.
- `nix develop -c cargo clippy -p animus-adapters --lib -- -D warnings` — failed on pre-existing unrelated Clippy warnings in existing adapter library code.
- `scripts/cairn/check-runtime-facade-inventory --root .` — failed with pre-existing runtime-facade inventory drift including stale classified exports.
