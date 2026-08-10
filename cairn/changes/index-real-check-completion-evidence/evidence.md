# Evidence

## Baseline

- `scripts/cairn/check-release-evidence-index --root .` — PASS before implementation.
- `scripts/cairn/check-current-head-release-evidence-index --root .` — PASS before implementation.
- `scripts/cairn/check-dogfood-readiness --root .` — PASS before implementation.

## Portfolio and adversarial audit

Three serial, correlated mechanisms were reviewed: extending generic check rows, trusting a supplied pass row, and adding a dedicated typed receipt row. The dedicated row survived because it preserves compatibility and binds the receipt schema, rail, mode, status, head, source BLAKE3, selected checks, and Cairn binding.

The audit covered missing and duplicate receipts, dirty index and receipt heads, stale heads, fixture mode, non-pass status, malformed records, malformed BLAKE3, wrong schema, incomplete selected checks, wrong Cairn binding, conflicting claims, generic-pass self-attestation, unknown receipt fields, and hidden-check execution.

## Focused evidence

- `scripts/cairn/check-real-check-completion-host-smoke --root .` — PASS in fixture mode with exact selected check ids and pinned Cairn metadata.
- `scripts/cairn/check-release-evidence-index --root .` — PASS with one positive and thirteen real-check negative fixtures, plus all existing fixtures.
- `scripts/cairn/check-current-head-release-evidence-index --root .` — PASS with generated positive, missing, stale, wrong-check, and unknown-field cases.
- `scripts/cairn/check-dogfood-readiness --root .` — PASS.
- Standalone checker and generator compile with `rustc -D warnings` — PASS.
- Rust formatting, shell syntax, and Git whitespace checks — PASS.
- Nix checks `release-evidence-index`, `current-head-release-evidence-index`, `dogfood-readiness`, and `real-check-completion-host-smoke` — PASS locally after the configured remote builder was unavailable.
- Cairn validation plus proposal, design, and task gates — PASS.
- `cargo tigerstyle check` — BLOCKED by the same 254 existing findings. The final finding remains `crates/animus-runtime/src/rejected_attempt/tests.rs:127`; the changed evidence-index files are outside those findings.

## Pending closeout evidence

Clean-current-head live receipt indexing, accepted-spec sync, archive, and final-head publication checks remain pending.

## Non-claims

This evidence does not prove release readiness, production readiness, check sufficiency, check-oracle soundness, executable authenticity, sandboxing, hermeticity, child correctness, or live-provider behavior.
