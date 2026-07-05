# Evidence: adapt-ornith-scaffold-profiles

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed after removing the archived ResearchSpec change from active roadmap classification.
- `nix run .#cairn -- gate proposal adapt-ornith-scaffold-profiles --root .` passed before implementation.
- `nix run .#cairn -- gate design adapt-ornith-scaffold-profiles --root .` passed before implementation.
- `nix run .#cairn -- gate tasks adapt-ornith-scaffold-profiles --root .` passed before implementation.

## Implementation evidence

- Added pure `animus-runtime::scaffold` DTOs and helpers for reviewed scaffold profiles, revisions, strategy knobs, provenance, review status, route constraints, BLAKE3 descriptor hashes, deterministic selection, revision admission, and influence receipts.
- Kept profile discovery, persistence, prompt assembly, Steel input construction, admission UI, provider execution, and future training/evaluation loops host-owned.
- Re-exported the scaffold runtime surface from `animus-runtime`.
- Added focused positive and negative tests for profile validation, malformed IDs, duplicate revisions, unsupported knobs, missing provenance, unreviewed active revisions, mismatched hashes, deterministic exact/fallback selection, disabled profiles, route mismatch, no-compatible-profile failure, denied/unavailable/stale admissions, receipt leak rejection, and executable-harness rejection.
- Documented route-neutral reviewed strategy data in `docs/ornith-scaffold-profiles.md` and linked it from `README.md`.
- Added scaffold public exports to `cairn/runtime-facade-inventory.ncl`; a scaffold-specific grep of the inventory rail output found no scaffold issues.
- After `cairn sync --execute`, manually merged the scaffold requirement delta into `cairn/specs/agent-harness-runtime/spec.md` because the sync plan executed without promoting the full delta text. A grep for `ornith_scaffold` confirms the accepted spec now contains the new requirement IDs.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime scaffold` passed: 5 focused scaffold tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 176 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `nix run .#cairn -- validate --root .` passed after accepted-spec merge.
- `nix run .#cairn -- gate proposal adapt-ornith-scaffold-profiles --root .` passed.
- `nix run .#cairn -- gate design adapt-ornith-scaffold-profiles --root .` passed.
- `nix run .#cairn -- gate tasks adapt-ornith-scaffold-profiles --root .` passed after task completion.
- `nix run .#cairn -- sync adapt-ornith-scaffold-profiles --root .` dry-run returned `reasons: []`.
- `nix run .#cairn -- sync adapt-ornith-scaffold-profiles --root . --execute` completed.
- `rg ornith_scaffold cairn/specs/agent-harness-runtime/spec.md` confirmed accepted spec sync after the manual merge repair.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings` is blocked by pre-existing `animus-adapters` Clippy errors, including `clippy::needless_pass_by_value` in `crates/animus-adapters/src/workspace/receipt/mod.rs`.
- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` is blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a scaffold-specific grep found no remaining scaffold issues after adding inventory records.

## Requirement coverage

- `r[agent_harness_runtime.ornith_scaffold.profile_contract]`
- `r[agent_harness_runtime.ornith_scaffold.profile_contract.invalid]`
- `r[agent_harness_runtime.ornith_scaffold.selection]`
- `r[agent_harness_runtime.ornith_scaffold.selection.policy]`
- `r[agent_harness_runtime.ornith_scaffold.admission]`
- `r[agent_harness_runtime.ornith_scaffold.admission.unreviewed]`
- `r[agent_harness_runtime.ornith_scaffold.receipts]`
- `r[agent_harness_runtime.ornith_scaffold.receipts.redacted]`
- `r[agent_harness_runtime.ornith_scaffold.scope]`
- `r[agent_harness_runtime.ornith_scaffold.scope.executable_rejected]`
- `r[agent_harness_runtime.ornith_scaffold.verification]`
- `r[agent_harness_runtime.ornith_scaffold.verification.closeout]`
