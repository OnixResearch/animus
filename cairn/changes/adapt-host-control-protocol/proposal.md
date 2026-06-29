# Proposal: Adapt host control protocol for Animus

## Why

Herdr exposes workspaces, tabs, panes, agents, layout operations, bounded pane reads, input sends, waits, and event subscriptions through a local socket and CLI wrappers. Animus should adapt that control shape for future headless, SDK, TUI, and orchestration hosts, while keeping transport, terminal state, and process ownership outside reusable crates.

The useful pattern is a typed host-control boundary: agents and scripts can inspect or coordinate host-owned execution surfaces without needing private UI state or unsafe ambient shell access.

## What Changes

- Define UI-neutral host-control DTOs for workspaces, tabs, panes, agents, layout snapshots, layout apply requests, reads, sends, waits, and focus/rename/close operations.
- Treat public host IDs as live-session selectors unless a host explicitly supplies durable IDs; callers must re-resolve IDs after topology changes.
- Define bounded read surfaces for visible, recent, unwrapped recent, and detector-safe output classes without storing unbounded transcripts in runtime state.
- Define event subscription and one-shot wait contracts over semantic runtime and agent states, including filters, correlation IDs, cancellation, and timeout outcomes.
- Keep sockets, named pipes, CLI wrappers, PTYs, process input, terminal rendering, and workspace storage in host adapters.

## Impact

- **Files**: future headless/TUI host protocol DTOs, layout tree contracts, event subscription contracts, host adapter traits, bounded-output fixtures, docs, and tests.
- **Testing**: positive command validation, layout export/apply, read/wait, and subscription tests; negative stale ID, cross-host selector, unbounded read, unsupported send, timeout, malformed layout, and transport-version tests.
- **Non-claims**: this does not copy Herdr's JSON socket protocol, implement a terminal multiplexer, add PTY management to core, or require a particular IPC transport.
