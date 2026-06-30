# Evidence: adapt-trellis-verified-core-boundaries

## Implementation evidence

- `docs/trellis-verified-core-boundaries.md` inventories pure safety-critical candidate boundaries, declares proof obligations, names the selected capability/effect authorization boundary, and records non-claims for unmodeled candidates.
- `crates/animus-adapters/src/effect/authority/model.rs` adds a test-only pure model for authorization issue ordering over explicit contract/grant/caveat/sandbox inputs.
- `crates/animus-adapters/src/effect/authority/tests/equivalence.rs` adds positive equivalence fixtures for allowed model/exec paths and negative counterexamples for contract mismatch, local denial, missing grant, malformed sandbox, and unavailable sandbox.
- `scripts/cairn/check-trellis-verified-core` checks the scope doc, optional Trellis feature declaration, no-default root build, and model equivalence tests, then emits a safe local receipt.
- `flake.nix` wires `checks.x86_64-linux.trellis-verified-core` and includes the new rail in the clean source filter.
- `docs/focused-verification-rails.md` lists the Trellis verified-core rail and its Nix check.

## Verification

- Baseline before model edits: `nix develop -c cargo test -p animus-adapters --locked authority::`: PASS, 10 passed.
- `nix develop -c cargo test -p animus-adapters --locked authority::`: PASS, 12 passed.
- `nix develop -c cargo fmt --check`: PASS.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: PASS.
- `nix develop -c sh scripts/cairn/check-trellis-verified-core --root $ROOT`: PASS; receipt `target/cairn/focused-rails/trellis-verified-core.md`.
- `nix build path:$ROOT#checks.x86_64-linux.trellis-verified-core --no-link -L`: PASS.
- `nix run path:$ROOT#cairn -- gate proposal adapt-trellis-verified-core-boundaries --root $ROOT`: PASS.
- `nix run path:$ROOT#cairn -- gate design adapt-trellis-verified-core-boundaries --root $ROOT`: PASS.
- `nix run path:$ROOT#cairn -- gate tasks adapt-trellis-verified-core-boundaries --root $ROOT`: PASS after task closeout.
- `nix run path:$ROOT#cairn -- validate --root $ROOT`: PASS.

## Non-claims

This change does not formally verify all Animus logic, does not make Trellis a default runtime dependency, does not replace ordinary Rust tests or review, and does not claim verified coverage for ledger, schema, receipt, compaction, or roadmap candidate boundaries that only have inventory entries.
