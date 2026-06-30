# Evidence: Adapt Pi provider registry shell

## Baseline

- PASS: `nix develop -c cargo test -p animus-std` before implementation; existing std tests passed.

## Focused checks

- PASS: `nix develop -c cargo test -p animus-std`; 27 tests passed, including provider selection, thinking clamp/reject, invalid metadata, credential redaction, dynamic provider validation, and runtime-core isolation checks.
- PASS: `nix develop -c cargo clippy -p animus-std --all-targets -- -D warnings`.
- PASS: `nix develop -c scripts/config/check-provider-registry`; valid Nickel registry exported and negative fixtures failed as expected.
- PASS: `pre-commit run --all-files`; Nickel generated configs and Octet lints passed.
- PASS: `TMPDIR=<repo>/.pi/tmp nix develop -c cargo test --workspace`.
- PASS: `TMPDIR=<repo>/.pi/tmp nix develop -c cargo clippy --workspace --all-targets -- -D warnings`.
- PASS: `nix run path:<repo>#cairn -- validate --root <repo>`.
- PASS: `TMPDIR=<repo>/.pi/tmp scripts/cairn/check-roadmap --root .`; known remaining blocker reports only.
- PASS: `TMPDIR=<repo>/.pi/tmp scripts/cairn/check-evidence-safety --root .`.

## Notes

- `docs/pi-provider-registry-shell.md` documents the shell-owned boundary, thinking policy, Nickel config workflow, redacted credentials, and runtime purity evidence.
- `config/provider-registry.ncl` is typed Nickel source for repo-owned registry examples.
