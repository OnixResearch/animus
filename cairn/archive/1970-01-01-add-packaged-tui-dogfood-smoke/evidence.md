# Evidence: Packaged TUI dogfood smoke

## Baseline

- `cargo test -p animus-tui --locked`: PASS before core edits.
  - 154 unit tests passed.
  - Doc tests passed.
- `scripts/cairn/check-ratatui-tool-host-surface --root .`: PASS before core edits.
- `scripts/cairn/check-packaged-headless-dogfood-smoke --root .`: PASS before core edits.
- `scripts/cairn/check-dogfood-readiness --root .`: PASS before core edits.

## Portfolio frame

Goal: prove one non-default packaged TUI path through accepted headless contracts and a real Ratatui render pipeline.

Completion evidence requires positive and negative focused checks, bounded receipts, claim hygiene, Cairn sync, archive success, and current validation.

False completion includes text-only projection without Ratatui, default feature coupling, raw-content evidence, live-terminal overclaims, or missing cleanup denial.

The selected mechanism wraps the accepted headless smoke, validates only its safe receipt, and renders a fixed metadata view through `TestBackend`.

A separate TUI implementation was rejected because it would duplicate accepted request, model, tool, event, and receipt logic.

## Focused implementation evidence

- `cargo test -p animus-tui --features packaged-tui-dogfood-smoke --locked`: PASS.
  - All 161 feature-enabled tests passed.
  - The seven focused TUI dogfood tests passed.
- `cargo test -p animus-std --features dogfood-smoke packaged_smoke --locked`: PASS.
  - Three accepted headless smoke tests passed.
- `cargo fmt --all -- --check`: PASS.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --locked`: PASS.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo clippy -p animus-tui --all-targets --features packaged-tui-dogfood-smoke --locked -- -D warnings`: PASS.
- `nix develop path:$PWD -c cargo-octet check -p animus-tui -- --all-targets --features packaged-tui-dogfood-smoke`: PASS with zero findings.
- `nix run path:$PWD#cargo-tigerstyle -- check -L`: PASS with zero findings.
- `scripts/cairn/check-packaged-tui-dogfood-smoke --root .`: PASS.
- `nix build path:$PWD#checks.x86_64-linux.packaged-tui-dogfood-smoke --no-link -L`: PASS.
- `scripts/cairn/check-dogfood-readiness --root .`: PASS.
  - The positive packaged TUI fixture passed.
  - The missing-TUI-evidence fixture failed with the expected issue code.
- `scripts/cairn/check-roadmap --root .`: PASS.
  - The product safety path reached accepted Cairn policy and runtime validation requirements.
- `scripts/cairn/check-component-catalog --root .`: PASS.
- `nix run path:$PWD#cairn -- validate --root "$PWD"`: PASS after implementation.
- Cairn `proposal`, `design`, and `tasks` gates: PASS.
- `nix run path:$PWD#cairn -- review completeness --root "$PWD" --change add-packaged-tui-dogfood-smoke`: PASS.

## Bounded workspace blocker

- Global `traceability coverage` still reports older workspace-wide dangling and missing references outside this change.
- The scoped review completeness check for this change passes with no findings.

## Corrected failures

- The first compile found an invalid constant relation. The relation now admits the exact named bound.
- The first focused test expected the readiness name `tui-dogfood`. The accepted headless receipt uses `tui`; the parser now uses that exact marker.
- The first feature Clippy run exposed two existing `animus-std` dogfood warnings. Small ownership-preserving fixes removed both warnings, and the accepted smoke tests still pass.
- The first scoped Octet command used the full-check wrapper with package flags. The corrected dev-shell command ran the scoped check.
- The first scoped Octet result found new TUI smoke findings and older owner-path findings. Explicit types, assertions, qualified owner paths, and a responsibility-based dogfood module path cleared all findings.
- One obsolete Nix build was cancelled after its staged source changed. The current staged source passed the focused Nix check.
