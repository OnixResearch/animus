# Agent Harness Runtime

## ADDED Requirements

### Requirement: Tool execution uses one fixed stage plan

r[agent_harness_runtime.tool_execution_stages.plan] Animus MUST represent pre-admission, final admission, pre-execution, execution, output narrowing, final validation, durable commit, and post-commit observation as one fixed ordered stage plan with exact handler and cohort identities.

#### Scenario: Zero optional handlers are selected

- GIVEN a valid tool request uses no optional pre-admission, pre-execution, output, or observation handlers
- WHEN stage planning runs
- THEN the fixed final-admission, execution, final-validation, and durable-commit barriers MUST remain present
- AND the existing tool executor kit MUST remain authoritative.

#### Scenario: Handler phase is reordered

- GIVEN a stage plan places execution before final admission or observation before durable commit
- WHEN validation runs
- THEN Animus MUST reject the plan before tool effects.

### Requirement: Every stage binds the exact request and authority facts

r[agent_harness_runtime.tool_execution_stages.binding] Each stage input and candidate outcome MUST bind the exact tool request, schema, correlation, executor, runtime authority seal, mutation posture, output policy, cancellation, deadline, and immutable stage-plan identity.

#### Scenario: Handler substitutes tool arguments

- GIVEN a handler candidate cites different argument values from the admitted request
- WHEN stage candidate validation runs
- THEN Animus MUST reject the candidate
- AND it MUST NOT execute the substituted request.

### Requirement: Tool-stage denial and narrowing are monotonic

r[agent_harness_runtime.tool_execution_stages.monotonic] No later stage may reverse a denial, add capability or authority, widen mutation, increase a bound, change fixed request identity, remove a required redaction or commit barrier, or convert cancellation, timeout, failure, or truncation into success.

#### Scenario: Later handler tries to reverse denial

- GIVEN an earlier stage denied the request
- WHEN a later candidate reports allow or completed
- THEN validation MUST preserve the denial
- AND no executor or provider effect may occur.

#### Scenario: Pre-execution wrapper widens authority

- GIVEN a handler requests a wrapper outside the admitted effect scope
- WHEN current wrapper validation runs
- THEN the wrapper MUST be denied
- AND the original admission MUST NOT authorize it.

### Requirement: Output handlers can only narrow model-visible output

r[agent_harness_runtime.tool_execution_stages.output] Output handlers MAY redact, truncate, classify, or replace unsafe visible text with a bounded safe marker, but they MUST preserve the execution outcome class, remain within the admitted output policy, and bind input and narrowed-output identities.

#### Scenario: Sensitive output is redacted

- GIVEN a completed executor result contains content that the admitted output policy requires to hide
- WHEN an exact output-narrowing handler runs and Rust revalidation succeeds
- THEN the final visible text MAY contain only the bounded safe replacement
- AND the raw output MUST remain absent from public evidence.

#### Scenario: Handler changes failure to success

- GIVEN executor output has a failure, timeout, cancellation, or truncation class
- WHEN an output handler returns a completed-success class
- THEN final validation MUST reject the candidate
- AND no completed-effect receipt may be emitted.

### Requirement: Durable result commit precedes observation and provider continuation

r[agent_harness_runtime.tool_execution_stages.commit] The coding-session owner MUST commit the exact final validated tool result before post-commit observation or dependent provider continuation.

#### Scenario: Tool result commits durably

- GIVEN final output validation succeeds and the coding session commits the exact correlated result
- WHEN a matching durable observation is returned
- THEN metadata-only post-commit observation MAY run
- AND provider continuation MAY use only the committed result.

#### Scenario: Commit fails

- GIVEN final validation succeeds but durable result commit is missing, stale, partial, or mismatched
- WHEN the stage shell evaluates continuation
- THEN post-commit observation and provider continuation MUST remain blocked
- AND the effect outcome MUST remain explicit rather than becoming a committed tool result.

### Requirement: Tool stages have adversarial verification

r[agent_harness_runtime.tool_execution_stages.verification] The focused rail MUST cover fixed ordering, exact binding, sticky denial, monotonic narrowing, executor invocation, output safety, durable commit, post-commit observation, provider continuation, missing handlers, substitution, widening, and evidence leaks.

#### Scenario: Tool-stage rail passes

- GIVEN positive and negative core, shell, executor-kit, and coding-session fixtures are present
- WHEN the focused rail runs
- THEN admitted stage sequences MUST preserve existing authority and ordering
- AND invalid candidates MUST fail before prohibited effects, continuation, or claims.
