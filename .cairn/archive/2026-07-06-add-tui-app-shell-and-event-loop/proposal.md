# Proposal: TUI app shell and event loop

## Why

The prior six TUI cairns add the conversation render model, streaming markdown view, prompt input/keymap, tool/approval panels, status rail, and control dashboard as pure or host-only components. To make `animus-tui` an actual agent harness like pi, this change adds the imperative shell that owns the terminal lifecycle, event loop, pane layout, peripherals, theme, and the binding to the `animus-std` runtime host and `animus-runtime` turn loop. It wires the components together so a user can boot the harness, type a prompt, watch the loop stream, approve tools, and steer the agent.

## What Changes

- Add crossterm setup/teardown, raw mode, and alternate-screen handling in `animus-tui`, isolated so terminal failures do not corrupt runtime state.
- Add the event loop: terminal polling, event routing to the prompt reducer, render model, and panels, plus render-loop scheduling with coalesced redraws.
- Add pane/split layout via `ratatui-hypertile` consuming host-control-protocol layout DTOs, with no layout types in reusable crates.
- Add clipboard, mouse, theme, and scrollbar registry as host-only peripherals.
- Bind the shell to `animus-std::load_runtime` and the `animus-runtime` turn loop, routing model/tool events into the render model and prompt/tool/control commands back to the runtime.
- Compose on cairns 1–6; add no new reusable-crate contracts.

## Impact

- **Files**: new `animus-tui` app, event loop, terminal lifecycle, layout, peripherals, theme, and runtime-binding modules; integration tests over a fake terminal backend and a fake runtime host.
- **Testing**: positive tests for boot, prompt-to-loop round-trip, streaming render, tool approval, and shutdown; negative tests for terminal setup failure, dropped event consumer, closed command channel, cancellation, layout mismatch, and reusable-crate contract leakage.
- **Non-claims**: this change does not dogfood a live daemon, does not replace headless/RPC protocols, does not spawn providers or workers, does not open pull requests, and does not add reusable-crate contracts.
