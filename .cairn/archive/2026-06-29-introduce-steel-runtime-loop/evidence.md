# Evidence: introduce-steel-runtime-loop

## Baseline

- `nix run path:$PWD#cairn -- validate --root $PWD`: PASS before edits; lifecycle tree valid with 76 active changes.
- `nix run path:$PWD#cairn -- gate proposal/design/tasks introduce-steel-runtime-loop --root $PWD`: PASS before implementation; tasks gate verdict PASS.
- `cargo test -p animus-steel -p animus-adapters --locked`: PASS before runtime edits; 18 tests passed.
- `scripts/cairn/check-roadmap --root .`: PASS before edits; reported unrelated dependency-order blocked changes but roadmap validation succeeded.

## Implementation

- Added `crates/animus-runtime` as the reusable agent-loop runtime crate with typed `RuntimeInput`, `RuntimeConfig`, `MaxTurns`, `RuntimeEvent`, `RunReport`, `RuntimeStatus`, and runtime error/failure DTOs.
- Added a pure transition core over `TurnState`, `TurnPlan`, `DecisionStep`, `AdapterStep`, adapter requests/outcomes, max-turn handling, final/stop handling, and fabric/adapter failure reports.
- Added a thin generic runtime shell that accepts an injected `LoopFabric`, `ModelAdapter`, and `ToolAdapter`; added `run_with_steel_program` for the default Steel fabric without constructing concrete providers or tools.
- Added positive runtime tests for immediate final response, explicit stop, model dispatch, tool dispatch, and multi-turn propagation through prior model/tool outputs.
- Added negative runtime tests for invalid Steel decisions, invalid Steel load, model adapter failure, tool adapter failure, max-turn exhaustion, blank runtime input, and invalid zero-turn config.
- Extended `animus-preserves` with runtime event/report/failure envelopes and round-trip plus malformed-payload tests, while keeping the runtime crate free of a Preserves dependency.
- Updated README, safety contract, request lifecycle, runtime facade inventory, root feature wiring, flake checks, and a runtime Preserves schema note under `schemas/preserves/animus-runtime.md`.
- Fixed the runtime facade inventory rail so crate-root public exports from `animus-runtime` and adapter manifest DTOs are classified, and reusable authority scanning ignores export-path substrings while still checking source/review evidence text.

## Verification

- `cargo test -p animus-runtime --locked`: PASS after adding runtime; 11 tests passed.
- `cargo test -p animus-preserves --locked`: PASS after runtime envelopes; 16 tests passed.
- `cargo test -p animus-runtime -p animus-preserves --locked && cargo clippy -p animus-runtime -p animus-preserves --all-targets --locked -- -D warnings`: PASS after final refactor.
- `cargo check --workspace --all-targets --locked`: PASS.
- `cargo test --workspace --locked`: PASS.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS.
- `cargo check -p animus --features preserves --locked && cargo check -p animus --features runtime --locked && cargo check -p animus --features steel --locked`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `scripts/cairn/check-request-lifecycle --root .`: PASS after lifecycle doc update.
- `scripts/cairn/check-runtime-facade-inventory --root .`: PASS after inventory updates; 109 entries matched 109 public exports before final runtime refactor and fixtures passed.
- `scripts/cairn/check-wasm-surface --root .`: PASS; Wasm surface remains no-std root/core/adapters only.
- `scripts/cairn/check-roadmap --root .`: PASS; remaining blocked entries are dependency-order reports, not validation failures.
- `pre-commit run --all-files`: PASS after staging the new runtime crate so Nix checks could see untracked files.
- `nix develop -c cargo-octet check -p animus-runtime`: PASS after final runtime refactor; Octet summary clean.
- `nix run .#cargo-tigerstyle -- check`: PASS after final runtime refactor; task completed successfully.
- `nix flake check -L`: PASS; all checks passed. Remote cache/builder warnings were non-blocking.

## Sync and archive closeout

- `nix run path:$PWD#cairn -- validate --root $PWD` plus proposal/design/tasks gates: PASS after task completion.
- `nix run path:$PWD#cairn -- sync introduce-steel-runtime-loop --root $PWD`: dry-run PASS; no blockers.
- `nix run path:$PWD#cairn -- sync introduce-steel-runtime-loop --root $PWD --execute`: PASS, followed by Cairn validation PASS.
- Accepted spec verification: all runtime-loop requirement IDs are present in `cairn/specs/agent-harness-runtime/spec.md`.
- `nix run path:$PWD#cairn -- archive introduce-steel-runtime-loop --root $PWD`: dry-run PASS; no blockers.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run path:$PWD#cairn -- archive introduce-steel-runtime-loop --root $PWD --execute`: PASS; change moved to `cairn/archive/2026-06-29-introduce-steel-runtime-loop/`.
- Post-archive `nix run path:$PWD#cairn -- validate --root $PWD`: PASS; 75 active changes, 76 accepted specs.
- Final `pre-commit run --all-files`: PASS.
- Final `nix flake check -L`: PASS; all checks passed.
