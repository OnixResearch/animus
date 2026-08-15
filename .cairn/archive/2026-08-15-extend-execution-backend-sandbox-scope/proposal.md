## Why

The execution-backend selection planner picks the weakest policy-admitted backend for host effects and requires a current host recheck. It does not yet name a sandbox per spawned component. A whole-host sandbox cannot match the actual needs of a UI process, a provider client, a tool executor, and a web-search extension at the same time.

An explicit per-component sandbox scope should make the planner name exactly one sandbox descriptor per spawned extension or component, reusing the current sandlock descriptors and the bubblewrap agent guardrail without a new mechanism. Live enforcement, host rechecks, and sandbox backends remain host-owned.

## What Changes

- Make the planner name exactly one sandbox scope per spawned extension or component. r[agent_harness_runtime.execution_backend_sandbox.scope]
- Reuse the current sandlock descriptors and bubblewrap guardrail; introduce no new sandbox mechanism. r[agent_harness_runtime.execution_backend_sandbox.reuse]
- Match the selected sandbox to the declared component need and current capability request, keeping the weakest-admitted-backend rule. r[agent_harness_runtime.execution_backend_sandbox.matched]
- Reject unknown components, missing descriptors, unreachable backends, and unsatisfiable capability requests before effects. r[agent_harness_runtime.execution_backend_sandbox.fail_closed]
- Emit metadata-only receipts. r[agent_harness_runtime.execution_backend_sandbox.receipts]
- Add positive and negative fixtures. r[agent_harness_runtime.execution_backend_sandbox.verification]

## Impact

- **Rust**: extends the execution-backend selection planner contract; sandlock and guardrail enforcement stay in their owning hosts.
- **Consumers**: each spawned component gets a scope matched to its actual need instead of a broad whole-host scope.
- **Claims**: planning only; live enforcement and host rechecks remain outside this change.
