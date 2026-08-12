# Evidence: Turn lifecycle hook boundaries

## Implementation

- Added `pre_turn` and `post_turn` lifecycle hook kinds to `crates/animus-runtime/src/hooks/lifecycle.rs`.
- Kept startup mapped to the existing `session_start` hook point and added rejection coverage for a duplicate `startup` alias.
- Added `crates/animus-runtime/src/hooks/turn.rs` with a pure FSM (`advance_turn_hooks`) that emits `session_start`, `pre_turn`, and `post_turn` hook effects without executing adapters or hooks.
- Exported turn lifecycle FSM types through runtime exports and the policy prelude.
- Updated lifecycle hook docs and added `docs/turn-lifecycle-hooks.md`.

## Validation

- Baseline before core changes: `nix develop -c cargo test -p animus-runtime hook --locked` passed, 27 tests.
- Focused turn/FSM: `nix develop -c cargo test -p animus-runtime turn --locked` passed, 27 tests.
- Focused hooks: `nix develop -c cargo test -p animus-runtime hook --locked` passed, 37 tests.
- Runtime package: `nix develop -c cargo test -p animus-runtime --locked` passed, 286 tests.
- Formatting: `nix develop -c cargo fmt --check` passed.
- Focused clippy: `nix develop -c cargo clippy -p animus-runtime --all-targets --locked -- -D warnings` passed.
- Tiger Style: `nix develop -c cargo tigerstyle check` passed.
- Cairn: `nix run .#cairn -- validate --root .` plus proposal, design, and tasks gates for `add-turn-lifecycle-hooks` passed.

## Non-claims

This change does not implement concrete shell hook files, Steel hook execution, background daemons, persistence, network exporters, direct tool execution from hooks, UI hook browsers, or a host hook runner. Post-turn continuation requests remain typed follow-up signals that must re-enter normal runtime/continuation policy.

## Known unrelated validation

`nix run .#cairn -- tracey coverage --root .` currently fails the repository-wide coverage gate with pre-existing uncovered/untested requirement output; the narrowed output did not identify the new `turn_lifecycle_hooks` IDs as the cause.
