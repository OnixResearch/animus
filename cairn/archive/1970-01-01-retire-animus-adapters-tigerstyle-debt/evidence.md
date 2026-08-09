# Verification evidence

## Focused results

- `cargo test -p animus-adapters --locked`: PASS, 632 unit tests and 2 integration tests.
- Public compatibility path test: PASS for host authority, messaging, Pijul, SecretSpec, skill-source, and trajectory contracts.
- `cargo clippy -p animus-adapters --all-targets -- -D warnings`: PASS.
- `cargo check -p animus-adapters --no-default-features --locked`: PASS.
- `cargo check -p animus-adapters --target wasm32-unknown-unknown --locked`: PASS.
- `nix build .#checks.x86_64-linux.adapters-octet --no-link -L`: PASS with 0 findings, 0 warnings, and 0 errors.
- Focused Octet config BLAKE3: `905fdb5837fb5a5d55ed7c7b388931717a49d05efa209c6b6300085c02bbfd35`.
- Focused Octet profile BLAKE3: `6b18ceb31eefa762d7c3f827deec69266bc051bfc747e67fdcc9ab5c05715e80`.
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

## Broad result

- `nix flake check -L`: BLOCKED by 2 pre-existing `excessive_file_length` findings in the root compatibility facade. The focused adapter derivation passed before the broad rail stopped.

## Boundaries

- The refactor adds no dependency or ambient authority.
- Existing public compatibility paths remain available through re-export-only modules.
- Existing positive and negative adapter behavior remains checked.
- The focused result makes no claim about Tiger Style debt in other crates or whole-repository release readiness.
