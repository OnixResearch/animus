# Evidence: adapt-clankers-steel-runtime-policy

## Baseline

- `nix run path:/home/brittonr/git/animus#cairn -- gate proposal adapt-clankers-steel-runtime-policy --root /home/brittonr/git/animus`: PASS.
- `nix run path:/home/brittonr/git/animus#cairn -- gate design adapt-clankers-steel-runtime-policy --root /home/brittonr/git/animus`: PASS.
- `nix run path:/home/brittonr/git/animus#cairn -- gate tasks adapt-clankers-steel-runtime-policy --root /home/brittonr/git/animus`: PASS.
- `nix develop -c scripts/cairn/check-steel-runtime-policy --root /home/brittonr/git/animus`: PASS before closeout edits; receipt recorded the pre-existing future-policy non-claim.
- `nix develop -c scripts/config/check-nickel-configs`: PASS.

## Implementation evidence

- `policy/steel-runtime/policy.ncl` is the Nickel source for Steel profile identity, script binding, BLAKE3 script hash, turn budget, host-action allowlist, fallback mode, and receipt redaction.
- `policy/steel-runtime/generated/steel-runtime-policy.json` is checked generated data, and `scripts/cairn/check-steel-runtime-policy --root .` now exports Nickel to a temporary JSON file and diffs it against the checked artifact.
- `crates/animus-steel` validates profile drafts, script labels, BLAKE3 hashes, budgets, host actions, fallback modes, and receipt redaction before activation returns an allow/deny receipt.
- `crates/animus-runtime` activates policy before constructing the Steel loop fabric and returns `SteelPolicy` failure without model/tool adapter dispatch on denial.
- `target/cairn/focused-rails/steel-runtime-policy.md` records safe fixture names and non-claims only.

## Verification

- `nix develop -c cargo check -p animus-steel --tests --offline`: PASS; refreshed local lock metadata for dev-only generated-policy JSON parsing.
- `nix develop -c cargo fmt --check`: PASS.
- `nix develop -c cargo clippy -p animus-steel --all-targets --locked -- -D warnings`: PASS.
- `nix develop -c scripts/config/check-nickel-configs`: PASS.
- `nix develop -c scripts/cairn/check-steel-runtime-policy --root /home/brittonr/git/animus`: PASS after closeout edits; covers Nickel export drift, generated-policy activation, invalid policy fixtures, runtime policy-denial no-dispatch, positive Steel runtime dispatch, and invalid Steel no-dispatch fixtures.

## Non-claims

This change does not grant Steel ambient filesystem, shell, network, provider, credential, daemon, or session mutation authority. It does not claim live-provider dogfood, TUI or daemon dogfood, VM checks, remote services, plugin runtime, performance, or production readiness.
