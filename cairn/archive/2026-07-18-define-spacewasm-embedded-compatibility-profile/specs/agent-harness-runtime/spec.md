# Agent Harness Runtime SpaceWasm Compatibility Delta

## Purpose

Define an opt-in diagnostic profile that compiles and executes a bounded test guest over selected Animus no-std behavior under one exact SpaceWasm cohort without broadening Animus's generic Wasm compile, runtime, plugin, or authority claims.

## Requirements

### Requirement: SpaceWasm compatibility uses a typed non-default profile

r[agent_harness_runtime.spacewasm_compat.profile] Animus MUST define a typed Nickel `spacewasm-embedded-compat` profile binding the exact crate/feature/export surface, producer target/toolchain, guest wrapper, runtime/materialization cohort, Octet report, limits, expected outcomes, and non-claims, and MUST consume a fresh deterministic export.

#### Scenario: Complete profile is selected

- GIVEN the compatibility profile is explicitly enabled and every identity matches
- WHEN compatibility planning runs
- THEN Animus may plan the declared diagnostic lane

#### Scenario: Profile is absent, stale, or disabled

- GIVEN the profile is not selected or its Nickel/export facts differ
- WHEN compatibility planning runs
- THEN Animus rejects the lane without changing generic Wasm compile support

### Requirement: The supported fixture surface is exact and bounded

r[agent_harness_runtime.spacewasm_compat.surface] The profile MUST name every selected crate, feature, exported fixture operation, imported host function, ABI field, input/output limit, and excluded shell capability and MUST reject implicit surface widening.

#### Scenario: Declared pure operation is included

- GIVEN an operation belongs to the named no-std core/adapter fixture surface
- WHEN surface admission runs
- THEN the guest may export that operation under the declared ABI

#### Scenario: Shell or undeclared operation is introduced

- GIVEN the fixture pulls provider, tool execution, persistence, process, filesystem, network, credential, clock, terminal, Steel, browser, WASI, plugin, or other undeclared behavior
- WHEN surface admission runs
- THEN compatibility validation fails before guest execution

### Requirement: Default and reusable dependencies remain runtime-free

r[agent_harness_runtime.spacewasm_compat.isolation] Animus MUST keep SpaceWasm, Wasmtime, WASI, runtime loaders, fixture builders, and std diagnostic shells outside default and reusable no-std dependency trees.

#### Scenario: Default and no-default trees are inspected

- GIVEN ordinary Animus feature profiles
- WHEN dependency isolation checks run
- THEN no SpaceWasm compatibility runtime or shell package appears

#### Scenario: Runtime dependency leaks into a reusable crate

- GIVEN a reusable core/adapter/router crate starts depending on a runtime, loader, or std compatibility shell
- WHEN isolation validation runs
- THEN the check fails and identifies the package edge

### Requirement: The compatibility guest uses a conservative producer profile

r[agent_harness_runtime.spacewasm_compat.guest] Animus MUST build a test-only guest wrapper with the pinned conservative WebAssembly 1.0 producer target/profile and MUST NOT lower arbitrary modern modules or present the wrapper as a public plugin SDK.

#### Scenario: Conservative target is supported

- GIVEN the pinned toolchain and selected crates support the declared target and `core`/`alloc` surface
- WHEN guest compilation runs
- THEN the exact artifact and producer configuration are recorded

#### Scenario: Target or dependency is unsupported

- GIVEN the pinned toolchain or dependency closure cannot emit the declared profile
- WHEN compilation runs
- THEN the compatibility lane is blocked rather than switching targets or lowering opaquely

### Requirement: Static artifact evidence precedes execution

r[agent_harness_runtime.spacewasm_compat.artifact] Animus MUST remeasure exact guest bytes and verify matching Octet artifact/profile/cohort evidence before invoking SpaceWasm.

#### Scenario: Artifact report matches

- GIVEN guest bytes, producer facts, Octet report, and SpaceWasm profile identities agree
- WHEN artifact admission runs
- THEN the injected shell may receive an execution request

#### Scenario: Artifact or report is stale

- GIVEN bytes, digest, feature facts, profile, or cohort differ
- WHEN admission runs
- THEN execution is denied without trusting build success or a runtime fallback

### Requirement: Runtime execution is injected and diagnostic-only

r[agent_harness_runtime.spacewasm_compat.execution] Animus MUST invoke SpaceWasm only through a non-default injected diagnostic service that receives explicit artifact/input/profile facts, exposes no ambient host authority, and returns bounded normalized observations.

#### Scenario: Injected service is available and admitted

- GIVEN exact bundle and artifact evidence plus bounded canonical input
- WHEN the service executes the guest
- THEN it returns explicit result/trap/resource/output facts for pure classification

#### Scenario: Service is missing or fails

- GIVEN no injected service exists or runtime setup/execution fails
- WHEN compatibility execution is requested
- THEN Animus fails closed without ambient discovery, plugin activation, or native fallback

### Requirement: Compatibility decisions have a functional core

r[agent_harness_runtime.spacewasm_compat.functional_core] Profile/surface/dependency/evidence/resource/outcome admission, identity matching, receipt projection, and diagnostic ordering MUST be pure deterministic logic over owned explicit inputs.

#### Scenario: Identical facts are classified repeatedly

- GIVEN identical normalized profile, artifact, evidence, and execution facts
- WHEN the core evaluates them
- THEN it returns identical decisions, identities, receipts, and diagnostics without filesystem, process, network, clock, environment, runtime, or output effects

### Requirement: Compatibility receipts are bounded and secret-free

r[agent_harness_runtime.spacewasm_compat.evidence] Compatibility receipts MUST bind Animus source/crate/feature profile, producer/toolchain, exact guest artifact, SpaceWasm bundle/cohort/configuration, Octet report, input/output identities, result/trap/resource class, and non-claims and MUST exclude secrets, raw environment, unrestricted paths/logs, and provider payloads.

#### Scenario: Compatibility run completes

- GIVEN admitted execution returns normalized facts
- WHEN receipt projection runs
- THEN the receipt contains only declared safe fields and exact parent identities

#### Scenario: Receipt input contains sensitive or unbounded data

- GIVEN raw credentials, environment, unrestricted logs, or undeclared payloads are supplied
- WHEN receipt validation runs
- THEN the receipt is rejected or safely redacted according to existing policy

### Requirement: Compatibility documentation preserves claim boundaries

r[agent_harness_runtime.spacewasm_compat.docs] Animus MUST document SpaceWasm compatibility separately from generic Wasm compilation and MUST state excluded runtime, plugin, WASI, authority, sandbox, production, and release claims in the README, safety contract, component catalog, and focused-rail documentation.

#### Scenario: Operator reads support documentation

- GIVEN both generic compile and SpaceWasm compatibility rails exist
- WHEN support status is rendered
- THEN the exact crate/profile/runtime evidence and exclusions are distinguishable

### Requirement: Compatibility has positive and negative verification

r[agent_harness_runtime.spacewasm_compat.verification] The profile MUST include positive profile, conservative-target, artifact/evidence, pure-operation, deterministic-output, expected-trap, and bounded-resource cases plus negative target, dependency, feature, import, evidence, malformed-data, resource, service, runtime, dependency-leak, plugin-activation, receipt-leak, and overclaim cases.

#### Scenario: Compatibility boundary changes

- GIVEN profile, guest, producer, artifact admission, injected execution, receipt, or dependency behavior changes
- WHEN closeout evidence is assembled
- THEN generic and SpaceWasm-specific checks, positive and negative fixtures, config freshness, Cairn validation, and lifecycle gates run or record an exact blocker and next-best check

#### Scenario: Passing fixture is promoted to general runtime support

- GIVEN one exact compatibility fixture passes
- WHEN a report claims arbitrary Animus Wasm runtime, plugin loading, sandbox completeness, embedded readiness, or release eligibility
- THEN evidence validation rejects the unsupported promotion
