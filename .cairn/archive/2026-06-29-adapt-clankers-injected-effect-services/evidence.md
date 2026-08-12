# Evidence: adapt-clankers-injected-effect-services

## Implementation evidence

- Added `animus-adapters` injected service contracts for model/provider dispatch, tool execution, host effects, capability decisions, cancellation, confirmation, and runtime policy checks.
- Added typed `ServiceOutcome` classes for absent, unavailable, denied, cancelled, timed-out, malformed-output, and completed service results.
- Added default-deny confirmation behavior through `default_deny_confirmation` with redacted receipts.
- Added `Receipt` and `SafeHash` DTOs that carry typed service kind, capability, outcome, reason, hash/count metadata, and redaction markers without raw provider bodies, credentials, environment values, full paths, raw tool args, or raw tool output.
- Updated the runtime shell so absent injected model/tool services fail closed before later dispatch.

## Verification evidence

- Baseline before edits: `nix develop -c cargo test -p animus-adapters -p animus-runtime` passed.
- After edits: `nix develop -c cargo test -p animus-adapters -p animus-runtime` passed.
- After edits: `nix develop -c cargo clippy -p animus-adapters -p animus-runtime --all-targets -- -D warnings` passed.
- Boundary scan: no matches for ambient filesystem, environment, process, network, client-construction, async, or direct file-read tokens in `crates/animus-runtime/src`.

## Non-claims

This evidence does not claim real provider implementations, plugin runtime execution, OAuth, daemon sessions, product tool catalogs, live network behavior, TUI behavior, or production readiness.
