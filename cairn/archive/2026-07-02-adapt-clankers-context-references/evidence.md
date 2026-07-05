# Evidence: adapt-clankers-context-references

## Implementation

- Added `crates/animus-adapters/src/context.rs` with typed `ContextReference` DTOs, parser/normalizer, host resolver traits, fail-closed resolution planning, deterministic budget decisions, BLAKE3 content identities, and safe provenance receipt validation.
- Re-exported the context-reference surface from `crates/animus-adapters/src/lib.rs`.
- Added `docs/context-references.md` and linked it from `README.md`.

## Verification

- Baseline before core edits: `nix develop -c cargo test -p animus-adapters` — passed, 287 tests.
- Gates before implementation: `nix run path:.#cairn -- gate proposal adapt-clankers-context-references --root .`; `gate design`; `gate tasks` — passed.
- Focused positive/negative context tests: `nix develop -c cargo test -p animus-adapters context` — passed, 11 selected tests.
- Focused crate regression: `nix develop -c cargo fmt --check && nix develop -c cargo test -p animus-adapters` — passed, 296 tests.
- Clippy triage: `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` still reports pre-existing workspace lint debt outside `crates/animus-adapters/src/context.rs`; no context-reference diagnostics remained after the focused grep check.

## Safe evidence posture

The evidence records commands, file paths, and aggregate test counts only. It does not include raw prompts, expanded context bodies, credentials, provider payloads, or full host directory dumps.
