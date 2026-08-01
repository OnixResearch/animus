## Phase 1: Baseline and ownership

- [x] [serial] B1 Record adapter, runtime, and standard-host baselines before core changes. r[agent_harness_runtime.agent_control.verification]
- [x] [serial] B2 Record current queued pause, runtime dispatch, cancellation, event, coding-session recovery, and host-control ownership boundaries. r[agent_harness_runtime.agent_control.port] r[agent_harness_runtime.agent_control.runtime]

## Phase 2: Contracts and pure core

- [x] [serial] C1 Add transport-neutral command, phase, checkpoint, permit, event, acknowledgement, subscription, and wait-port contracts. r[agent_harness_runtime.agent_control.port] r[agent_harness_runtime.agent_control.evidence]
- [x] [serial] C2 Add pure revisioned command and checkpoint reduction with stale-run, stale-revision, duplicate, terminal, and cancellation precedence. r[agent_harness_runtime.agent_control.state] r[agent_harness_runtime.agent_control.concurrency]
- [x] [parallel] C3 Add positive and negative reducer tests for every declared transition and rejection. r[agent_harness_runtime.agent_control.verification]

## Phase 3: Runtime and standard host

- [x] [serial] R1 Add optional loop-fabric, model, and tool safe-checkpoint gates without changing absent-control behavior or planning budgets. r[agent_harness_runtime.agent_control.runtime] r[agent_harness_runtime.agent_control.port]
- [x] [serial] H1 Add the focused `animus-std-control` crate with a mutex state latch, predicate-based condition-variable waiter, publisher, explicit closure, and bounded event journal. r[agent_harness_runtime.agent_control.host] r[agent_harness_runtime.agent_control.concurrency]
- [x] [parallel] H2 Add positive and negative race tests for resume-before-wait, spurious wakes, cancellation precedence, stale publishers, journal lag, and host closure. r[agent_harness_runtime.agent_control.host] r[agent_harness_runtime.agent_control.verification]
- [ ] [depends:add-durable-active-turn-recovery] H3 Compose durable parked control with active-turn recovery after that contract is accepted. r[agent_harness_runtime.agent_control.runtime]

## Phase 4: Composition and documentation

- [x] [serial] D1 Document control ownership, safe checkpoints, external transport bridges, in-flight policy, event subscriptions, closure, and non-claims. r[agent_harness_runtime.agent_control.port] r[agent_harness_runtime.agent_control.evidence]
- [x] [serial] D2 Update workspace, feature, facade, component, roadmap, request-lifecycle, host-control, coding-session, and README indexes. r[agent_harness_runtime.agent_control.runtime]
- [x] [serial] D3 Add a focused agent-control rail for adapter, runtime, standard-host, dependency-isolation, no-std, lifecycle, and race evidence. r[agent_harness_runtime.agent_control.verification]

## Phase 5: Validation and closeout

- [x] [serial] V1 Run focused tests, formatting, Clippy, no-std and Wasm checks, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.agent_control.verification]
- [x] [serial] V2 Review dry-run sync and archive plans, preserve the durable-parking dependency, and archive only after current evidence passes. r[agent_harness_runtime.agent_control.verification]
