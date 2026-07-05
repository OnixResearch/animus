# Proposal: Add a Ratatui-compatible tool host surface for Animus

## Why

Animus already keeps reusable agent contracts separate from product edges: `animus-core` owns pure state, `animus-adapters` owns model/tool DTOs, `animus-steel` owns loop decisions, and `animus-std` owns process-facing orchestration. To let tools participate in an interactive terminal UI like Ratatui without coupling every tool to terminal rendering, Animus needs a host-owned TUI boundary.

This change makes the boundary explicit: tools and runtime code exchange typed UI-neutral events and commands, while a Ratatui host renders those events and owns terminal input/output.

## What Changes

- Define a UI-neutral tool host contract for tool lifecycle events, progress, bounded output, approvals, cancellation, and final results.
- Add or reserve an optional Ratatui-compatible host shell that consumes those events and renders session/tool state without exposing Ratatui or Rats types to reusable contracts.
- Route interactive tool decisions through host-injected command channels instead of direct terminal reads inside tools.
- Keep terminal setup, raw mode, alternate screen handling, Rats/Ratatui widgets, keybindings, and redraw scheduling in the TUI shell.
- Add pure reducer tests, fake-host tests, and Ratatui `TestBackend`-style rendering tests covering successful tool progress and failure/cancellation paths.

## Impact

- **Files**: future tool-host DTOs, optional `animus-tui` or equivalent host crate, Rats SSH Git dependency wiring, std-shell integration points, docs, dependency guardrails, and tests.
- **Testing**: positive tests for lifecycle/progress rendering, approval flow, and final results; negative tests for denied approvals, cancellation, terminal setup failure, dropped event consumers, malformed events, and dependency leakage into reusable crates.
- **Non-claims**: this change does not make Rats or Ratatui a core dependency, does not let tools draw directly to the terminal, does not add a daemon or headless protocol replacement, and does not adopt Clankers' product TUI as a default Animus surface.
