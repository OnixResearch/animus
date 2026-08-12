# agent-harness-runtime Change Spec: TUI app shell and event loop

## Requirements

### Requirement: Isolated terminal lifecycle

r[agent_harness_runtime.tui_shell.terminal_lifecycle] Crossterm setup, raw mode, alternate screen, and teardown MUST live in `animus-tui`, and terminal setup failures, render errors, dropped consumers, and closed channels MUST surface as typed host errors or cancellation events without corrupting runtime state.

#### Scenario: Terminal setup failure cancels the run safely

r[agent_harness_runtime.tui_shell.terminal_lifecycle.setup_failure]
- GIVEN crossterm raw-mode or alternate-screen setup fails
- WHEN the shell handles the failure
- THEN it returns a typed terminal-setup error and tears down partial state
- AND no runtime turn, model call, or tool dispatch is left running uncontrolled.

### Requirement: Event loop routes events to reducers and runtime

r[agent_harness_runtime.tui_shell.event_loop] The event loop MUST poll terminal events, route them to prompt and panel reducers, route runtime/model/tool events to the render model, and route prompt/tool/control commands back to the `animus-runtime` turn loop, with coalesced redraws scheduled by the shell.

#### Scenario: Prompt-to-loop round-trip drives the runtime

r[agent_harness_runtime.tui_shell.event_loop.round_trip]
- GIVEN the harness is booted and the user submits a prompt
- WHEN the event loop routes the submit command to the runtime turn loop
- THEN the loop emits model/tool events that update the render model
- AND the shell renders the streaming transcript without blocking the runtime.

### Requirement: Pane layout consumes host-control DTOs

r[agent_harness_runtime.tui_shell.layout] Pane/split layout MUST use `ratatui-hypertile` consuming host-control-protocol layout DTOs, layout types MUST stay host-only, and layout mismatches MUST fail closed with a typed diagnostic.

#### Scenario: Layout mismatch fails closed

r[agent_harness_runtime.tui_shell.layout.mismatch]
- GIVEN a layout apply request references panes absent from the current topology
- WHEN the shell applies the layout
- THEN it reports a typed layout-mismatch diagnostic
- AND no partial or corrupted pane layout is rendered.

### Requirement: Host-only peripherals and theme

r[agent_harness_runtime.tui_shell.peripherals] Clipboard, mouse, theme, and scrollbar registry types MUST live only in `animus-tui`, and reusable crates MUST NOT import peripheral types.

#### Scenario: Peripheral types stay in the TUI crate

r[agent_harness_runtime.tui_shell.peripherals.host_only]
- GIVEN maintainers inspect reusable crate dependencies and public exports
- WHEN the peripherals and theme are added
- THEN clipboard, mouse, theme, and scrollbar types appear only in `animus-tui`
- AND reusable crates import no peripheral types.

### Requirement: Downstream-only runtime binding

r[agent_harness_runtime.tui_shell.runtime_binding] The shell MUST boot via `animus-std::load_runtime` and drive the `animus-runtime` turn loop with injected adapters, and MUST NOT construct providers, read credentials, spawn processes, or open network connections directly.

#### Scenario: Shell does not construct providers or read credentials

r[agent_harness_runtime.tui_shell.runtime_binding.no_ambient]
- GIVEN the shell boots the harness
- WHEN focused boundary tests run
- THEN provider construction, credential reads, process spawns, and network opens are absent from `animus-tui`
- AND all such effects flow through injected std-shell adapters.

### Requirement: Terminal failures do not corrupt runtime state

r[agent_harness_runtime.tui_shell.failure_isolation] Dropped event consumers, closed command channels, cancellations, and layout mismatches MUST surface as typed host errors or cancellation events without corrupting core state, bypassing tool policy, or leaking unbounded output.

#### Scenario: Dropped consumer cancels interactive wait

r[agent_harness_runtime.tui_shell.failure_isolation.dropped_consumer]
- GIVEN an interactive tool is waiting for a host approval decision
- WHEN the TUI event consumer or command channel closes
- THEN the hosted execution path returns a typed cancellation or host-unavailable failure
- AND no pending guarded action is executed by default.

### Requirement: App shell verification

r[agent_harness_runtime.tui_shell.verification] The change MUST include focused positive integration tests over a fake terminal backend and fake runtime host for boot, prompt-to-loop round-trip, streaming render, tool approval, and shutdown, plus negative tests for setup failure, dropped consumers, closed channels, cancellation, layout mismatch, and reusable-crate contract leakage before closeout.

#### Scenario: Closeout evidence covers app-shell fixtures

r[agent_harness_runtime.tui_shell.verification.closeout]
- GIVEN maintainers intend to archive the app shell and event loop change
- WHEN closeout validation runs
- THEN evidence names focused integration, failure-isolation, dependency-isolation, workspace, Cairn validation, and Cairn gate checks
- AND any skipped live-terminal dogfood check is recorded as a non-claim with a blocker and next-best validation.
