# Evidence: adapt-clankers-neutral-session-ledger

## Implementation evidence

- Added no-std neutral ledger DTOs in `animus-adapters` for model-visible user, assistant, tool, and summary entries; safe usage observations; safe receipt entries; unsupported entries; adapter-owned markers; replay requests; replay reports; counts; and typed replay errors.
- Added pure in-memory `replay_ledger` that preserves supported model-visible order and emits safe metadata counts without filesystem, process, network, provider, storage, or terminal authority.
- Added fail-closed handling for unsupported and adapter-owned entries that returns safe entry IDs, kind labels, and redaction markers without retaining raw adapter-owned payloads.
- Added a `SessionLedgerService` adapter contract plus `missing_session_ledger_service` absent-service outcome for resume paths that require an injected ledger service before model/tool dispatch.
- Did not add branch parent/head fields; the branch-query task is satisfied by omission, so duplicate-parent/cycle/dead-end semantics remain out of scope until a branch graph DTO is accepted.
- Added `scripts/cairn/check-neutral-ledger` plus a Nix `neutral-ledger` check and README verification entry for the focused replay rail.

## Verification evidence

- Baseline before edits: `nix develop -c cargo test -p animus-adapters` passed.
- After edits: `nix develop -c cargo test -p animus-adapters` passed.
- After edits: `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` passed.
- Focused rail: `nix develop -c scripts/cairn/check-neutral-ledger --root .` passed.
- Nix rail: `nix build .#checks.x86_64-linux.neutral-ledger --no-link -L` passed after staging the new source and rail files into the flake source snapshot.

## Non-claims

This evidence does not claim durable ledger storage, branch graph replay, Automerge integration, daemon sessions, live provider replay, TUI session restore, distributed synchronization, or production readiness.
