# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: One sandbox scope per spawned component

r[agent_harness_runtime.execution_backend_sandbox.scope] Animus MUST name exactly one sandbox descriptor per spawned extension or component when the execution-backend selector plans effects, and MUST NOT apply a single whole-host sandbox to every component.

#### Scenario: Planner names a per-component descriptor

- GIVEN a host spawns distinct UI, provider, tool-executor, and web-search components under one session
- WHEN the execution-backend selector plans effects
- THEN the plan names one admitted sandbox descriptor for each spawned component
- AND no component silently inherits a broad whole-host scope.

### Requirement: Reuse existing sandbox mechanisms

r[agent_harness_runtime.execution_backend_sandbox.reuse] The per-component sandbox scope MUST reuse the admitted sandlock descriptors and the bubblewrap agent guardrail contracts and MUST NOT introduce a new sandbox mechanism.

#### Scenario: Extensions reuse the reviewed descriptor catalog

- GIVEN a spawned component matches a sandlock descriptor or guardrail profile
- WHEN the planner selects its scope
- THEN the plan references the reviewed descriptor identity
- AND no duplicate enforcement mechanism is created.

### Requirement: Capability-matched scope selection

r[agent_harness_runtime.execution_backend_sandbox.matched] The selected scope MUST intersect the declared component sandbox need and the current capability request under the weakest-admitted-backend rule, and MUST keep the current host-recheck rule in force before effects.

#### Scenario: Satisfying scope is selected

- GIVEN a component declares a sandbox need and a capability request
- WHEN the planner selects a scope
- THEN the scope satisfies both the declared need and the capability request
- AND the weakest admitted backend that satisfies them is selected.

### Requirement: Sandbox-scope validation fails closed

r[agent_harness_runtime.execution_backend_sandbox.fail_closed] Sandbox-scope validation MUST reject unknown components, missing descriptors, unreachable backends, and unsatisfiable capability requests with bounded safe error classes before any spawn effect.

#### Scenario: Missing descriptor blocks the spawn

- GIVEN a spawned component has no admitted descriptor
- WHEN the planner validates its scope
- THEN validation rejects the spawn with a descriptor diagnostic
- AND no broad fallback scope is used silently.

### Requirement: Metadata-only sandbox-scope receipts

r[agent_harness_runtime.execution_backend_sandbox.receipts] Sandbox-scope receipts MUST include component identities, the selected backend, the sandbox descriptor identity, capability-gate facts, and rejection classes, and MUST NOT include raw environment values, credentials, or sandbox rule bodies.

#### Scenario: Receipt rejects raw environment value

- GIVEN a sandbox-scope receipt fixture includes a raw environment value or credential
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Sandbox-scope verification evidence

r[agent_harness_runtime.execution_backend_sandbox.verification] The adaptation MUST include focused positive and negative tests for per-component scoping, mechanism reuse, capability-matched selection, fail-closed validation, receipt leak rejection, and feature isolation.

#### Scenario: Closeout evidence covers sandbox-scope fixtures

- GIVEN maintainers intend to archive the sandbox-scope adaptation
- WHEN closeout validation runs
- THEN evidence names focused execution-backend, sandlock, guardrail, workspace, receipt, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
