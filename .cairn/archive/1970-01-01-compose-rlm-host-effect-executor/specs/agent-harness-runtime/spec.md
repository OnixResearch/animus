# Agent Harness Runtime

## ADDED Requirements

### Requirement: The RLM executor is an explicit non-default host

r[agent_harness_runtime.rlm_host_executor.boundary] Animus MUST provide the complete RLM host executor only as a non-default authority-scoped component with explicit route bindings, capability roots, command profiles, services, limits, and current authority facts.

#### Scenario: The executor is constructed

r[agent_harness_runtime.rlm_host_executor.boundary.explicit]
- GIVEN a host supplies exact bindings for the selected RLM operation classes
- WHEN executor construction validates the bindings
- THEN it MUST return one explicit executor with stable binding identities
- AND reusable and default crates MUST gain no filesystem, process, network, credential, persistence, or child authority.

### Requirement: Every operation uses its owning capability pack

r[agent_harness_runtime.rlm_host_executor.dispatch] The executor MUST route each admitted operation to its exact bound capability pack and MUST recheck current seal, generation, cancellation, scope, and binding identity before the effect.

#### Scenario: A required capability pack is absent

r[agent_harness_runtime.rlm_host_executor.dispatch.missing]
- GIVEN an admitted request names an operation with no bound pack
- WHEN executor dispatch runs
- THEN dispatch MUST return a typed unavailable outcome before the effect
- AND it MUST NOT use standard-library, environment, current-directory, shell, or global-registry fallback behavior.

### Requirement: Workspace operations remain capability-relative

r[agent_harness_runtime.rlm_host_executor.workspace] File reads MUST use an explicit capability root with lexical admission and no-follow observation, and file edits MUST use the existing revision-gated workspace edit host.

#### Scenario: A file changes before edit publication

r[agent_harness_runtime.rlm_host_executor.workspace.conflict]
- GIVEN an admitted edit binds an observed revision and target
- WHEN the current target no longer matches that revision
- THEN the workspace route MUST return a typed conflict before publication
- AND it MUST NOT overwrite, retry against a new revision, or widen the target scope.

### Requirement: Shell requests use exact bounded command profiles

r[agent_harness_runtime.rlm_host_executor.process] Shell-class requests MUST resolve to exact admitted command profiles with explicit argv, environment, root, input, output, deadline, cancellation, and teardown limits before bounded process execution.

#### Scenario: A command does not match a profile

r[agent_harness_runtime.rlm_host_executor.process.unknown]
- GIVEN a shell request cannot resolve to an exact admitted command profile
- WHEN process-route admission runs
- THEN it MUST deny the request before process creation
- AND it MUST NOT pass the request to an ambient command shell.

### Requirement: Tool, context, child, and clear routes retain their owners

r[agent_harness_runtime.rlm_host_executor.services] Named tools, context reads, child requests, and kernel clears MUST delegate to their existing application owners through explicit bindings without service discovery or authority duplication.

#### Scenario: A context result exceeds its model-visible limit

r[agent_harness_runtime.rlm_host_executor.services.context_bound]
- GIVEN an explicit context resolver returns more content than the admitted result limit
- WHEN the executor translates the result
- THEN it MUST return a typed bound outcome without exposing the excess content
- AND safe receipts MUST contain only bounded labels, counts, identities, and status classes.

### Requirement: Complete executor conformance covers every route

r[agent_harness_runtime.rlm_host_executor.verification] The executor MUST include positive and negative production-entrypoint tests for every operation class, current rechecks, service absence, capability denial, conflicts, process limits, cancellation, timeout, result bounds, failure translation, fallback rejection, receipt safety, feature isolation, and license isolation.

#### Scenario: The executor change is ready to archive

r[agent_harness_runtime.rlm_host_executor.verification.closeout]
- GIVEN maintainers intend to sync and archive the RLM executor
- WHEN focused and lifecycle validation runs
- THEN bridge, tool, workspace, process, context, child, receipt, dependency, license, Cairn, and relevant Nix checks MUST pass
- AND sandboxing, hermeticity, effect correctness, reversible mutation, task success, and production readiness MUST remain non-claims.
