# Tasks

## B. Terminal boundary

- [x] [serial] B1: Inventory current TUI reducers, app shell, TestBackend smoke, daemon client, and terminal dependency boundaries. r[agent_harness_runtime.live_terminal_agent_host.verification]
- [x] [serial] B2: Define typed terminal configuration, lifecycle state, input events, connection mode, limits, cleanup observations, failures, and safe receipts. r[agent_harness_runtime.live_terminal_agent_host.terminal]
- [x] [depends:B2] B3: Add the non-default terminal client package without widening reusable or default dependency graphs. r[agent_harness_runtime.live_terminal_agent_host.terminal]

## L. Live client shell

- [x] [depends:B3] L1: Implement idempotent partial-start and full-start terminal guards with explicit restoration. r[agent_harness_runtime.live_terminal_agent_host.cleanup]
- [x] [depends:L1] L2: Implement bounded input, resize, tick, session-event, render, cancellation, disconnect, and shutdown orchestration. r[agent_harness_runtime.live_terminal_agent_host.event_loop]
- [x] [depends:L2] L3: Bind explicit daemon and direct local modes to one client-facing interface without automatic fallback. r[agent_harness_runtime.live_terminal_agent_host.connection]
- [x] [depends:L3] L4: Map keybindings and controls to typed prompt, queue, approval, cancellation, child, and session commands only. r[agent_harness_runtime.live_terminal_agent_host.commands]
- [x] [depends:L4] L5: Project snapshot-first streaming state through existing view reducers and metadata-only evidence. r[agent_harness_runtime.live_terminal_agent_host.event_loop]

## T. Tests and closeout

- [x] [depends:L5] T1: Extend pure and TestBackend tests for input reduction, commands, streaming, queue conflicts, child state, errors, and resize. r[agent_harness_runtime.live_terminal_agent_host.verification]
- [x] [depends:L5] T2: Add pseudo-terminal tests for raw mode, alternate screen, input, resize, disconnect, cancellation, partial startup, backend errors, supported signals, and restoration. r[agent_harness_runtime.live_terminal_agent_host.verification]
- [x] [depends:T2] T3: Add negative tests for direct effect calls, implicit daemon discovery, automatic mode fallback, event floods, unsafe receipts, and cleanup overclaims. r[agent_harness_runtime.live_terminal_agent_host.verification]
- [x] [depends:T3] T4: Add separately scoped manual live evidence, update docs and component metadata, run focused checks, and close through Cairn gates. r[agent_harness_runtime.live_terminal_agent_host.verification]
