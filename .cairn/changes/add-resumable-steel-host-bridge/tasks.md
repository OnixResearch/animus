# Tasks

## B. Baseline and contract

- [ ] [serial] B1: Record the reviewed Animus, Prime Agent, and Steel revisions plus the current marker-only behavior. r[agent_harness_runtime.steel_host_bridge.verification]
- [ ] [serial] B2: Define bridge request, result, correlation, state, terminal outcome, and safe receipt DTOs in the narrow owning contract. r[agent_harness_runtime.steel_host_bridge.continuation]
- [ ] [depends:B2] B3: Define named limits and cancellation checkpoints without magic numbers. r[agent_harness_runtime.steel_host_bridge.bounds]

## C. Functional core and shell

- [ ] [depends:B2] C1: Implement the pure bridge transition reducer and stale or mismatched result rejection. r[agent_harness_runtime.steel_host_bridge.continuation]
- [ ] [depends:C1] C2: Add the injected runtime driver that admits every request before executor dispatch. r[agent_harness_runtime.steel_host_bridge.authority]
- [ ] [depends:C2] C3: Resume the same staged Steel cell with each bounded typed host result. r[agent_harness_runtime.steel_host_bridge.continuation]
- [ ] [depends:C3] C4: Commit staged kernel state only after a successful final Steel result. r[agent_harness_runtime.steel_host_bridge.commit_boundary]
- [ ] [depends:C4] C5: Preserve completed host-call observations when later evaluation fails. r[agent_harness_runtime.steel_host_bridge.partial_effects]

## T. Tests and closeout

- [ ] [depends:C5] T1: Add positive tests for multiple calls, result use, final commit, and stable correlation. r[agent_harness_runtime.steel_host_bridge.verification]
- [ ] [depends:C5] T2: Add negative tests for denial, stale results, wrong correlation, cancellation, timeout, bound overflow, host failure, later Steel failure, and receipt leaks. r[agent_harness_runtime.steel_host_bridge.verification]
- [ ] [depends:T2] T3: Preserve marker-only compatibility and prove default, no-std, and Wasm authority isolation. r[agent_harness_runtime.steel_host_bridge.authority]
- [ ] [depends:T3] T4: Update docs and component metadata, add a focused rail, run relevant workspace checks, and close through Cairn gates. r[agent_harness_runtime.steel_host_bridge.verification]
