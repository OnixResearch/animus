## Phase 1: Contracts and configuration

- [x] [parallel] [task:crate] Add the non-default `animus-std-pueue` crate and workspace feature isolation. r[agent_harness_runtime.pueue_host.component]
- [x] [parallel] [task:config] Add typed Nickel configuration for namespace, programs, capabilities, bounds, and process-job profiles. r[agent_harness_runtime.pueue_host.configuration]
- [x] [serial] [task:admission] [after:config] Bind queue requests to exact reviewed process-job profiles and explicit controls. r[agent_harness_runtime.pueue_host.admission]

## Phase 2: Imperative host

- [x] [serial] [task:process] [after:admission] Implement the injected bounded-process pueue client and daemon lifecycle boundary. r[agent_harness_runtime.pueue_host.execution]
- [x] [parallel] [task:projection] [after:process] Parse bounded backend output into existing Animus queue responses and failure classes. r[agent_harness_runtime.pueue_host.projection]
- [x] [parallel] [task:isolation] [after:process] Enforce one explicit capability-scoped namespace without ambient fallback. r[agent_harness_runtime.pueue_host.isolation]
- [x] [serial] [task:tool] [after:projection] Add the pueue tool definition and `ToolExecutorService` adapter. r[agent_harness_runtime.pueue_host.tool]
- [x] [serial] [task:live-shell-tool] [after:tool] Admit the exact optional host-tool operation in live-agent configuration. r[agent_harness_runtime.pueue_host.live_shell_tool_route]

## Phase 3: Evidence and publication

- [x] [parallel] [task:positive-tests] [after:tool] Add positive fake-process and isolated real-daemon tests for every admitted operation. r[agent_harness_runtime.pueue_host.verification]
- [x] [parallel] [task:negative-tests] [after:tool] Add negative tests for raw commands, implicit targets, ambient state, malformed output, timeout, cancellation, and leaks. r[agent_harness_runtime.pueue_host.verification]
- [x] [serial] [task:catalog] [after:positive-tests] [after:negative-tests] Update component, contract, runtime facade, and conformance catalogs. r[agent_harness_runtime.pueue_host.publication]
- [ ] [serial] [task:gates] [after:catalog] Run formatting, crate tests, workspace checks, Clippy, Octet, Cairn gates, and focused Nix checks. r[agent_harness_runtime.pueue_host.verification]
