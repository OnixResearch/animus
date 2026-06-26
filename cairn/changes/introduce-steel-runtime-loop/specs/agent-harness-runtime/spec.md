# agent-harness-runtime Change Spec: Steel-backed runtime loop

## Requirements

### Requirement: Runtime crate boundary

r[agent_harness_runtime.loop.contract] The workspace MUST provide an `animus-runtime` crate that owns reusable agent-loop orchestration without depending on concrete provider, tool, filesystem, process, network, UI, or Syndicate implementations.

#### Scenario: Runtime crate is embeddable

r[agent_harness_runtime.loop.contract.embeddable]
- GIVEN an embedder depends on `animus-runtime`, `animus-adapters`, and a chosen loop fabric
- WHEN they construct a runtime with model and tool adapters
- THEN they can execute the agent loop without pulling in concrete provider or tool crates
- AND the runtime public API exposes typed inputs, configuration, events, reports, and errors.

### Requirement: Pure runtime loop core

r[agent_harness_runtime.loop.core] Runtime decision handling MUST be decomposed into a pure deterministic core over explicit turn state, decisions, adapter outcomes, and configuration, plus a thin shell that calls fabrics and adapters.

#### Scenario: Runtime core is directly testable

r[agent_harness_runtime.loop.core.pure]
- GIVEN a modeled turn state, runtime configuration, and next decision
- WHEN the runtime core evaluates the transition
- THEN it returns typed next state, events, and completion status deterministically
- AND it does not read files, inspect environment, spawn processes, use clocks, log, mutate global state, or call adapters.

### Requirement: Steel default loop fabric

r[agent_harness_runtime.steel_fabric.default] The runtime SHOULD provide a default constructor or helper that uses `animus-steel` as the scriptable agent-loop fabric while preserving a swappable fabric boundary.

#### Scenario: Steel decision drives runtime turn

r[agent_harness_runtime.steel_fabric.default.turn]
- GIVEN a Steel program defines a valid `(animus-decide)` function
- WHEN the runtime evaluates a turn through the Steel fabric
- THEN the returned decision is validated by Rust boundary DTOs before dispatch
- AND invalid Steel load, evaluation, shape, or payload errors stop the turn with typed runtime failure.

### Requirement: Adapter decision dispatch

r[agent_harness_runtime.adapter_dispatch] Runtime dispatch MUST route validated model and tool decisions through injected `ModelAdapter` and `ToolAdapter` implementations and MUST NOT construct concrete provider or tool clients internally.

#### Scenario: Model decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.model]
- GIVEN the loop fabric returns a validated model request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected model adapter with the validated request
- AND records a typed model-requested and model-responded event or a typed adapter-error event.

#### Scenario: Tool decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.tool]
- GIVEN the loop fabric returns a validated tool request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected tool adapter with the validated request
- AND records a typed tool-requested and tool-responded event or a typed adapter-error event.

### Requirement: Runtime stop and max-turn behavior

r[agent_harness_runtime.loop.max_turns] Runtime configuration MUST include an explicit max-turn limit and MUST halt with typed status when the fabric returns stop, final response, adapter failure, invalid decision, or max-turn exhaustion.

#### Scenario: Max turns fail closed

r[agent_harness_runtime.loop.max_turns.exhausted]
- GIVEN runtime configuration allows a finite number of turns
- WHEN the loop reaches that limit without final response or stop
- THEN runtime halts with a max-turns-exceeded status
- AND no additional model or tool adapter calls occur.

### Requirement: Runtime positive tests

r[agent_harness_runtime.tests.positive] Runtime work MUST include positive tests for one-turn model calls, tool calls, final responses, explicit stop behavior, and multi-turn propagation of previous model/tool outputs into the next fabric turn.

#### Scenario: Multi-turn state is propagated

r[agent_harness_runtime.tests.positive.multi_turn]
- GIVEN a model adapter response or tool adapter output is produced in one turn
- WHEN the next turn is evaluated
- THEN the loop fabric receives the previous output through typed turn input
- AND the final report records the ordered event sequence.

### Requirement: Runtime negative tests

r[agent_harness_runtime.tests.negative] Runtime work MUST include negative tests for invalid Steel decisions, malformed inputs, model adapter failure, tool adapter failure, max-turn exhaustion, missing final response, and stop-without-response behavior.

#### Scenario: Adapter failure stops safely

r[agent_harness_runtime.tests.negative.adapter_failure]
- GIVEN an injected model or tool adapter returns an error
- WHEN runtime dispatch handles that adapter call
- THEN runtime records a typed adapter-error event
- AND halts without invoking later adapters or fabric turns.

### Requirement: Preserves runtime schema

r[agent_harness_runtime.preserves_schema.contract] Runtime boundary messages SHOULD have Preserves schema coverage for stable event, request, response, final-report, and error payloads before those payloads are treated as external wire contracts.

#### Scenario: Schema validates runtime payloads

r[agent_harness_runtime.preserves_schema.contract.round_trip]
- GIVEN a runtime event or final report is encoded for a boundary
- WHEN Preserves schema validation and round-trip tests run
- THEN valid payloads decode into equivalent typed Rust values
- AND malformed, wrong-kind, missing-field, and invalid-text payloads fail closed.

### Requirement: Runtime docs and validation

r[agent_harness_runtime.docs.validation] Runtime work MUST update architecture docs and safety contracts to describe `animus-runtime`, Steel loop-fabric use, adapter dispatch boundaries, and validation commands.

#### Scenario: Runtime closeout is reviewable

r[agent_harness_runtime.validation]
- GIVEN runtime work is ready to archive
- WHEN reviewers inspect the change evidence
- THEN successful logs show focused crate tests, positive and negative runtime tests, Preserves schema tests when added, workspace check/clippy/tests, pre-commit, Octet/Tigerstyle, flake checks, Cairn proposal/design/tasks gates, and Cairn validation.
