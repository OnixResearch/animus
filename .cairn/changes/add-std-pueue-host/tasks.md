## Phase 1: Contracts and configuration

- [ ] [parallel] [task:crate] Add the non-default `animus-std-pueue` crate and workspace feature isolation. r[agent_harness_runtime.pueue_host.component]
- [ ] [parallel] [task:config] Add typed Nickel configuration for namespace, programs, capabilities, bounds, and process-job profiles. r[agent_harness_runtime.pueue_host.configuration]
- [ ] [task:admission] [after:config] Bind queue requests to exact reviewed process-job profiles and explicit controls. r[agent_harness_runtime.pueue_host.admission]

## Phase 2: Imperative host

- [ ] [task:process] [after:admission] Implement the injected bounded-process pueue client and daemon lifecycle boundary. r[agent_harness_runtime.pueue_host.execution]
- [ ] [task:projection] [after:process] Parse bounded backend output into existing Animus queue responses and failure classes. r[agent_harness_runtime.pueue_host.projection]
- [ ] [task:isolation] [after:process] Enforce one explicit capability-scoped namespace without ambient fallback. r[agent_harness_runtime.pueue_host.isolation]
- [ ] [task:tool] [after:projection] Add the pueue tool definition and `ToolExecutorService` adapter. r[agent_harness_runtime.pueue_host.tool]

## Phase 3: Evidence and publication

- [ ] [task:positive-tests] [after:tool] Add positive fake-process and isolated real-daemon tests for every admitted operation. r[agent_harness_runtime.pueue_host.verification]
- [ ] [task:negative-tests] [after:tool] Add negative tests for raw commands, implicit targets, ambient state, malformed output, timeout, cancellation, and leaks. r[agent_harness_runtime.pueue_host.verification]
- [ ] [task:catalog] [after:positive-tests] [after:negative-tests] Update component, contract, runtime facade, and conformance catalogs. r[agent_harness_runtime.pueue_host.publication]
- [ ] [task:gates] [after:catalog] Run formatting, crate tests, workspace checks, Clippy, Octet, Cairn gates, and focused Nix checks. r[agent_harness_runtime.pueue_host.verification]
