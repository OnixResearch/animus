# Evidence: evaluate-clankers-tool-catalog-packs

## Implementation evidence

- Added `docs/tool-catalog-capability-packs.md` with bounded adopt/adapt/reject decisions for Clankers tool-catalog and capability-pack patterns.
- Added Nickel-owned schema source `policy/tool-catalog/catalog.ncl` with typed descriptor/catalog contracts and a checked sample catalog.
- Added positive and negative catalog fixtures under `cairn/tool-catalog-fixtures/` for safe descriptors, duplicate names, built-in collisions, unknown runtimes, invalid schema references, dangerous allow-by-default capabilities, and secret-adjacent no-redaction descriptors.
- Added focused rail `scripts/cairn/check-tool-catalog` with Rust core `scripts/cairn/check-tool-catalog.rs` to validate docs, schema markers, fixtures, and runtime-neutral descriptor material.
- Added the rail to `docs/focused-verification-rails.md` and the README verification command list.
- Promoted `r[agent_harness_runtime.clankers_tool_catalog.*]` requirements into `cairn/specs/agent-harness-runtime/spec.md` after Cairn sync reported an unchanged accepted spec for the full-spec-shaped delta.

## Verification evidence

- Baseline lifecycle validation before edits: `nix run path:/home/brittonr/git/animus#cairn -- validate --root /home/brittonr/git/animus` passed with `valid: true`.
- Baseline Cairn gates before implementation: `gate proposal`, `gate design`, and `gate tasks` for `evaluate-clankers-tool-catalog-packs` passed.
- Focused rail: `nix develop -c scripts/cairn/check-tool-catalog --root /home/brittonr/git/animus` passed and wrote `target/cairn/focused-rails/tool-catalog.md`.
- Workspace dependency check: `nix develop -c cargo check --workspace --all-targets --locked` passed.
- Cairn closeout checks after implementation: `validate`, `gate proposal`, `gate design`, and `gate tasks` passed.
- Sync dry-run: `nix run path:/home/brittonr/git/animus#cairn -- sync evaluate-clankers-tool-catalog-packs --root /home/brittonr/git/animus` was unblocked with one `sync_delta_spec` action.
- Sync execute: `nix run path:/home/brittonr/git/animus#cairn -- sync evaluate-clankers-tool-catalog-packs --root /home/brittonr/git/animus --execute` completed; accepted spec content required manual promotion because before/after hashes were unchanged.
- Validation after spec promotion: `nix run path:/home/brittonr/git/animus#cairn -- validate --root /home/brittonr/git/animus` passed with `valid: true`.
- Task gate after spec promotion: `nix run path:/home/brittonr/git/animus#cairn -- gate tasks evaluate-clankers-tool-catalog-packs --root /home/brittonr/git/animus` passed.

## Non-claims

This evidence does not claim built-in product tools, plugin execution, shell/network tools, provider integration, live catalog discovery, TUI behavior, daemon behavior, or production readiness.
