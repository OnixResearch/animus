# Evidence

Date: 2026-08-24

Outcome: pass for the declared result and supervision convergence scope.

## Passing evidence

- `cargo test --workspace --locked` passed.
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` passed.
- `cargo fmt --all -- --check` passed.
- `cargo tigerstyle check` passed with zero Octet findings.
- `cargo check -p animus-runtime --no-default-features --target wasm32-unknown-unknown --locked` passed.
- `scripts/cairn/check-agent-harness-convergence --root .` passed.
- `nix build .#checks.x86_64-linux.agent-harness-convergence --no-link -L --option builders ''` passed.
- `nix flake check -L --option builders ''` passed. The expected incompatible `aarch64-linux` omission remained.
- Component-catalog and runtime-facade inventories passed with positive and negative fixtures.

## Focused matrix

- Model, human, and persisted projections retained one typed datum identity.
- Drifted string-only persisted data failed inspection.
- Truncated output without a typed handle failed closed.
- Unknown handles failed with the exact token.
- Structured append-only history rendered without an executor port.
- Over-bound silence detached with a handle and follow command.
- Suspension dumps named every item and duration.
- A two-call barrier fixture proved real parallel worker execution.
- Commit ran only after all workers joined.
- Parked cycles did not read as finished.
- Inside-guard and partial commit observations failed closed.
- Telemetry named CPU-time and output-byte bounds and rejected over-bound observations.

## Bounded warnings and non-claims

- The optional local binary cache timed out repeatedly. Nix disabled it and completed local builds.
- The rails do not prove tool correctness, result freshness, authorization, provider behavior, model quality, sandboxing, production readiness, or release eligibility.
