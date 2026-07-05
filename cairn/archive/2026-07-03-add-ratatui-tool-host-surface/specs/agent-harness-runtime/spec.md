# agent-harness-runtime Change Spec: Ratatui-compatible tool host surface

## Requirements

### Requirement: Optional TUI host dependency boundary

r[agent_harness_runtime.ratatui_tool_host.boundary] Animus MUST keep Rats, Ratatui, Crossterm, terminal raw-mode handling, redraw scheduling, and keybinding interpretation out of `animus-core`, `animus-adapters`, `animus-steel`, and reusable default tool contracts. A Ratatui-compatible shell MAY exist as an optional host crate or binary that depends on the reusable runtime contracts.

#### Scenario: Reusable crates remain terminal-free

r[agent_harness_runtime.ratatui_tool_host.boundary.terminal_free]
- GIVEN maintainers inspect reusable Animus crate dependencies and public DTOs
- WHEN a Ratatui-compatible host is added
- THEN terminal UI crates, Rats crates, and terminal-specific types appear only in the optional host shell
- AND ordinary model and tool adapters remain usable without linking a terminal backend.

### Requirement: Rats dependency is host-only and SSH-pinned

r[agent_harness_runtime.ratatui_tool_host.rats_dependency] The optional TUI host crate MUST depend on the Rats workspace through an SSH-authenticated Cargo Git URL pinned to an explicit revision, and MUST NOT expose Rats types through reusable core, adapter, Steel, std runtime, or tool contract APIs.

#### Scenario: Rats dependency stays inside TUI crate

r[agent_harness_runtime.ratatui_tool_host.rats_dependency.host_only]
- GIVEN the optional TUI host dependency graph is inspected
- WHEN the Rats dependency is resolved from `ssh://git@github.com/brittonr/rats.git`
- THEN only the TUI host crate depends on Rats packages
- AND the dependency is pinned to a concrete revision rather than a floating branch.

### Requirement: UI-neutral tool host bridge

r[agent_harness_runtime.ratatui_tool_host.bridge] Animus MUST define a UI-neutral tool host bridge for interactive tool execution with typed lifecycle events, progress updates, bounded output observations, approval requests, cancellation notices, final results, and failure classes.

#### Scenario: Tool progress renders without direct drawing

r[agent_harness_runtime.ratatui_tool_host.bridge.progress]
- GIVEN an interactive tool emits started, progress, output, and finished events through the host bridge
- WHEN a Ratatui host receives those events
- THEN it updates its render model from the typed events
- AND the tool never receives a Ratatui frame, terminal handle, or direct drawing callback.

### Requirement: Hosted execution remains adapter-compatible

r[agent_harness_runtime.ratatui_tool_host.adapter_compat] Hosted tools MUST execute around validated `ToolRequest` and `ToolResponse` values and MUST preserve the existing simple `ToolAdapter` path for noninteractive tools.

#### Scenario: Simple tool runs without TUI services

r[agent_harness_runtime.ratatui_tool_host.adapter_compat.simple]
- GIVEN a noninteractive tool implements the existing simple tool adapter contract
- WHEN the runtime dispatches the tool in a headless or test host
- THEN the tool runs without requiring any TUI host service
- AND the response still crosses the validated `ToolResponse` boundary.

### Requirement: Interactive commands are finite and correlated

r[agent_harness_runtime.ratatui_tool_host.commands] Interactive tool requests for host input MUST expose finite command choices with a correlation identity, and hosts MUST answer with typed decisions such as approve, deny, continue, or cancel rather than unbounded terminal-specific input.

#### Scenario: Approval denial fails safely

r[agent_harness_runtime.ratatui_tool_host.commands.denied]
- GIVEN a tool requests approval for a capability-bearing action through the host bridge
- WHEN the Ratatui host returns a denial decision for the matching correlation identity
- THEN runtime reports a typed denied or cancelled tool result
- AND the guarded tool action is not executed after the denial.

### Requirement: Render model is pure and replayable

r[agent_harness_runtime.ratatui_tool_host.render_model] The Ratatui-compatible shell MUST derive visible tool/session state by applying a pure reducer over typed runtime and tool-host events, with terminal I/O, polling, cleanup, channels, and redraw orchestration isolated in the shell.

#### Scenario: Test backend reproduces visible state

r[agent_harness_runtime.ratatui_tool_host.render_model.test_backend]
- GIVEN a recorded sequence of runtime and tool-host events
- WHEN the render reducer and a Ratatui test backend render the sequence
- THEN the visible panels show the expected session summary, active tool state, approval prompt, progress, final result, or failure state
- AND the test does not require a real terminal.

### Requirement: Terminal failures do not corrupt runtime state

r[agent_harness_runtime.ratatui_tool_host.failure_isolation] Terminal setup failures, render errors, dropped event consumers, closed command channels, and cancellation signals MUST surface as typed host errors or cancellation events without corrupting core state, bypassing tool policy, or leaking unbounded tool output.

#### Scenario: Dropped TUI consumer cancels interactive wait

r[agent_harness_runtime.ratatui_tool_host.failure_isolation.dropped_consumer]
- GIVEN an interactive tool is waiting for a host approval decision
- WHEN the TUI event consumer or command channel closes
- THEN the hosted execution path returns a typed cancellation or host-unavailable failure
- AND no pending guarded action is executed by default.

### Requirement: TUI host verification rails

r[agent_harness_runtime.ratatui_tool_host.verification] The TUI host surface MUST include focused positive and negative verification for dependency isolation, host bridge event handling, simple adapter compatibility, approval denial, cancellation, pure render reduction, terminal test-backend rendering, and safe failure isolation before closeout.

#### Scenario: Closeout evidence covers TUI and non-TUI hosts

r[agent_harness_runtime.ratatui_tool_host.verification.closeout]
- GIVEN maintainers intend to archive the Ratatui-compatible tool host change
- WHEN closeout validation runs
- THEN evidence names focused bridge tests, render reducer tests, Ratatui test-backend tests, dependency-isolation checks, workspace checks, Cairn validation, and Cairn gates
- AND any skipped TUI dogfood or live terminal check is recorded as a non-claim with a blocker and next-best validation.
