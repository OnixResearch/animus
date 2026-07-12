# Verification evidence

## Baseline

- Before the portfolio core was added, `cargo test -p animus-runtime subagents --locked` passed 14 tests with no failures.
- Before the change, `nix run .#cairn -- validate --root .` reported `valid: true`.

## Focused implementation evidence

- `cargo test -p animus-runtime portfolio_search --locked` passes 8 positive and negative tests.
- `cargo check -p animus-runtime --lib --locked` passes.
- `cargo fmt --manifest-path crates/animus-runtime/Cargo.toml -- --check` passes.
- `scripts/cairn/check-runtime-facade-inventory --root .` passes its inventory plus positive and negative fixtures after classifying the new public facade.
- `scripts/cairn/check-safe-receipts-artifacts --root .` passes.
- `nix run .#cairn -- validate --root .` reports `valid: true`.
- Proposal, design, and tasks gates for `add-bounded-portfolio-search` pass.
- Cairn sync executed, accepted portfolio requirements were verified in `cairn/specs/agent-harness-runtime/spec.md`, and the archive dry-run reached only the intentionally unchecked closeout task before that task was satisfied.

The standard `cargo clippy -p animus-runtime --lib --locked -- -D warnings` rail was run and reaches an unrelated pre-existing error at `crates/animus-runtime/src/background/checks/mod.rs:223`: `clippy::manual_assert_eq` recommends replacing `debug_assert!(policy.max_timeout_seconds != Some(0))` with `debug_assert_ne!`. After portfolio-local lint findings were corrected, this is the only reported clippy error. The change does not modify that pre-existing surface.

## Advisory review reconciliation

VibeThinker was asked to look for false-completion and state-linkage holes. Its concrete validator-policy concern was accepted: a structurally valid arbitrary validator-policy hash could previously pass. `PortfolioContract` now carries a separate admitted validator-policy fingerprint, validation receipts must match it, and a negative test rejects an unadmitted policy. Suggestions that asked this core to prove natural-language mechanism equivalence, host context isolation, redaction completeness, validator soundness, or whole-agent/release completion were rejected as explicit non-claims.

## Claim boundary

The evidence supports deterministic DTO validation, transition rules, BLAKE3 linkage, and the exercised fixtures only. It does not prove natural-language mechanism distinctness, model independence, validator soundness, proof correctness, repository readiness, release readiness, or authoritative agent completion.
