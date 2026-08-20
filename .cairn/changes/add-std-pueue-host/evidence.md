# Evidence

Date: 2026-08-20

Outcome: blocked by the existing repository-wide Octet gate.

## Passing evidence

- `cargo test -p animus-std-pueue --all-targets --all-features --locked` passed 19 positive and negative tests. The real-daemon test stayed ignored in this command by design.
- The isolated real-daemon test passed with pinned `pueue` and `pueued` executables.
- `cargo clippy -p animus-std-pueue --all-targets --all-features --locked -- -D warnings` passed.
- The pinned `octet-deny-all` package gate passed with workspace, all-target, and all-feature scope. It reported zero findings, warnings, and errors for `animus-std-pueue`.
- `scripts/cairn/check-pueue-host` passed the component tests, real-daemon test, strict Clippy gate, focused queue-profile rail, catalog gates, runtime-facade gate, and evidence-safety gate.
- `scripts/config/check-nickel-configs` passed source-to-projection drift checks and all pueue negative fixtures.
- `cargo test --workspace --all-targets --all-features --locked` passed.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` passed after narrow fixes to existing lint failures.
- `nix build .#checks.x86_64-linux.pueue-octet --no-link -L` passed with zero findings.
- `nix build .#checks.x86_64-linux.pueue-host --no-link -L` passed. This gate included the isolated real daemon, focused Cairn rail, and catalog checks.

## Corrected failures

- An early daemon auto-start path conflicted with process-group cleanup. The final design gives daemon lifecycle to a caller-owned supervisor and keeps a fail-closed health gate in the host.
- Early parser and command-shape defects failed positive and negative fixtures. Typed command builders and strict JSON parsing replaced those paths.
- Early strict Octet runs found issues in the new crate. The final component-level gate is clean without disabled lints, warning budgets, or baselines.

## Blocking evidence

`nix flake check -L` reached the new pueue checks, but the repository-wide `checks.x86_64-linux.octet` derivation failed. It reported 724 findings in existing workspace crates outside `animus-std-pueue`, including `animus-session-core`.

The change remains active. This evidence does not claim full-flake acceptance, archive completion, publication, or consumer readiness.
