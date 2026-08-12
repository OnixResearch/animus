# Evidence: adapt-hermes-skill-manifests

## Baseline before implementation

- `nix develop -c cargo test -p animus-adapters --locked` passed: 172 adapter tests.
- `nix run path:$ROOT#cairn -- validate --root $ROOT` passed.
- `nix run path:$ROOT#cairn -- gate proposal adapt-hermes-skill-manifests --root $ROOT` passed.
- `nix run path:$ROOT#cairn -- gate design adapt-hermes-skill-manifests --root $ROOT` passed.
- `nix run path:$ROOT#cairn -- gate tasks adapt-hermes-skill-manifests --root $ROOT` passed before implementation with all tasks open.
- `scripts/cairn/check-roadmap --root $ROOT` failed for pre-existing roadmap references to inactive change IDs and unknown overlap owners/related changes. Next best validation: Cairn validation and package gates.
- `nix develop -c scripts/cairn/check-runtime-facade-inventory --root $ROOT` failed before this change because existing workspace adapter exports were not yet classified. Next best validation: cargo check, adapter tests, and inventory entries for new skill exports.

## Implementation evidence

- Added pure no-std skill manifest descriptor validation in `crates/animus-adapters/src/skill/`.
- Added positive fixtures for valid summaries, explicit selected-body loading, and non-Hermes stores.
- Added negative fixtures for malformed fields, overlong descriptions, unsupported versions, duplicate tags, missing or unsafe content identity, malformed related names, duplicate names, unresolved related skills, untrusted selected bodies, mismatched content identity, and empty body omission.
- Added `docs/hermes-skill-manifests.md` and runtime-facade inventory entries for the new adapter exports.

## Post-implementation checks

- `nix develop -c cargo fmt --all` passed.
- `nix develop -c cargo test -p animus-adapters --locked` passed: 178 adapter tests.
- `nix develop -c cargo check --workspace --all-targets --locked` passed.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings` remains blocked by pre-existing diagnostics outside `crates/animus-adapters/src/skill/`; a post-fix diagnostic search found no `skill` matches.
- `nix run path:$ROOT#cairn -- validate --root $ROOT` passed after task completion and spec sync/manual accepted-spec merge.
- `nix run path:$ROOT#cairn -- gate proposal adapt-hermes-skill-manifests --root $ROOT` passed.
- `nix run path:$ROOT#cairn -- gate design adapt-hermes-skill-manifests --root $ROOT` passed.
- `nix run path:$ROOT#cairn -- gate tasks adapt-hermes-skill-manifests --root $ROOT` passed.
- `nix run path:$ROOT#cairn -- sync adapt-hermes-skill-manifests --root $ROOT` dry-run was unblocked.
- `nix run path:$ROOT#cairn -- sync adapt-hermes-skill-manifests --root $ROOT --execute` completed.
- Accepted spec `cairn/specs/agent-harness-runtime/spec.md` now contains `r[agent_harness_runtime.hermes_skill_manifest.*]` requirements.
