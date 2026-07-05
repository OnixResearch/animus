# Session restore contract

Animus models session restore as typed planning and safe evidence, not as PTY transfer or implicit conversation continuity. The reusable contract lives in `animus-adapters`; hosts own storage, process management, native agent resume calls, terminal state, and handoff mechanics.

## Contracts

- `RestoreMode` names the requested survival shape: live process, snapshot layout, history replay, native session resume, handoff transfer, or safe fallback.
- `RestoreHostCapabilities` records host-declared support before a plan can claim that process state survived, layout was rebuilt, display history was replayed, a native conversation resume was requested, or handoff was attempted.
- `NativeSessionReference` stores only safe source, agent, kind, uniqueness, freshness, resume-capability, and BLAKE3 identity metadata for native session handles.
- `HistoryReplay` keeps transcript or pane history opt-in, bounded, redacted, classified by storage posture, and display-only unless a separate policy admits it as model-visible context.
- `plan_restore` is the pure planning core over in-memory snapshots, capabilities, references, and policy; it returns `RestorePlan` with no filesystem, process, terminal, network, clock, or environment authority.
- `RestoreReceipt` validates safe restore evidence and rejects raw history, credentials, shell commands, full paths, and transcript bodies.

## Degraded outcomes

Unsupported capabilities, missing or malformed native references, stale or duplicated references, unsupported resume labels, disabled or unsafe history replay, unsupported handoff claims, and explicit fallbacks produce degraded outcomes. A degraded target never claims process survival, native resume, handoff success, or conversation continuity.

## Non-claims

This contract does not copy Herdr session files, persist pane transcripts by default, spawn replacement agents from reusable crates, transfer PTYs, serialize native provider handles, or make a reconstructed layout equivalent to a surviving process.

## Focused checks

```sh
cargo test -p animus-adapters restore --locked
cargo check -p animus-adapters --all-targets --locked
nix run path:$PWD#cairn -- validate --root $PWD
scripts/cairn/check-roadmap --root .
```
