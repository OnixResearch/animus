# Evidence: Harden Preserves boundary schemas

## Baseline

- PASS: `nix develop -c cargo test -p animus-preserves` before implementation; existing Preserves tests passed.

## Implementation checks

- PASS: `nix develop -c cargo test -p animus-preserves`; 33 tests passed, including versioned text/binary record round trips and negative malformed/compatibility fixtures.
- PASS: `nix develop -c cargo clippy -p animus-preserves --all-targets -- -D warnings`.
- PASS: `nix develop -c cargo check --workspace --all-targets`.
- PASS: `nix develop -c cargo test --workspace`.
- PASS: `nix develop -c cargo clippy --workspace --all-targets -- -D warnings`.
- PASS: `nix run path:<repo>#cairn -- validate --root <repo>`.
- PASS: `scripts/cairn/check-roadmap --root .`; known remaining blocker reports only.
- PASS: `scripts/cairn/check-evidence-safety --root .`.
- PASS: `pre-commit run --all-files`.
- PASS: core-isolation search found direct `preserves::de`, `preserves::ser`, and `preserves::value` use only in `crates/animus-preserves/src/codec.rs`.

## Notes

- `docs/preserves-boundary-schemas.md` documents coverage tiers, versioning, compatibility metadata limits, fixture expectations, and typed-boundary isolation.
- Versioned external codecs wrap typed payloads in `BoundaryRecord`; legacy bare codecs remain internal DTO round trips.
