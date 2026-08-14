# Design: Live terminal agent host

## Context

The current TUI smoke renders a fixed snapshot with Ratatui `TestBackend`. It proves view projection and bounded rendering, not raw mode, real input, resizing, or cleanup.

The accepted daemon split assigns rendering and input to the client. The new terminal host must obey that role.

## Success Contract

A terminal client can enter raw mode, attach to a local session, render snapshot-first events, send bounded commands, handle resize, and restore the terminal on every owned exit path.

Tool execution, provider dispatch, session mutation, and durable state remain behind client APIs. A widget or key handler cannot call these effects directly.

A client that renders once without input, leaves raw mode active after failure, or owns session execution is false completion.

## Decisions

### 1. Use a terminal lifecycle guard

A small shell owns alternate-screen entry, raw mode, cursor state, backend construction, restoration, and supported signal coordination.

Restoration is idempotent. Cleanup errors remain visible and can prevent a successful terminal receipt.

### 2. Keep the event loop explicit

The loop selects among bounded terminal input, resize, timer ticks, daemon or direct-session events, cancellation, and shutdown.

Each iteration performs at most one input reduction, one bounded event batch, and one render decision under named limits.

### 3. Prefer daemon attachment

Normal mode uses the local daemon client for snapshot, events, and commands. An explicit direct local mode can bind the same client-facing interface to one in-process live shell.

There is no implicit daemon discovery or automatic switch between modes.

### 4. Keep commands as DTOs

Keybindings and UI controls produce typed command descriptors for prompts, queue compare-and-swap, approvals, cancellation, child controls, and session operations.

The client sends these descriptors through the selected connection. It does not execute their effects.

### 5. Separate rendering from evidence

The UI can show admitted model and tool content according to view policy. Durable receipts contain only safe status, counts, dimensions, lifecycle observations, component identities, and non-claims.

The complete terminal buffer and user input never enter receipts.

### 6. Use layered validation

Pure reducers and `TestBackend` remain required. A controlled pseudo-terminal harness checks raw mode, resize, input, disconnect, and restoration.

A manual live run is separate host evidence and is not required by credential-free default checks.

## Client Flow

```text
explicit terminal configuration
  -> terminal lifecycle guard
  -> daemon or direct connection
  -> snapshot-first state
  -> bounded input and event reductions
  -> Ratatui render
  -> typed commands through client API
  -> idempotent terminal restore
```

## Alternatives Considered

### Put terminal handling in the live agent shell

Rejected because this mixes client UI authority with session execution authority.

### Treat Drop alone as complete cleanup

Rejected because supported signals, backend errors, and partial initialization need explicit observations.

### Auto-start or auto-discover the daemon

Rejected because process and endpoint authority must stay explicit.

## Risks and Controls

- A crash can leave terminal state changed. Idempotent guards and pseudo-terminal failure tests cover owned paths.
- Event floods can starve input. Bounded batches and explicit selection policy preserve responsiveness.
- Keybindings can bypass admission. They emit DTOs only.
- Visible content can enter receipts. Safe evidence projection excludes buffers and input.

## Non-Claims

This design does not establish arbitrary terminal compatibility, remote security, crash-proof restoration, PTY migration, sandboxing, task correctness, production readiness, or release eligibility.
