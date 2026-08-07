# Execution Backend Selection Planner Delta

## Requirements

### Requirement: Backend selection is a pure planning boundary

r[agent_harness_runtime.execution_backend_planner.boundary] Animus MUST define execution backend selection as a pure deterministic plan over explicit workload requirements, reviewed policy, and supplied backend facts. Reusable crates MUST NOT discover providers, spawn processes, open networks, read host state, or execute selected work.

#### Scenario: Explicit inputs produce a plan

- GIVEN a workload declares complete requirements and the caller supplies admitted backend candidates
- WHEN the planner evaluates the request
- THEN it returns one selected plan or a bounded denial without host effects

### Requirement: Workload and backend alternatives are closed and typed

r[agent_harness_runtime.execution_backend_planner.model] Animus MUST define closed alternatives for embedded component, guarded local process, isolated host, and external machine backend classes. Requirements MUST cover execution format, filesystem semantics, network posture, process model, terminal needs, browser or device needs, resource limits, persistence, and requested isolation claims.

#### Scenario: A known requirement maps to candidates

- GIVEN a workload requires a supported execution format and effect set
- WHEN candidate compatibility is evaluated
- THEN only backend classes that satisfy every declared requirement remain eligible

#### Scenario: An unknown requirement fails closed

- GIVEN a workload contains an unsupported requirement or backend class
- WHEN compatibility is evaluated
- THEN Animus denies selection instead of ignoring the unknown value

### Requirement: Policy selects the weakest admitted backend

r[agent_harness_runtime.execution_backend_planner.policy] A versioned Nickel policy MUST define backend admission, compatibility, and explicit preference order. Code MUST NOT embed an unexplained numeric backend ranking or infer a broader backend from command names, agent labels, or availability failures.

#### Scenario: The least-authoritative candidate is selected

- GIVEN more than one backend satisfies all requirements
- WHEN the policy orders those candidates
- THEN the planner selects the first fully admitted candidate and records why broader candidates were unnecessary

### Requirement: Selection denies silent fallback

r[agent_harness_runtime.execution_backend_planner.selection] The planner MUST deny ambiguous selection, missing requirements, unsupported combinations, and implicit downgrade or escalation. If a selected backend becomes unavailable, the result MUST require an explicit new plan.

#### Scenario: Selected backend becomes unavailable

- GIVEN a valid plan names one backend identity
- WHEN that backend is unavailable at execution time
- THEN the host denies execution and requests explicit reselection rather than using another backend

### Requirement: Current enforcement is rechecked before effects

r[agent_harness_runtime.execution_backend_planner.enforcement] The host MUST recheck backend identity, adapter readiness, application-host binding, runtime-authority unit, effect permissions, resources, and sandbox or isolation enforcement before execution. A plan receipt MUST NOT substitute for current observations.

#### Scenario: Authority drift blocks execution

- GIVEN a plan was valid under an earlier authority profile
- WHEN current enforcement differs
- THEN execution is denied before effects and the drift is recorded safely

### Requirement: Plan evidence is metadata-only

r[agent_harness_runtime.execution_backend_planner.evidence] Plan and enforcement receipts MUST bind BLAKE3 identities, requirement classes, selected backend class, policy, observations, denials, and non-claims. They MUST exclude credentials, commands, environment values, host paths, provider payloads, and raw sandbox diagnostics.

#### Scenario: Safe plan receipt passes

- GIVEN a plan contains only admitted metadata
- WHEN receipt validation runs
- THEN Animus emits a stable metadata-only receipt

### Requirement: Selection behavior has positive and negative fixtures

r[agent_harness_runtime.execution_backend_planner.verification] The focused rail MUST test each admitted backend class and negative unknown, ambiguity, availability, authority, resource, fallback, and overclaim cases.

#### Scenario: Focused selection rail passes

- GIVEN the required fixtures and frozen consumer mappings are current
- WHEN the focused rail runs
- THEN it reports the supported planner boundary and exact non-claims
