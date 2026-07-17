# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Sandlock host enforcement is optional and isolated

r[agent_harness_runtime.sandlock_host_adapter.feature_gate] Animus MUST expose the Sandlock Linux host adapter only through a non-default `animus-std` feature bound to a reviewed upstream compatibility cohort, and default, reusable-core, and non-Linux builds MUST remain independent of Sandlock.

#### Scenario: Default build does not select Sandlock

- GIVEN Animus is built with its default feature set or for a non-Linux target
- WHEN dependency and public-surface checks run
- THEN Sandlock MUST NOT be selected as a required dependency or available host adapter
- AND portable sandbox descriptor and receipt types MUST remain usable without Linux or Sandlock types.

### Requirement: Sandlock launch translation has a pure core

r[agent_harness_runtime.sandlock_host_adapter.translation] Animus MUST translate a normalized sandbox descriptor, safe capability-root bindings, policy identity, authorization identity, and declared host capability facts into a deterministic owned launch plan without filesystem, environment, process, clock, network, or kernel effects.

#### Scenario: Identical admitted inputs produce one launch plan

- GIVEN identical normalized intent, authorization, policy, root bindings, command identity, and capability facts
- WHEN Sandlock launch translation runs
- THEN it MUST return the same ordered constraints and domain-separated BLAKE3 plan identity
- AND the plan MUST NOT contain raw prompts, credentials, ambient roots, or model-visible absolute host paths.

#### Scenario: Root binding escapes its grant

- GIVEN a writable or readable root binding is missing, absolute where a confined relative root is required, or escapes its granted capability root
- WHEN launch translation validates the binding
- THEN it MUST reject the plan before a Sandlock builder or child process is created.

### Requirement: Requested Sandlock mechanisms are probed fail closed

r[agent_harness_runtime.sandlock_host_adapter.preflight] The Linux host adapter MUST prove every requested filesystem, syscall, supervisor, COW, resource, network, HTTP, environment, and determinism mechanism is usable under the selected Sandlock cohort before process launch, and it MUST reject unsupported or partial enforcement rather than waive a constraint.

#### Scenario: Every requested mechanism is available

- GIVEN an admitted launch plan requests a supported subset and functional probes confirm every required mechanism
- WHEN host preflight completes
- THEN the adapter MAY proceed to process launch
- AND the preflight result MUST bind the requested and proven capability sets to the cohort identity.

#### Scenario: One requested mechanism is unavailable

- GIVEN filesystem confinement is available but a requested seccomp-notification, COW, network, HTTP, resource, environment, or determinism mechanism is unavailable or unproven
- WHEN host preflight runs
- THEN dispatch MUST stop before child creation with a typed missing-capability result
- AND the adapter MUST NOT silently remove the constraint, use an upstream waiver, or report partial enforcement as success.

### Requirement: Sandbox enforcement follows authorization

r[agent_harness_runtime.sandlock_host_adapter.authority] The Sandlock host adapter MUST accept execution only when a prior Animus/Basalt authorization decision matches the action, subject, target, descriptor, and policy identities, and sandbox evidence MUST NOT create, widen, or replace authorization.

#### Scenario: Valid authorization is bound to one launch

- GIVEN a prior allow decision matches every launch identity and remains valid for the requested action
- WHEN host admission runs
- THEN the launch plan MAY proceed to Sandlock preflight
- AND the receipt MUST keep authorization identity separate from enforcement outcome.

#### Scenario: Authorization is absent or stale

- GIVEN authorization is denied, missing, expired, or bound to a different action, subject, target, descriptor, or policy
- WHEN the Sandlock adapter is invoked
- THEN it MUST reject execution before capability probes with side effects or process launch
- AND a successful sandbox probe MUST NOT override the denial.

### Requirement: Sandlock execution remains a thin bounded shell

r[agent_harness_runtime.sandlock_host_adapter.execution] The Linux shell MUST realize only the admitted launch plan through the pinned `sandlock-core` API, bound command input and output, propagate cancellation and timeout, classify COW and cleanup outcomes, and return typed execution facts to the pure receipt projector.

#### Scenario: Constrained process completes

- GIVEN authorization and preflight pass and the process exits under all requested controls
- WHEN the host shell returns execution facts
- THEN the result MUST distinguish exit, timeout, cancellation, COW disposition, and cleanup status
- AND no unrequested host capability may be added by the adapter.

#### Scenario: Setup fails before exec

- GIVEN Sandlock setup, supervisor initialization, root resolution, or COW staging fails after admission
- WHEN the shell handles the failure
- THEN no unconstrained fallback process may run
- AND the result MUST remain a typed setup or cleanup failure.

### Requirement: Sandlock host receipts are redacted and scoped

r[agent_harness_runtime.sandlock_host_adapter.receipts] Sandlock host receipts MUST contain only safe status and issue classes, descriptor, policy, authorization, plan, and cohort identities, requested and enforced capability labels, bounded COW counts or digests, bounded output digests, exit classification, cleanup status, and a domain-separated BLAKE3 receipt identity.

#### Scenario: Successful receipt is projected

- GIVEN a constrained process completes and cleanup facts are available
- WHEN receipt projection runs
- THEN it MUST bind the exact admitted plan and enforced capability cohort
- AND it MUST state that confinement evidence is not authorization, kernel proof, build hermeticity, or cross-platform parity.

#### Scenario: Unsafe evidence is supplied

- GIVEN receipt input contains a raw prompt, credential, absolute path dump, file content, network payload, stdout or stderr body, Sandlock debug log, or unbounded event list
- WHEN receipt validation runs
- THEN it MUST fail closed without emitting an accepted durable receipt.

### Requirement: Sandlock host behavior has positive and negative evidence

r[agent_harness_runtime.sandlock_host_adapter.verification] The Sandlock host adapter MUST include pure positive and negative tests plus an opt-in real-Linux behavior rail for feature isolation, translation, authorization binding, capability preflight, constrained execution, denial behavior, COW disposition, cleanup, and receipt safety.

#### Scenario: Host-adapter closeout is reviewed

- GIVEN maintainers intend to archive the Sandlock host-adapter change
- WHEN closeout evidence is assembled
- THEN it MUST include passing positive and negative pure tests, default and feature-gated build checks, focused adapter checks, and Cairn gates
- AND unavailable real-kernel checks MUST be recorded as blocked with prerequisites and a next-best validation rather than reported as passed.
