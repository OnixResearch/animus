# Evidence: adapt-pi-runtime-event-lifecycle

## Implementation

- Added typed runtime lifecycle events, stable event-kind labels, loop decision classes, cancellation/compaction DTOs, and terminal shutdown status classes in `crates/animus-runtime/src/telemetry.rs`.
- Added host-injected middleware contracts and deterministic middleware-stack execution in `crates/animus-runtime/src/middleware.rs`.
- Wired model/tool dispatch middleware into the runtime shell before adapter calls, with fail-safe guard block/error completion in the transition core.
- Documented SDK/RPC/receipt reuse and TUI-free mode neutrality in `docs/pi-runtime-event-lifecycle.md`.

## Verification

- Baseline before runtime edits: `nix develop -c cargo test -p animus-runtime` — passed, 22 tests.
- Focused after runtime edits: `nix develop -c cargo test -p animus-runtime` — passed, 29 tests.
- Formatting: `nix develop -c cargo fmt --all -- --check` — passed.
- Linting: `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings` — passed.
- Workspace interface check: `nix develop -c cargo check --workspace --all-targets` — passed after extending Preserves runtime-event envelopes for the new DTOs.
- Preserves compatibility: `nix develop -c cargo test -p animus-preserves` — passed, 18 tests.
- Preserves linting: `nix develop -c cargo clippy -p animus-preserves --all-targets -- -D warnings` — passed.
- Pre-commit: `pre-commit run --all-files` — passed after splitting runtime hooks, shell runner, telemetry DTOs, and transition boundary/denial modules under the Octet file-length and path-name limits.

## Coverage notes

- Positive tests cover deterministic model/tool lifecycle event order, headless event-kind consumption, and mutate-then-replace hook chaining.
- Negative tests cover tool middleware block and hook error paths proving adapters are not called and typed hook/denial events are emitted.
- Guarded future surfaces (`compaction`, `replay`, and `session_replacement`) are modeled as fail-safe hook boundaries until their host shell integrations exist.
