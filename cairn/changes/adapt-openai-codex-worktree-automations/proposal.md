# Proposal: Adapt OpenAI Codex worktree automations for Animus

## Why

OpenAI Codex uses Git worktrees to isolate background tasks and automations from the foreground checkout. It also distinguishes standalone scheduled automations from thread-bound wakeups. Those are useful patterns, but Animus must not assume Git-only workflows because this workspace defaults to Jujutsu where available and should remain VCS-neutral.

Animus should adapt the isolation and automation semantics as an abstraction over task workspaces, snapshots, schedules, and handoff rather than copying Codex's Git-specific implementation.

## What Changes

- Define a VCS-neutral isolated task workspace contract for background agent runs, including source revision, writable roots, snapshot identity, and cleanup policy.
- Add automation run definitions for scheduled standalone runs and thread-bound wakeups.
- Add handoff semantics for moving a thread between foreground and isolated workspace contexts with explicit snapshot evidence.
- Add allowlisted ignored-file materialization for local-only setup inputs, with redaction and provenance metadata.
- Record automation findings, no-op outcomes, workspace cleanup, and restore evidence as typed runtime events.

## Impact

- **Files**: workspace isolation DTOs, automation schedule/run contracts, snapshot and cleanup records, VCS adapter interfaces, docs, and positive/negative tests.
- **Testing**: positive tests for isolated runs, thread wakeups, handoff snapshots, and cleanup records; negative tests for dirty-state conflicts, missing source revisions, unsafe ignored-file copying, failed restores, and Git-only assumptions.
- **Non-claims**: this change does not add PR creation, Codex app UI, Git-only worktree management, or unattended full-access execution by default.
