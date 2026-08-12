# Design: TUI app shell and event loop

## Context

Cairns 1–6 provide the pure and host-only pieces of a pi-style agent TUI. What remains is the imperative shell that owns the terminal, polls events, schedules rendering, lays out panes, and binds to the runtime so the whole thing acts as an agent harness. `animus-std` already owns the runnable host (`load_runtime`, `run_with_model`), and `animus-runtime` owns the turn loop (`RuntimeLoopTurn`, fabric, shell, runner). The app shell is the downstream consumer that drives them interactively.

## Decisions

### 1. Terminal lifecycle is isolated and failure-safe

**Choice:** Crossterm setup, raw mode, alternate screen, and teardown live in `animus-tui`. Setup failures, render errors, dropped consumers, and closed channels surface as typed host errors or cancellation events without corrupting runtime state.

**Rationale:** Matches the archived failure-isolation requirement and keeps terminal concerns out of runtime logic.

### 2. Event loop routes events to reducers and back to the runtime

**Choice:** The loop polls terminal events, routes key/mouse/resize events to the prompt and panel reducers, routes runtime/model/tool events into the render model, and routes prompt/tool/control commands back to the `animus-runtime` turn loop and `animus-std` host. Redraws are coalesced and scheduled by the shell.

**Rationale:** Keeps reducers pure and the shell thin, matching the functional-core/imperative-shell split.

### 3. Pane layout consumes host-control-protocol DTOs

**Choice:** Pane/split layout uses `ratatui-hypertile` and consumes `LayoutNode`/`LayoutApplyRequest` DTOs from the host-control protocol. Layout types stay host-only.

**Rationale:** Reuses the accepted host-control layout contract and keeps terminal layout out of reusable crates.

### 4. Peripherals and theme are host-only

**Choice:** Clipboard, mouse handling, theme, and scrollbar registry live only in `animus-tui`.

**Rationale:** Preserves the archived dependency boundary.

### 5. Runtime binding is downstream-only

**Choice:** The shell calls `animus-std::load_runtime` to boot and drives the `animus-runtime` turn loop with an injected model/tool adapter set. The shell does not construct providers, read credentials, spawn processes, or open network connections; those remain std-shell adapter concerns.

**Rationale:** Keeps the TUI strictly downstream and the runtime the single loop authority.

### 6. No live-daemon or headless-replacement claim

**Choice:** The shell is an interactive binary entry point. It does not claim a production daemon, does not replace headless/RPC protocols, and does not open pull requests.

**Rationale:** Matches the repo's product-surface non-claim posture.

## Risks / Trade-offs

- A live event loop is hard to test fully; cover the routing reducers and a fake-terminal/fake-runtime integration harness, and record any live-terminal dogfood as a non-claim.
- Async/channel choices could leak into contracts; keep the runtime turn loop synchronous and data-oriented at the boundary.
- Layout mismatches between hypertile and host-control DTOs must fail closed with a typed diagnostic.
