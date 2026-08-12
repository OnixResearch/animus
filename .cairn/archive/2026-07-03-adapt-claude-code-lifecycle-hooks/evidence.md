# Evidence: adapt Claude Code lifecycle hooks

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation with 18 active changes.
- `scripts/cairn/check-roadmap --root .` initially failed on the pre-existing stale archived roadmap entry `adapt-claude-code-agent-teams`; the stale classification was removed and roadmap validation then passed with only blocked dependents reported.
- `nix run .#cairn -- gate proposal adapt-claude-code-lifecycle-hooks --root .`, `gate design`, and `gate tasks` passed before implementation.
- `cargo test -p animus-runtime lifecycle_hook --locked` passed with 0 matching tests before implementation.
- `cargo test -p animus-runtime hook --locked` passed before implementation with 22 matching hook tests.

## Implementation evidence

- Extended `LifecycleHookEventKind` with Claude hook points for session end, prompt assembly, model request/response, mutation pre/post, validation, receipt emission, and workflow completion while preserving existing OpenAI/Codex hook labels.
- Added `LifecycleHookCapability` declarations to hook definitions and bound definition hashes to normalized capabilities.
- Added safe hook input summaries alongside event IDs, resource classes, redacted labels, body sizes, statuses, BLAKE3 body hashes, and redaction classes.
- Added request-revalidation hook effects and `LifecycleHookOutput::validate_for_definition` so hook outputs must match both event contracts and declared host-owned capabilities.
- Added documentation in `docs/claude-code-lifecycle-hooks.md` and README linkage.

## Verification

- `cargo fmt` passed.
- `cargo test -p animus-runtime hook --locked` passed after implementation with 26 matching hook tests.
- `cargo test -p animus-runtime --locked` passed after implementation.
