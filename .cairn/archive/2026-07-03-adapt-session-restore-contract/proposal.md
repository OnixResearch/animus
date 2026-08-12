# Proposal: Adapt session restore contract for Animus

## Why

Herdr documents a useful restore matrix: live detach keeps processes running, snapshot restore brings layout back after restart, pane history replay is opt-in because it can contain secrets, native agent session restore resumes supported conversations, and live handoff is separate from restart reconstruction. Animus needs this distinction before it exposes durable sessions, headless runners, TUI panes, or hosted agent fleets.

The valuable adaptation is the survival contract, not Herdr's PTY implementation. Animus should define what a runtime or host may claim survived and what must degrade explicitly.

## What Changes

- Define restore-mode DTOs and a capability matrix for live process persistence, snapshot layout restore, bounded history replay, native agent session resume, and handoff-style process transfer.
- Define safe native session reference records with source, agent, kind, bounded value identity, freshness, uniqueness, and resume capability metadata.
- Keep pane or transcript history disabled by default; when enabled, history must be bounded, redacted, and marked as display replay rather than process survival.
- Return explicit fallback plans for unsupported, stale, duplicated, malformed, or unavailable session references rather than claiming conversation continuity.
- Keep PTY transfer, process spawning, filesystem snapshots, history storage, and native agent command execution in host adapters.

## Impact

- **Files**: future session/restore DTOs, restore planner core, ledger metadata, host capability traits, receipt fixtures, docs, and tests.
- **Testing**: positive live/snapshot/native-resume planning tests; negative unsupported reference, duplicate reference, stale reference, history leak, unavailable host service, and false-survival-claim tests.
- **Non-claims**: this does not implement Herdr live handoff, adopt Herdr session files, persist pane transcripts by default, launch native agents from core, or make PTY/process state reusable across hosts.
