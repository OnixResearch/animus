# Evidence: align-clash-patterns-with-ucan

## Implementation evidence

- `docs/clash-ucan-authority.md` documents bounded Clash adaptation scope and explicitly rejects Clash Starlark policy, brush shell, plugin machinery, editor/LSP surfaces, installer flows, audit schema, and precedence-as-authority.
- `crates/animus-adapters/src/effect/authority.rs` adds a pure no-std authorization core for guarded action capability requests, Nickel-reviewed policy contract fixtures, UCAN grant checks, local caveats, sandbox constraints, and redacted authorization receipts.
- `policy/capability/policy.ncl` is the Nickel source for model, tool, filesystem, exec, network, and Steel resource/ability mappings; `policy/capability/generated/capability-policy.json` is the checked export.
- `scripts/cairn/check-clash-ucan-authority` validates Nickel export drift and runs focused positive/negative authority fixtures.
- `target/cairn/focused-rails/clash-ucan-authority.md` records safe fixture names and non-claims only.

## Verification

- `nix run path:/home/brittonr/git/animus#cairn -- gate proposal align-clash-patterns-with-ucan --root /home/brittonr/git/animus`: PASS.
- `nix run path:/home/brittonr/git/animus#cairn -- gate design align-clash-patterns-with-ucan --root /home/brittonr/git/animus`: PASS.
- `nix run path:/home/brittonr/git/animus#cairn -- gate tasks align-clash-patterns-with-ucan --root /home/brittonr/git/animus`: PASS before task closeout edits.
- `nix develop -c cargo test -p animus-adapters --locked authority::`: PASS.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: PASS.
- `nix develop -c sh scripts/cairn/check-clash-ucan-authority --root /home/brittonr/git/animus`: PASS.
- `nix build .#checks.x86_64-linux.clash-ucan-authority --no-link -L`: PASS.
- `nix run path:/home/brittonr/git/animus#cairn -- validate --root /home/brittonr/git/animus`: PASS.
- `nix develop -c scripts/cairn/check-roadmap --root /home/brittonr/git/animus`: PASS with blocked dependent reports.

## Non-claims

This change does not adopt Clash's Starlark DSL, full policy engine, brush shell, agent plugin machinery, editor/LSP surfaces, installer flows, audit schema, or precedence-as-authority model. It does not grant filesystem, shell, network, provider, credential, daemon, or session mutation authority by itself.
