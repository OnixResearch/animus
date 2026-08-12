# Evidence: add-ledger-session-adapter-seam

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed before implementation; unrelated blockers remained for `add-tool-executor-kit` and `split-std-host-adapter-crates` waiting on `split-runtime-kernel-feature-packs`.
- `nix run .#cairn -- gate proposal add-ledger-session-adapter-seam --root .`, `gate design`, and `gate tasks` passed before implementation.
- `scripts/cairn/check-request-lifecycle --root .`, `scripts/cairn/check-safe-receipts-artifacts --root .`, and `scripts/cairn/check-component-catalog --root .` passed before implementation.
- `cargo test -p animus-runtime session_persistence --locked` passed before implementation with 3 tests.

## Implementation evidence

- Extended `animus-runtime::session_persistence` with host-injected `LedgerSessionService` descriptors, `LedgerReplayRequest`, `SessionRestoreRequest`, `LedgerEntry`, `LedgerMutationRecord`, pure `plan_ledger_replay`, pure `plan_session_restore`, and `validate_ledger_session_receipt`.
- Kept filesystem, database, network, clock, process, credential, terminal, and persistence effects out of reusable runtime code. Hosts must supply descriptors and already-loaded ledger entries.
- Added positive tests for safe replay, safe restore planning, bounded context projection, omitted stale/private/unsafe entries, and safe receipt validation.
- Added negative tests for missing services, ambient global-store fallback, malformed sequence ranges, stale snapshots, conflicting branches, and receipt leak rejection.
- Updated `docs/request-lifecycle.md`, `docs/session-restore-contract.md`, `docs/provenance-ledger.md`, `docs/dogfoodable-host-readiness-roadmap.md`, `docs/safety-contract.md`, `README.md`, `cairn/component-catalog.ncl`, `docs/component-catalog.md`, and `cairn/runtime-facade-inventory.ncl`.

## Verification

- `cargo fmt --check` passed after implementation.
- `cargo test -p animus-runtime session_persistence --locked` passed after implementation with 6 tests.
- `scripts/cairn/check-request-lifecycle --root .` passed after docs updates.
- `scripts/cairn/check-safe-receipts-artifacts --root .` passed after receipt updates.
- `scripts/cairn/check-component-catalog --root .` passed after catalog updates.
- `scripts/cairn/check-runtime-facade-inventory --root .` passed after facade classification updates.
- `scripts/cairn/check-dogfood-readiness --root .` passed after readiness-roadmap updates.
- `nix run .#cairn -- sync add-ledger-session-adapter-seam --root .` dry-run was unblocked.
- `nix run .#cairn -- sync add-ledger-session-adapter-seam --root . --execute` completed, but the accepted spec text was not promoted by the sync tool; the requirement delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` and verified with `rg 'ledger_session_adapter'`.
- `nix run .#cairn -- validate --root .` passed after accepted-spec merge.

## Non-claims

- This change does not add durable storage, a default global session directory, file/database/network persistence, storage migrations, daemon attach, native provider resume, TUI restore, live process transfer, production restore, or dogfood readiness beyond the existing fake-backed rung.
