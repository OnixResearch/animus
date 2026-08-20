# Animus Standard Pueue Host

## ADDED Requirements

### Requirement: Separate pueue host component

r[agent_harness_runtime.pueue_host.component] Animus MUST provide pueue through a separate non-default `animus-std-pueue` host crate. Reusable default crates MUST gain no pueue process authority.

#### Scenario: Consumer enables the component

r[agent_harness_runtime.pueue_host.component.enabled]
- GIVEN a consumer explicitly selects the pueue host feature and crate
- WHEN the consumer builds its host shell
- THEN the public pueue host contracts MUST be available

#### Scenario: Default workspace surface builds

r[agent_harness_runtime.pueue_host.component.default]
- GIVEN a consumer does not select the pueue host
- WHEN default reusable crates build
- THEN they MUST contain no pueue process or daemon dependency

### Requirement: Typed pueue configuration

r[agent_harness_runtime.pueue_host.configuration] Typed Nickel configuration MUST declare namespace, executable identities, backend capabilities, bounds, and reviewed process-job profiles. Rust MUST revalidate the generated projection.

#### Scenario: Valid configuration loads

r[agent_harness_runtime.pueue_host.configuration.valid]
- GIVEN a generated projection satisfies the pueue contract
- WHEN the host loads it
- THEN the host MUST receive all declared authority and bounds

#### Scenario: Configuration drifts or omits a bound

r[agent_harness_runtime.pueue_host.configuration.invalid]
- GIVEN the projection drifts or omits a required authority or bound
- WHEN the configuration gate runs
- THEN the gate MUST fail before pueue execution

### Requirement: Exact queue admission

r[agent_harness_runtime.pueue_host.admission] Enqueue MUST resolve an exact reviewed process-job profile. Mutating controls MUST declare explicit task, group, default-group, or all-groups targets.

#### Scenario: Reviewed enqueue request is admitted

r[agent_harness_runtime.pueue_host.admission.enqueue]
- GIVEN an enqueue request names an available reviewed profile
- WHEN admission binds the request
- THEN the plan MUST contain the exact program, arguments, root, environment, timeout, and output policy

#### Scenario: Raw command or implicit target is supplied

r[agent_harness_runtime.pueue_host.admission.denied]
- GIVEN a request supplies raw shell text or omits a mutating control target
- WHEN admission runs
- THEN admission MUST fail before any process call

### Requirement: Bounded pueue execution

r[agent_harness_runtime.pueue_host.execution] The host MUST invoke pueue without a shell through an injected bounded-process service. It MUST enforce cancellation, deadlines, output bounds, and process-tree teardown.

#### Scenario: Pueue operation completes

r[agent_harness_runtime.pueue_host.execution.valid]
- GIVEN an admitted operation and current host authority
- WHEN the bounded-process service invokes pueue
- THEN the host MUST return the bounded backend observation

#### Scenario: Operation times out or is cancelled

r[agent_harness_runtime.pueue_host.execution.interrupted]
- GIVEN an active pueue operation exceeds its deadline or receives cancellation
- WHEN the process service observes the interruption
- THEN it MUST stop the admitted process tree
- AND the host MUST return the typed interruption class

### Requirement: Bounded queue projection

r[agent_harness_runtime.pueue_host.projection] The host MUST parse only reviewed bounded pueue output cohorts and MUST project them into existing Animus queue DTOs.

#### Scenario: Supported status output parses

r[agent_harness_runtime.pueue_host.projection.valid]
- GIVEN pueue returns supported bounded status output
- WHEN the host parses it
- THEN the host MUST return typed task and group observations

#### Scenario: Backend output is malformed or oversized

r[agent_harness_runtime.pueue_host.projection.invalid]
- GIVEN pueue returns malformed, unsupported, or oversized output
- WHEN the host parses it
- THEN parsing MUST fail without exposing raw output

### Requirement: Capability-scoped isolation

r[agent_harness_runtime.pueue_host.isolation] The host MUST use one caller-supplied capability-scoped pueue namespace. It MUST NOT discover home paths, current directories, ambient configuration, or unrelated daemons.

#### Scenario: Product namespace is configured

r[agent_harness_runtime.pueue_host.isolation.valid]
- GIVEN the caller supplies an admitted namespace and configuration leaf
- WHEN the host contacts pueue
- THEN every operation MUST use that namespace

#### Scenario: Ambient daemon contains tasks

r[agent_harness_runtime.pueue_host.isolation.ambient]
- GIVEN an unrelated ambient pueue daemon contains tasks
- WHEN the isolated host reads status
- THEN no ambient task or group MUST appear

### Requirement: Animus tool integration

r[agent_harness_runtime.pueue_host.tool] The component MUST supply a pueue tool definition and executor adapter for Animus tool admission and staged execution.

#### Scenario: Admitted tool operation completes

r[agent_harness_runtime.pueue_host.tool.valid]
- GIVEN an exact queue tool request passes current authority and capability gates
- WHEN staged execution invokes the adapter
- THEN the result MUST pass bounded validation before durable commit

#### Scenario: Tool request is denied

r[agent_harness_runtime.pueue_host.tool.denied]
- GIVEN the request has invalid arguments, denied capability, stale authority, or unsafe output policy
- WHEN final admission runs
- THEN the adapter MUST NOT call pueue

### Requirement: Published component contract

r[agent_harness_runtime.pueue_host.publication] Animus MUST publish the pueue component, feature, configuration contract, component catalog entry, focused rail, and claim boundaries from one immutable revision.

#### Scenario: Consumer records the component

r[agent_harness_runtime.pueue_host.publication.consumer]
- GIVEN focused evidence passes for the published revision
- WHEN a consumer records the dependency
- THEN the consumer MUST be able to use public contracts without a sibling worktree

### Requirement: Pueue verification matrix

r[agent_harness_runtime.pueue_host.verification] Positive and negative tests MUST cover configuration, admission, operations, parsing, isolation, tool integration, cancellation, bounds, and receipt redaction.

#### Scenario: Focused pueue rail passes

r[agent_harness_runtime.pueue_host.verification.matrix]
- GIVEN the component, configuration, adapters, docs, and fixtures are complete
- WHEN the focused and repository gates run
- THEN every admitted operation MUST pass
- AND every raw, implicit, ambient, malformed, denied, cancelled, timed-out, or leaking case MUST fail as declared
