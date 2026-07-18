# Session restore contract

Animus models session restore as typed planning and safe evidence, not as PTY transfer or implicit conversation continuity. Provider/terminal restore DTOs live in `animus-adapters`; the host-injected ledger/session replay seam lives in `animus-runtime::session_persistence`. Hosts own storage, process management, native agent resume calls, terminal state, and handoff mechanics. Coding-session restart additionally compares its committed [session runtime authority profile](session-runtime-authority-profile.md) before repair or continuation; generic restore planning does not infer or authorize that profile.

## Contracts

- `RestoreMode` names the requested survival shape: live process, snapshot layout, history replay, native session resume, handoff transfer, or safe fallback.
- `RestoreHostCapabilities` records host-declared support before a plan can claim that process state survived, layout was rebuilt, display history was replayed, a native conversation resume was requested, or handoff was attempted.
- `NativeSessionReference` stores only safe source, agent, kind, uniqueness, freshness, resume-capability, and BLAKE3 identity metadata for native session handles.
- `HistoryReplay` keeps transcript or pane history opt-in, bounded, redacted, classified by storage posture, and display-only unless a separate policy admits it as model-visible context.
- `plan_restore` is the pure planning core over in-memory snapshots, capabilities, references, and policy; it returns `RestorePlan` with no filesystem, process, terminal, network, clock, or environment authority.
- `RestoreReceipt` validates safe restore evidence and rejects raw history, credentials, shell commands, full paths, and transcript bodies.
- `LedgerSessionService`, `LedgerReplayRequest`, `SessionRestoreRequest`, `plan_ledger_replay`, `plan_session_restore`, and `LedgerSessionReceiptDraft` define the host-injected replay/restore seam over already-loaded ledger entries; missing services, stale snapshots, conflicting branches, malformed ranges, ambient global-store fallback, and unsafe receipts fail closed.

## Degraded outcomes

Unsupported capabilities, missing or malformed native references, stale or duplicated references, unsupported resume labels, disabled or unsafe history replay, unsupported handoff claims, and explicit fallbacks produce degraded outcomes. A degraded target never claims process survival, native resume, handoff success, or conversation continuity.

## Non-claims

This contract does not copy Herdr session files, persist pane transcripts by default, spawn replacement agents from reusable crates, transfer PTYs, serialize native provider handles, create a default global session store, or make a reconstructed layout equivalent to a surviving process.

## Focused checks

```sh
cargo test -p animus-adapters restore --locked
cargo test -p animus-runtime session_persistence --locked
cargo check -p animus-adapters --all-targets --locked
nix run path:$PWD#cairn -- validate --root $PWD
scripts/cairn/check-roadmap --root .
```
