# Evidence: adapt-openai-codex-hook-trust-lifecycle

## Baseline

- `cargo test -p animus-runtime` passed before core edits: 125 tests, 0 failures.
- `nix run .#cairn -- validate --root .` passed before implementation.
- `nix run .#cairn -- gate proposal adapt-openai-codex-hook-trust-lifecycle --root .`, `gate design`, and `gate tasks` passed before implementation.
- `scripts/cairn/check-roadmap --root .` had a pre-existing roadmap issue: `ROADMAP_UNKNOWN_CHANGE_ID adapt-claude-code-agent-teams`. It did not report `adapt-openai-codex-hook-trust-lifecycle` as blocked.

## Implementation evidence

- Added a pure lifecycle hook core in `crates/animus-runtime/src/hooks/lifecycle.rs` for typed hook events, BLAKE3 content-addressed definitions, scoped trust records, redacted hook inputs, event-scoped outputs, policy binding, and outcome reports.
- Added positive and negative tests for routing, unknown events, changed hashes, managed policy coverage, event-scoped outputs, malformed/unsupported effects, policy-bypass attempts, redacted reports, denied/untrusted route outcomes, and required-service-unavailable failures.
- Added `RuntimeEvent::LifecycleHookOutcome` and `RuntimeEventKind::LifecycleHookOutcome` for report projection.
- Added `docs/openai-codex-hook-trust-lifecycle.md` and linked it from `README.md`.

## Verification

- `cargo fmt` passed.
- `cargo test -p animus-runtime` passed after implementation: 140 tests, 0 failures.
- `nix run .#cairn -- validate --root .` passed after implementation.
- `nix run .#cairn -- gate proposal adapt-openai-codex-hook-trust-lifecycle --root .`, `gate design`, and `gate tasks` passed after task completion.
- `nix run .#cairn -- sync adapt-openai-codex-hook-trust-lifecycle --root .` dry-run was unblocked.
- `nix run .#cairn -- sync adapt-openai-codex-hook-trust-lifecycle --root . --execute` reported `mutated: true`, but the accepted spec content hash was unchanged, so the delta requirements were manually merged into `cairn/specs/agent-harness-runtime/spec.md` and revalidated.

## Blocked rails

- `cargo clippy -p animus-runtime --all-targets -- -D warnings` is blocked by pre-existing `animus-adapters` clippy errors.
- `cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` is blocked by pre-existing `animus-runtime` clippy errors outside the lifecycle hook changes, including missing `# Errors` docs in worker-team APIs.
