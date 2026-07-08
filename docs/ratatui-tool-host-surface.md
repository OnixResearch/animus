# Ratatui-compatible tool host surface

Animus keeps terminal UI dependencies at the host shell boundary. `animus-tui` is the only crate that may depend on Rats/Subwayrat/Ratatui-facing packages; reusable crates (`animus-core`, `animus-adapters`, `animus-steel`, `animus-runtime`, and default adapter contracts) stay terminal-free.

## Boundary

- `animus-tui` owns host-only terminal dependencies and re-exports the pinned Subwayrat widget and keymap packages for TUI hosts.
- UI-neutral bridge DTOs describe hosted tool lifecycle events, progress, bounded output, approval requests, correlated commands, cancellation, final responses, and typed failures.
- Tools and reusable runtime code exchange validated `ToolRequest` and `ToolResponse` values. They never receive terminal handles, Ratatui frames, key events, raw-mode controls, redraw loops, or Rats widget types.
- Terminal setup, cleanup, polling, channels, keybindings, and rendering backends remain imperative-shell concerns outside reusable contracts.

## Reducer and hosted execution

`ToolRenderModel` is the functional core. It applies `ToolHostEvent` and `RuntimeEvent` values to derive visible session/tool state. `render_text_snapshot` is the deterministic test-backend equivalent used by focused tests; live Ratatui rendering can consume the same model without changing tool contracts.

`run_simple_tool` preserves the existing noninteractive `ToolAdapter` path. `run_hosted_tool` wraps that path with lifecycle events. `run_guarded_tool` requires finite approval commands with a correlation identity and fails closed before adapter execution when the host denies, cancels, drops the event consumer, closes the command channel, or returns a mismatched command.

## Focused rail

Run the focused checks with:

```sh
cargo test -p animus-tui
scripts/cairn/check-ratatui-tool-host-surface --root .
```

The rail verifies that the Subwayrat dependencies are SSH-pinned in `animus-tui`, `animus-tui` does not bypass Subwayrat with a direct Crossterm dependency, reusable crate manifests and public exports remain terminal-free, and bridge/reducer code does not import terminal backend types.

## Non-claims

This surface does not dogfood a live terminal, start a daemon, implement raw-mode setup, claim production TUI readiness, replace headless/RPC protocols, or allow tools to draw directly to the terminal.
