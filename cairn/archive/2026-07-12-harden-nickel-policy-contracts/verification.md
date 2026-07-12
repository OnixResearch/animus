# Verification evidence

## Baseline

- Before the policy-contract edits, `nix run path:$PWD#cairn -- validate --root .` reported `valid: true` for seven active changes.
- Before core parity edits, `cargo test -p animus-runtime --locked` passed and `scripts/cairn/check-steel-runtime-policy --root .` passed.
- The first baseline command used the nonexistent path `scripts/cairn/check-nickel-configs`; that command failed after the runtime and Steel checks. The correct repository command is `scripts/config/check-nickel-configs`, which is included in post-change evidence.

## Implemented boundary

- `policy/contracts/primitives.ncl` owns stable exact-schema, integer-bound, enum, lowercase-BLAKE3, safe-path, and uniqueness diagnostics.
- Model-routing, process-job, Steel-runtime, and tool-catalog sources apply family whole-value contracts and maintain positive plus negative Nickel fixtures.
- `scripts/config/export-policy-projections` exports six checked JSON artifacts and `policy/generated/policy-projection-receipts.json`, binding source, shared-prelude, family-contract, evaluator-profile, and output BLAKE3 identities.
- Rust tests deserialize the generated model-routing, process-job, Steel-runtime, and tool-catalog artifacts and compare normalized DTO meaning without adding runtime Nickel evaluation.
- `docs/nickel-policy-contracts.md` records exact source, generated artifact, checker, Rust owner, runtime owner, and claim boundaries.

## Focused evidence

- `scripts/config/export-policy-projections --execute` and `--check`: pass.
- `scripts/cairn/check-nickel-policy-contracts --root .`: pass with shared and family positive/negative fixtures plus projection and Rust parity checks.
- `scripts/cairn/check-process-job-profiles --root .`: pass.
- `scripts/cairn/check-steel-runtime-policy --root .`: pass.
- `scripts/cairn/check-tool-catalog --root .`: pass.
- `scripts/config/check-nickel-configs`: pass.
- `cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: pass.
- `cargo clippy -p animus-runtime --all-targets --locked -- -D warnings`: pass after correcting the pre-existing process-job assertion form and portfolio fixture copy semantics reached by the widened rail.
- `cargo clippy -p animus-steel --all-targets --locked -- -D warnings`: pass.
- `cargo test -p animus-adapters --locked`: pass.
- `cargo test -p animus-runtime --locked`: pass.
- `cargo test -p animus-steel --locked`: pass.
- `nix build path:$PWD#checks.x86_64-linux.nickel-policy-contracts --no-link -L`: pass after changing the nested export invocation to use the Nix-provided `sh` rather than relying on `/usr/bin/env` inside the derivation.
- `nix run path:$PWD#cairn -- validate --root .`: pass.
- Proposal, design, and tasks gates for `harden-nickel-policy-contracts`: pass.

## Adversarial review reconciliation

VibeThinker was asked for invariant and false-positive risks. Suggestions to accept uppercase hashes, treat script labels as directories with trailing slashes, replace the integer `% 1 == 0` check, add ambient environment-wide defaults, or move freshness decisions into runtime routing were rejected because they contradict the exact lowercase-BLAKE3, file-path, integer, and authority requirements. Independent audit did identify two concrete gaps: routing role minima could leave declared threshold bands uncovered, and allowed process environment prefixes could still admit secret-like keys. The contracts now reject both, with `negative-role-threshold-coverage.ncl` and `negative-secret-environment-key.ncl` fixtures.

## Claim boundary

This evidence establishes checked authoring-time shape, declared whole-value invariants, deterministic projection freshness, and exercised Rust DTO parity only. It does not prove runtime authorization, provider truth, process confinement, tool safety, evaluator soundness, successful host effects, release eligibility, or whole-system correctness. Rust remains responsible for external-data revalidation and runtime admission; host shells retain filesystem, process, provider, tool, clock, credential, persistence, and execution-receipt authority.

## Lifecycle evidence

- `cairn sync harden-nickel-policy-contracts --root .` dry-run: unblocked; `--execute`: completed.
- The eight `agent_harness_runtime.nickel_policy_contracts.*` requirement IDs are present in `cairn/specs/agent-harness-runtime/spec.md` after sync.
- `cairn archive harden-nickel-policy-contracts --root .` dry-run: unblocked; `CAIRN_ARCHIVE_DATE=2026-07-12 ... --execute`: completed.
- Post-archive Cairn validation reports `valid: true` with six active changes and no change or spec issues.
