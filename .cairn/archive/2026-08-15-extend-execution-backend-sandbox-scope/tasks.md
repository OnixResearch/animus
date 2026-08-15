## Phase 1: Sandbox-scope planning contract

- [x] [serial] Extend the execution-backend planner to name exactly one sandbox descriptor per spawned extension or component. r[agent_harness_runtime.execution_backend_sandbox.scope]
- [x] [serial] Reuse sandlock descriptors and the bubblewrap guardrail contracts without adding a new mechanism. r[agent_harness_runtime.execution_backend_sandbox.reuse]
- [x] [serial] Intersect the selected scope with the declared component need and the current capability request under the weakest-admitted rule. r[agent_harness_runtime.execution_backend_sandbox.matched]

## Phase 2: Fail-closed validation and receipts

- [x] [serial] Add fail-closed rejection for unknown components, missing descriptors, unreachable backends, and unsatisfiable capability requests before effects. r[agent_harness_runtime.execution_backend_sandbox.fail_closed]
- [x] [depends:scope] Add metadata-only receipts with selected backend, sandbox identity, and capability-gate facts. r[agent_harness_runtime.execution_backend_sandbox.receipts]

## Phase 3: Evidence

- [x] [parallel] Add positive fixtures for matched scopes, minimal defaults, and capability-satisfying intersections. r[agent_harness_runtime.execution_backend_sandbox.verification]
- [x] [parallel] Add negative fixtures for unknown components, missing descriptors, unreachable backends, unsatisfiable capability requests, and receipt leak rejection. r[agent_harness_runtime.execution_backend_sandbox.fail_closed] r[agent_harness_runtime.execution_backend_sandbox.verification]
- [x] [serial] Verify default builds remain independent of the planner extension. r[agent_harness_runtime.execution_backend_sandbox.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document per-component scoping, mechanism reuse, capability intersection, and non-claims. r[agent_harness_runtime.execution_backend_sandbox.scope] r[agent_harness_runtime.execution_backend_sandbox.matched]
- [x] [serial] Run focused execution-backend, sandlock, guardrail, workspace, formatting, clippy, Cairn validation, and Cairn gate rails, then sync and archive only after all deterministic gates pass. r[agent_harness_runtime.execution_backend_sandbox.verification]
