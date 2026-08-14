# Proposal: Add a live terminal agent host

## Why

Animus has pure TUI reducers, application views, controls, and a packaged Ratatui `TestBackend` smoke. It does not own a live terminal lifecycle.

A Prime Agent-style interactive client needs raw-mode setup, input polling, rendering, command routing, resize handling, and reliable terminal restoration.

The terminal client must remain an input and rendering shell. Session execution stays in the worker or direct headless application service.

## What Changes

- Add a non-default live terminal backend and lifecycle guard around the existing TUI application shell. r[agent_harness_runtime.live_terminal_agent_host.terminal]
- Add bounded input, resize, tick, event, command, and render orchestration. r[agent_harness_runtime.live_terminal_agent_host.event_loop]
- Connect through the daemon client API, with an explicit direct local mode for test and recovery use. r[agent_harness_runtime.live_terminal_agent_host.connection]
- Preserve queue compare-and-swap, approvals, cancellation, child status, streaming, and safe view projection. r[agent_harness_runtime.live_terminal_agent_host.commands]
- Restore terminal state on normal exit, errors, cancellation, disconnect, and supported signals. r[agent_harness_runtime.live_terminal_agent_host.cleanup]
- Add deterministic backend tests, pseudo-terminal checks, and separately scoped manual live evidence. r[agent_harness_runtime.live_terminal_agent_host.verification]

## Prerequisites

- `add-live-steel-agent-shell`
- `add-local-daemon-supervisor-host`
- `add-coding-session-queue-cas`
- `agent_harness_runtime.ratatui_tool_host.render_model`
- `agent_harness_runtime.ratatui_tool_host.bridge`
- `agent_harness_runtime.model_streaming.runtime_events`
- `agent_harness_runtime.agent_control.state`

## Impact

- **Product surface**: one non-default interactive terminal client.
- **Authority**: the client owns terminal input, rendering, and local preferences only.
- **Execution**: commands cross the existing client or coding-session API. Widgets never execute tools or providers.
- **Evidence**: `TestBackend`, pseudo-terminal, and manual live claims remain distinct.

## Out of Scope

- Remote network clients, terminal multiplexing, PTY migration, arbitrary terminal compatibility, images or video, mouse support, desktop UI, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: terminal lifecycle, event loop, connection, commands, cleanup, and verification.
