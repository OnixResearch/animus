# Agent Harness Runtime

## ADDED Requirements

### Requirement: Steel host calls resume the active cell

r[agent_harness_runtime.steel_host_bridge.continuation] Animus MUST provide a host-injected bridge that can return bounded correlated host results to the active model-written Steel cell, and the bridge MUST preserve explicit call order and terminal state.

#### Scenario: A Steel cell uses two host results

r[agent_harness_runtime.steel_host_bridge.continuation.multiple]
- GIVEN an admitted Steel cell issues two ordered host calls within policy bounds
- WHEN the injected executor returns a valid correlated result for each call
- THEN the bridge MUST resume the same staged cell after each result
- AND the final Steel expression MUST observe both typed results in call order.

#### Scenario: A host result has stale correlation

r[agent_harness_runtime.steel_host_bridge.continuation.stale]
- GIVEN an active cell awaits one call identity and sequence
- WHEN the runtime receives a result for another call, cell, session, or sequence
- THEN the bridge MUST fail closed before result delivery
- AND the staged kernel state MUST NOT commit.

### Requirement: Runtime authority remains outside Steel

r[agent_harness_runtime.steel_host_bridge.authority] Every bridged host request MUST pass current runtime admission before the injected host executor runs, and Steel MUST NOT receive ambient filesystem, process, network, credential, clock, persistence, or child-lifecycle authority.

#### Scenario: Steel requests an unadmitted operation

r[agent_harness_runtime.steel_host_bridge.authority.denied]
- GIVEN model-written Steel requests an operation outside the current policy or authority seal
- WHEN bridge admission evaluates the request
- THEN admission MUST return a typed denial before executor dispatch
- AND the bridge MUST NOT use direct Steel effects or an ambient fallback.

### Requirement: Kernel state commits only after cell completion

r[agent_harness_runtime.steel_host_bridge.commit_boundary] Persistent kernel source and environment state MUST commit only after the bridged cell reaches a successful final Steel value, and all earlier bridge states MUST remain staged.

#### Scenario: A later Steel expression fails

r[agent_harness_runtime.steel_host_bridge.commit_boundary.failure]
- GIVEN one or more host calls returned successfully to a staged cell
- WHEN a later Steel expression fails before the final value
- THEN the kernel MUST discard the staged cell state
- AND a later cell MUST NOT observe definitions or assignments from the failed cell.

### Requirement: Completed external effects remain explicit

r[agent_harness_runtime.steel_host_bridge.partial_effects] If a host effect completes before a later bridge or Steel failure, Animus MUST retain that completed observation and MUST NOT claim that discarding kernel state rolled back the external effect.

#### Scenario: Mutation succeeds before a later error

r[agent_harness_runtime.steel_host_bridge.partial_effects.recorded]
- GIVEN an admitted mutation completes and its observation is recorded
- WHEN a later continuation fails
- THEN the terminal outcome MUST reference the completed effect identity and the later failure class
- AND no receipt may claim atomic external rollback or exactly-once execution.

### Requirement: Bridge execution has finite policy bounds

r[agent_harness_runtime.steel_host_bridge.bounds] Bridge policy MUST define finite positive limits for host calls per cell, individual and cumulative result size, nesting, evaluator work, and wall time, with cancellation checks before admission, execution, and resumption.

#### Scenario: A cell exceeds its host-call limit

r[agent_harness_runtime.steel_host_bridge.bounds.exceeded]
- GIVEN a cell has consumed its admitted host-call count
- WHEN it requests one more host call
- THEN the bridge MUST stop with a typed bound outcome before executor dispatch
- AND it MUST NOT commit staged kernel state.

### Requirement: Resumable bridge evidence covers positive and negative paths

r[agent_harness_runtime.steel_host_bridge.verification] The bridge MUST include production-entrypoint tests for multiple calls, typed result use, final commit, authority denial, stale correlation, cancellation, timeout, bounds, host failure, later Steel failure, partial-effect evidence, leak rejection, and dependency isolation.

#### Scenario: The bridge change is ready to archive

r[agent_harness_runtime.steel_host_bridge.verification.closeout]
- GIVEN maintainers intend to sync and archive the bridge change
- WHEN focused and lifecycle validation runs
- THEN Steel, runtime, RLM, receipt, composition, no-std, Wasm, Cairn, and relevant Nix checks MUST pass
- AND durable mid-cell restore, effect rollback, sandboxing, task correctness, and production readiness MUST remain non-claims.
