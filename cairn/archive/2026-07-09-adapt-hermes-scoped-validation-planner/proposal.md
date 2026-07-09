# Proposal: Adapt Hermes scoped validation planner contracts

## Why

Hermes' auto-test extension contains a useful planner: map changed files to a small relevant validation set. The unsafe part for Animus would be baking in file scanning, shell commands, language-specific defaults, and auto-fix loops.

Animus should adapt the planner as optional SDK contracts. Hosts supply changed-artifact summaries and validation profile catalogs; reusable logic selects check intents and safe continuation outcomes without executing commands or fixing failures automatically.

## What Changes

- Define changed-artifact summaries, validation profile descriptors, validation plan items, execution result summaries, and validation follow-up intents.
- Produce deterministic scoped validation plans over host-supplied data and process-job profile labels, not raw shell commands.
- Keep filesystem walking, VCS diffing, command execution, nextest/pytest discovery, timers, UI updates, and auto-fix turns host-owned.
- Add fail-closed behavior for missing validation services, unknown profile labels, unsafe command payloads, stale changed-artifact summaries, and oversized outputs.

## Impact

- **Files**: future validation planner DTOs/reducers, docs, tests, and focused fixtures.
- **Testing**: positive scoped Rust/Node/Python-style label planning plus negative unsafe command, missing profile, missing executor, stale summary, auto-fix, and receipt leak fixtures.
- **Non-claims**: this does not add automatic testing, execute `cargo`/`npm`/`pytest`, inspect files, discover language workspaces, or trigger auto-fix by default.
