# Evidence: ensure-model-token-thought-streaming

## Implementation evidence

- Added canonical model stream DTOs in `crates/animus-adapters/src/model.rs` for content, thought/reasoning, tool-call, usage, finish, cancellation, and failure frames with call correlation, per-call order, visibility metadata, fixture parsing, stream validation, and visible-content final assembly.
- Added explicit streaming capability metadata and negotiation in `crates/animus-std/src/providers/capability.rs`, including reject and content-only downgrade paths for unsupported thought streaming.
- Added runtime stream projection events in `crates/animus-runtime/src/telemetry/` and protocol-neutral headless event labels in `crates/animus-runtime/src/headless/`.
- Added `docs/model-streaming.md` and README linkage for streaming boundaries, fixture expectations, redaction defaults, and non-claims.

## Validation evidence

- Baseline focused tests before edits: `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-std` — PASS.
- Gates before edits: `nix run path:$PWD#cairn -- gate proposal ensure-model-token-thought-streaming --root .`, `gate design`, and `gate tasks` — PASS.
- Focused implementation tests: `nix develop -c cargo fmt --check && nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-std` — PASS.
- Focused clippy check: `nix develop -c cargo clippy -p animus-adapters -p animus-runtime -p animus-std --all-targets -- -D warnings` — BLOCKED by pre-existing unrelated adapter warnings in active worktree files; grep of the clippy log found no remaining diagnostics for `crates/animus-adapters/src/model.rs`, `crates/animus-std/src/providers/capability.rs`, `crates/animus-runtime/src/telemetry`, or `crates/animus-runtime/src/headless` after local fixes.

## Safety notes

- Default final assembly includes only assistant-visible content frames.
- Thought/reasoning frames are redacted or policy-gated by default and are covered by negative leak tests.
- No network provider, private chain-of-thought exposure, production-readiness, or provider-native raw-stream API claim is made.
