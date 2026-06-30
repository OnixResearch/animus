# Evidence: adapt Hermes policy profiles

## Scope

- Change: `adapt-hermes-policy-profiles`
- Requirement IDs: `r[agent_harness_runtime.hermes_policy_profiles.*]`
- Reference: Hermes operating-posture/profile patterns are used as bounded guidance only.
- Non-claims: no Hermes prompt is embedded verbatim, no single global prompt is forced, and policy descriptors do not replace runtime capability enforcement.

## Baseline

- `nix develop -c cargo test -p animus-adapters --locked`: PASS before policy-profile edits; 101 adapter tests passed.

## Lifecycle gates

- `nix run .#cairn -- gate proposal adapt-hermes-policy-profiles --root .`: PASS before implementation.
- `nix run .#cairn -- gate design adapt-hermes-policy-profiles --root .`: PASS before implementation.
- `nix run .#cairn -- gate tasks adapt-hermes-policy-profiles --root .`: PASS before implementation.

## Focused implementation checks

- `nix develop -c cargo test -p animus-adapters --locked policy_profile::`: PASS; 5 tests cover trusted global composition, global safety precedence over project overrides, prompt-hygiene rejection for process-heavy/unbounded/stale/missing-validation text, project trust gating, and enforcement-boundary behavior.
- `nix develop -c cargo test -p animus-adapters --locked`: PASS; 106 adapter tests passed.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: PASS.
- `nix develop -c cargo fmt --check`: PASS.

## Evidence safety

Evidence and diagnostics retain only safe provenance labels and bounded status summaries. Conflict diagnostics name the overridden project policy by provenance only and do not embed unsafe project prompt text.
