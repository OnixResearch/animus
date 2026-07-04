# Evidence: adapt-openai-codex-jsonl-exec-sdk

## Implementation evidence

- Added `crates/animus-runtime/src/automation.rs` with typed non-interactive run requests, versioned automation event envelopes, JSONL rendering, Preserves-compatible event tags, final-output selection, schema-identified structured-output validation, thread snapshots, controller trait, and resume/cancel guards.
- Added `crates/animus-runtime/src/automation/tests.rs` with positive request/event/final-output/structured/controller coverage and negative missing-prompt, invalid-sandbox, duplicate-context, resume-mismatch, out-of-order event, oversized event body, missing/failure terminal output, malformed schema, missing field, extra field, ledger mismatch, wrong thread, and non-running cancellation cases.
- Added `docs/openai-codex-jsonl-exec-sdk.md` and README link.

## Validation evidence

- Focused implementation before Octet split: `cargo fmt && cargo test -p animus-runtime automation` — PASS, 6 tests.
- Focused after headless split: `cargo test -p animus-runtime headless` — PASS, 7 matching tests.
- Runtime suite/check: `cargo test -p animus-runtime && cargo check -p animus-runtime --all-targets` — PASS, 39 tests.
- Final touched-crate/workspace check: `cargo test -p animus-adapters && cargo test -p animus-runtime && cargo check --workspace --all-targets` — PASS.
- Final repository hook check: `pre-commit run --all-files` — PASS (`Nickel generated configs`, `Octet lints`).
