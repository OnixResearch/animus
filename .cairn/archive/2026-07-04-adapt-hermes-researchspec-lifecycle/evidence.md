# Evidence: adapt-hermes-researchspec-lifecycle

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed before implementation.
- `nix run .#cairn -- gate proposal adapt-hermes-researchspec-lifecycle --root .` passed before implementation.
- `nix run .#cairn -- gate design adapt-hermes-researchspec-lifecycle --root .` passed before implementation.
- `nix run .#cairn -- gate tasks adapt-hermes-researchspec-lifecycle --root .` passed before implementation.
- `nix develop -c cargo test -p animus-runtime` showed only the final doc-test block in this harness, so `nix develop -c cargo test -p animus-runtime -- --list` and `nix develop -c cargo test -p animus-runtime --lib` were used for reliable unit-test evidence.

## Implementation evidence

- Added pure `animus-runtime::researchspec` DTOs and deterministic helpers for typed study contracts, baseline evidence, hypotheses, run evidence, keep decisions, promotion groups, archive conclusions, BLAKE3 fingerprints, and validation diagnostics.
- Kept benchmark execution, metric extraction, clocks, filesystems, process control, and archive storage outside the reusable runtime. Hosts must pass already-loaded DTOs into pure validation and gate functions.
- Re-exported the public ResearchSpec runtime surface from `animus-runtime`.
- Added focused positive and negative unit tests for valid/incomplete study contracts, missing primary metrics, guarded fingerprints, vague hypotheses, failed-run rollback context, clean improvements, suspicious improvements, metric regression, minimize direction, independent/dependent/overlapping promotion groups, and unresolved archive decisions.
- Documented the optional lifecycle in `docs/hermes-researchspec-lifecycle.md` and linked it from `README.md`.
- Added ResearchSpec public exports to `cairn/runtime-facade-inventory.ncl` so this change does not add inventory drift.
- After `cairn sync --execute`, manually merged the ResearchSpec requirement delta into `cairn/specs/agent-harness-runtime/spec.md` because the sync plan executed without promoting the full delta text. A grep for `hermes_researchspec` confirms the accepted spec now contains the new requirement IDs.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime researchspec` passed: 9 focused ResearchSpec tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 171 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-hermes-researchspec-lifecycle --root .` passed.
- `nix run .#cairn -- gate design adapt-hermes-researchspec-lifecycle --root .` passed.
- `nix run .#cairn -- gate tasks adapt-hermes-researchspec-lifecycle --root .` passed before and after task completion.
- `nix run .#cairn -- sync adapt-hermes-researchspec-lifecycle --root .` dry-run returned `reasons: []`.
- `nix run .#cairn -- sync adapt-hermes-researchspec-lifecycle --root . --execute` completed.
- `rg hermes_researchspec cairn/specs/agent-harness-runtime/spec.md` confirmed accepted spec sync after the manual merge repair.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings` is blocked by pre-existing `animus-adapters` Clippy errors, including `clippy::needless_pass_by_value` in `crates/animus-adapters/src/workspace/receipt/mod.rs`.
- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` is blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, including missing public exports for several `animus-adapters::*` inventory entries. A ResearchSpec-specific grep of the rail output found no remaining ResearchSpec issues after adding inventory records.

## Requirement coverage

- `r[agent_harness_runtime.hermes_researchspec.study_contract]`
- `r[agent_harness_runtime.hermes_researchspec.study_contract.valid]`
- `r[agent_harness_runtime.hermes_researchspec.study_contract.incomplete]`
- `r[agent_harness_runtime.hermes_researchspec.baseline]`
- `r[agent_harness_runtime.hermes_researchspec.baseline.fingerprints]`
- `r[agent_harness_runtime.hermes_researchspec.baseline.missing_metric]`
- `r[agent_harness_runtime.hermes_researchspec.hypotheses]`
- `r[agent_harness_runtime.hermes_researchspec.hypotheses.vague]`
- `r[agent_harness_runtime.hermes_researchspec.runs]`
- `r[agent_harness_runtime.hermes_researchspec.runs.failed_requires_context]`
- `r[agent_harness_runtime.hermes_researchspec.keep_promotion]`
- `r[agent_harness_runtime.hermes_researchspec.keep_promotion.suspicious]`
- `r[agent_harness_runtime.hermes_researchspec.keep_promotion.overlap]`
- `r[agent_harness_runtime.hermes_researchspec.archive]`
- `r[agent_harness_runtime.hermes_researchspec.archive.unresolved]`
- `r[agent_harness_runtime.hermes_researchspec.verification]`
- `r[agent_harness_runtime.hermes_researchspec.verification.closeout]`
