# Evidence: adapt-pi-session-tree-ledger

## Implementation evidence

- Extended `crates/animus-adapters/src/ledger/` with neutral session-tree nodes, active-leaf snapshots, replay-neutral labels, explicit compaction/branch-handoff summary kinds, active-branch replay, summary span replacement, and typed fail-closed errors for duplicate IDs, missing parents, cycles, invalid active leaves, unsupported entries, orphaned tool results, malformed summary boundaries, missing label targets, missing ledger services, missing sessions, and unavailable ledger services.
- Added crate-level aliases and helpers for `LedgerNode`, `LedgerLabel`, `LedgerSnapshot`, `LedgerSummaryKind`, `replay_session_tree`, and typed missing-service/session/unavailable errors.
- Added positive and negative ledger tests covering linear replay, branch selection, sibling exclusion, tool-result ordering, summary replacement, labels as metadata-only, duplicate IDs, missing parents, cycles, invalid active leaves, bad label targets, orphaned tool results, malformed summaries, unsupported/adapter-owned entries, empty ledgers, and missing service/session errors.
- Added `docs/pi-session-tree-ledger.md` and README link documenting DTO field names, active-leaf replay rules, labels, summary semantics, and non-claims.

## Validation evidence

- Baseline before ledger extension: `nix develop -c cargo test -p animus-adapters ledger --locked` — PASS, 5 ledger tests.
- Focused after ledger extension: `nix develop -c cargo fmt --check` — PASS.
- Focused after ledger extension: `nix develop -c cargo test -p animus-adapters ledger --locked` — PASS, 11 ledger tests.
- Touched adapter suite: `nix develop -c cargo test -p animus-adapters --locked` — PASS, 254 tests.
- Runtime dependent suite: `nix develop -c cargo test -p animus-runtime --locked` — PASS, 41 tests.
- Adapter clippy probe: `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings` — BLOCKED by pre-existing dirty-worktree lint failures outside `crates/animus-adapters/src/ledger/`; rerun output contained no `ledger/` or `ledger::` diagnostics after local ledger cleanup.
