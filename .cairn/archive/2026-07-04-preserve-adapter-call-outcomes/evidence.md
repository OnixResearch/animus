# Evidence: preserve-adapter-call-outcomes

r[verify agent_harness_runtime.adapter_outcomes.verification]

## Implementation evidence

- Added typed adapter-call outcome envelopes in `crates/animus-adapters/src/invocation.rs` with completed responses and absent, denied, unavailable, malformed, cancelled, timed-out, and execution-failure classes.
- Added safe failure metadata validation for bounded messages, correlation IDs, receipt references, and raw material leak markers.
- Added opt-in typed outcome adapter traits while preserving the legacy `ComponentAdapter::call` path.
- Runtime adapter transitions now preserve typed fail-closed classes through `RuntimeEvent::AdapterOutcomeClassified` and map legacy display failures to conservative execution failures.
- Preserves runtime event encoding now supports `AdapterOutcomeClassified` so typed outcome evidence round-trips.

## Checks

- `nix develop -c cargo test -p animus-adapters invocation --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-runtime shell --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-preserves --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-preserves -p animus-std --locked` — passed.
- `nix develop -c cargo clippy -p animus-preserves --all-targets --locked --no-deps -- -D warnings` — passed for the touched Preserves event coverage.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks preserve-adapter-call-outcomes --root .` — passed.

## Blocked rails

- Combined `animus-std` clippy remains blocked by existing `crates/animus-std/src/providers/capability/flow.rs` `too_many_arguments` lint; full log captured at `target/focused-clippy-preserves-std-nodeps.log`.
