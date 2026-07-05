# Evidence: adapt-ornith-eval-feedback-loop

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed after archiving the scaffold and trust-boundary prerequisites and removing their active roadmap blockers.
- `nix run .#cairn -- gate proposal adapt-ornith-eval-feedback-loop --root .` passed before implementation.
- `nix run .#cairn -- gate design adapt-ornith-eval-feedback-loop --root .` passed before implementation.
- `nix run .#cairn -- gate tasks adapt-ornith-eval-feedback-loop --root .` passed before implementation.

## Implementation evidence

- Added pure `animus-runtime::eval_feedback` DTOs and helpers for local offline scaffold evaluation plans, redacted evaluation receipts, feedback candidates, host review outcomes, evidence freshness policy, and unsupported scope rejection.
- Kept batch execution, provider calls, verifier execution, filesystem/process/network access, review UI, and scaffold admission host-owned.
- Re-exported the evaluation feedback runtime surface from `animus-runtime`.
- Added focused positive and negative tests for valid local plans, remote task set rejection, missing hashes, unsupported output formats, non-positive run counts, provider/public benchmark claims, hidden-test/cloud/live-mutation/RL scope rejection, receipt leak rejection, feedback non-auto-acceptance, exact freshness, stale evidence, and reduced-weight evidence.
- Documented the offline reviewed feedback workflow in `docs/ornith-eval-feedback-loop.md` and linked it from `README.md`.
- Added eval feedback public exports to `cairn/runtime-facade-inventory.ncl`; an eval-feedback-specific grep of the inventory rail output found no eval feedback issues after rewording task set review notes to avoid unsafe evidence-marker false positives.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime eval_feedback` passed: 5 focused evaluation feedback tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 186 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings` is blocked by pre-existing `animus-adapters` Clippy errors, including `clippy::needless_pass_by_value` in `crates/animus-adapters/src/workspace/receipt/mod.rs`.
- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` is blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but an eval-feedback-specific grep found no remaining eval feedback issues after adding inventory records.

## Requirement coverage

- `r[agent_harness_runtime.ornith_eval_feedback.plan]`
- `r[agent_harness_runtime.ornith_eval_feedback.plan.invalid]`
- `r[agent_harness_runtime.ornith_eval_feedback.receipts]`
- `r[agent_harness_runtime.ornith_eval_feedback.receipts.redacted]`
- `r[agent_harness_runtime.ornith_eval_feedback.review]`
- `r[agent_harness_runtime.ornith_eval_feedback.review.no_auto_accept]`
- `r[agent_harness_runtime.ornith_eval_feedback.freshness]`
- `r[agent_harness_runtime.ornith_eval_feedback.freshness.stale]`
- `r[agent_harness_runtime.ornith_eval_feedback.scope]`
- `r[agent_harness_runtime.ornith_eval_feedback.scope.live_mutation_rejected]`
- `r[agent_harness_runtime.ornith_eval_feedback.verification]`
- `r[agent_harness_runtime.ornith_eval_feedback.verification.closeout]`
