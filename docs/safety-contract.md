# Safety Contract

Animus is structured around a functional-core / imperative-shell (FCIS) split. This contract is repo-owned and must be updated with any intentional change to the safety model.

## FCIS split

- `crates/animus-core` is the functional core. It owns pure deterministic logic, small typed state, and the minimal validation error surface.
- `crates/animus-adapters` is the component-boundary crate. It owns implementation-free port traits and typed DTOs that separate the harness from swappable model, tool, memory, and host integrations.
- `crates/animus-preserves` is the optional Preserves serialization bridge. It owns cross-language text/binary encoding for adapter boundary messages and must not become a required dependency of the core or adapter-contract crates.
- `crates/animus-steel` is the canonical Steel Scheme loop fabric. It owns script evaluation for agent turn selection and must revalidate every selected action through Rust adapter DTOs before calling model, tool, or response boundaries.
- `crates/animus-std` is the imperative shell. It owns environment parsing, runtime setup, logging, async work, I/O, default std adapters, and other process-facing concerns.
- The root `animus` package is a thin compatibility layer. It must not accumulate non-trivial business logic.
- Shell-owned runtime inputs must not leak into exported core state types. Component implementations must be reached through adapter contracts rather than direct cross-crate coupling. Serialized boundary payloads and Steel loop decisions must revalidate through adapter DTO constructors after decoding/evaluation.

## no_std core boundary

`animus-core` and `animus-adapters` are `#![no_std]` and may use `alloc` for owned data. Do not use `std`, process environment, filesystem, networking, clocks, randomness, interpreters such as Steel, serialization backends such as Preserves, or global mutable state in either crate.

## Config ownership

Parse configuration once at the shell boundary, validate it, and keep the typed result in owned structs such as `RuntimeConfig`. Do not validate-and-drop raw settings and then re-read or re-parse them later.

Repo-owned tool configuration must be authored in Nickel under `config/repo.ncl` with typed contracts. Keep tool-native TOML/YAML files generated from Nickel for consumers that require those formats, and run the generated-config drift check after every config change.

## Tests

Every test suite must include both positive and negative cases. Positive tests cover valid inputs and expected behavior. Negative tests cover invalid config, invalid input, missing fields, boundary failures, and shell-side error paths.

Run the smallest relevant baseline tests before modifying core logic, then run them again after the change. If no baseline is possible, record why in the change evidence.

Tests that touch process-global state, such as environment variables, clocks, or current directories, must add explicit isolation. Prefer deterministic helpers such as `RuntimeConfig::from_env_pairs` over mutating process globals.

## No magic numbers

Numeric literals that are not self-evident must be named constants, parameters, or configuration values. If a number needs a comment, it needs a name.

## Unsafe code

Workspace lints forbid unsafe code. If this policy is intentionally relaxed in the future, every unsafe block must include a clear safety explanation and must pass `clippy::undocumented_unsafe_blocks`.

## Octet lint levels

`config/repo.ncl` is the source of truth for the consumer-owned Octet configuration exported to `dylint.toml`. Every registered Octet lint for the pinned flake input must be listed under `octet.lint_levels` with level `"deny"`. Keep the list complete when `flake.lock` updates; any suppression must be narrow, justified, owned, and tested.

## Verification rails

Use these commands as the standard verification rails:

```sh
cargo test -p animus-core
cargo test -p animus-adapters
cargo test -p animus-preserves
cargo test -p animus-steel
cargo test -p animus-std
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p animus --features preserves
cargo check -p animus --features steel
cargo tigerstyle check
check-nickel-configs
scripts/cairn/check-clankers-product-surfaces --root .
nix run .#cargo-tigerstyle -- check
nix run .#check-nickel-configs
nix flake check
```

`cargo tigerstyle check` is the local executable Tiger Style policy rail backed by workspace metadata. `nix run .#cargo-tigerstyle -- check` exposes the pinned flake app for the same check. `check-nickel-configs` and `nix run .#check-nickel-configs` verify that generated TOML/YAML configs match `config/repo.ncl`. `scripts/cairn/check-clankers-product-surfaces --root .` verifies that bounded Clankers documentation remains allowed while reusable crates stay free of rejected Clankers product-surface dependencies and runtime assumptions. The `nickel-configs` and `octet-lints` pre-commit hooks in `.pre-commit-config.yaml` call `scripts/config/check-nickel-configs` and `scripts/hooks/octet-lints`. Keep this document, `config/repo.ncl`, generated config files, `workspace.metadata.tigerstyle`, `workspace.metadata.octet`, the flake apps, and the pre-commit hooks in sync with the Octet command.
