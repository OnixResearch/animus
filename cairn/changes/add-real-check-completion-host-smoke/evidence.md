# Evidence

## Baseline

- `cargo test -p animus-std --features authoritative-completion-smoke --locked authoritative_completion_smoke` — PASS before the core refactor, 4 tests.
- `scripts/cairn/check-authoritative-completion-host-smoke --root .` — PASS before the core refactor.
- Initial Cairn validation found that scenarios lacked separate GIVEN, WHEN, and THEN clauses. The change spec was corrected before implementation, then validation and proposal, design, and task gates passed.

## Portfolio and adversarial audit

Three serial routes covered direct `animus-std` integration, shell-owned execution, and a separate authority crate. The separate AGPL crate was selected because it preserves the default MIT crate boundary and keeps command admission inside the packaged host.

The audit checked license isolation, exact argv comparison, shell absence, explicit roots and environment, executable search, nonzero exit, signal termination, timeout, output overflow, missing executable, stale Cairn evidence, dirty state, metadata leakage, process-group teardown, and default/no-default isolation.

## Focused evidence

- `cargo test -p animus-std-check-exec --features test-fixtures --locked` — PASS, 2 parser tests and 4 actual bounded fixture-child tests.
- `cargo clippy -p animus-std-check-exec --features test-fixtures --all-targets --locked -- -D warnings` — PASS.
- `cargo clippy -p animus-std --features authoritative-completion-smoke --all-targets --locked -- -D warnings` — PASS after the injected-service refactor.
- `cargo check -p animus --features real-check-completion-host` — PASS.
- `cargo check -p animus --no-default-features --locked` — PASS without the new authority crate.
- `scripts/cairn/check-real-check-completion-host-smoke --root .` — PASS in fixture mode.
- Dirty live-mode rejection — PASS with `error: live real-check smoke requires a clean Git worktree`.
- Direct development execution of `animus-real-check-completion-smoke` ran the exact generated `nickel-config` and `cairn-validate` argv and returned two passing checks with pinned Cairn validation. This run was not a clean-head receipt and is not final evidence.
- `nix build .#checks.x86_64-linux.real-check-completion-host-smoke --no-link -L` — PASS in a sandbox after fixture tests.

## Boundary evidence

- `cargo check --workspace --all-targets --locked` — PASS with existing `animus-std-openai` warnings.
- `scripts/cairn/check-agent-role-profiles --root .` — PASS.
- `scripts/cairn/check-multi-role-host-smoke --root .` — PASS.
- `scripts/cairn/check-authoritative-agent-completion --root .` — PASS.
- `scripts/cairn/check-authoritative-completion-host-smoke --root .` — PASS.
- `scripts/cairn/check-dogfood-readiness --root .` — PASS.
- `cargo tigerstyle check` — BLOCKED by the same 254 existing findings. No finding names `animus-std-check-exec` or the real-check smoke. The final finding remains `crates/animus-runtime/src/rejected_attempt/tests.rs:127`.

## Pending final-head evidence

The clean-current-head `--live` rail must run after the implementation commit and again after archive integration. The final receipt must report `mode: live`, `head_dirty: false`, and the published head.

## Non-claims

The evidence does not prove check-oracle soundness, executable authenticity, sandboxing, hermeticity, child correctness, side-effect safety, live-provider behavior, release readiness, or production readiness.
