# agent-harness-runtime Change Spec: Turn lifecycle hook boundaries

## Requirements

### Requirement: Bounded turn lifecycle hook scope

r[agent_harness_runtime.turn_lifecycle_hooks.scope] Animus MUST model turn lifecycle hooks as typed runtime boundaries and MUST map startup to the existing `session_start` lifecycle hook point rather than adding a duplicate startup label.

#### Scenario: Startup uses session start

r[agent_harness_runtime.turn_lifecycle_hooks.scope.startup]
- GIVEN a host wants to run startup hook logic
- WHEN Animus validates lifecycle hook descriptors
- THEN `session_start` is the supported startup hook point
- AND a duplicate `startup` hook point is rejected as unknown.

### Requirement: Pre-turn and post-turn hook points

r[agent_harness_runtime.turn_lifecycle_hooks.points] Animus MUST provide stable `pre_turn` and `post_turn` lifecycle hook point labels for turn-scoped host extension.

#### Scenario: Turn hook points parse deterministically

r[agent_harness_runtime.turn_lifecycle_hooks.points.parse]
- GIVEN hook descriptors for `pre_turn` and `post_turn`
- WHEN lifecycle hook event validation runs
- THEN both descriptors parse to stable event kinds
- AND their stable string forms round-trip unchanged.

#### Scenario: Unsupported turn capability fails validation

r[agent_harness_runtime.turn_lifecycle_hooks.points.negative]
- GIVEN a post-turn hook descriptor declares denial or request-revalidation authority
- WHEN lifecycle hook definition validation runs
- THEN validation fails with an unsupported-capability diagnostic
- AND the hook is not registered.

### Requirement: Pure turn lifecycle FSM

r[agent_harness_runtime.turn_lifecycle_hooks.fsm] Animus MUST model startup, pre-turn, post-turn, and completion progression in a pure deterministic runtime core that returns typed lifecycle hook effects and performs no process, filesystem, network, clock, provider, persistence, or hook-adapter execution.

#### Scenario: FSM emits startup and turn effects in order

r[agent_harness_runtime.turn_lifecycle_hooks.fsm.positive]
- GIVEN a new turn lifecycle FSM
- WHEN startup, pre-turn, post-turn, and completion events are advanced in order
- THEN the FSM returns hook effects for `session_start`, `pre_turn`, and `post_turn` in deterministic order
- AND the completed state has no pending adapter-owned transition.

#### Scenario: Invalid turn order is rejected

r[agent_harness_runtime.turn_lifecycle_hooks.fsm.negative]
- GIVEN a turn lifecycle FSM that has not entered a pre-turn state
- WHEN a post-turn event is advanced
- THEN the transition is rejected with a typed invalid-transition diagnostic
- AND no lifecycle hook effect is emitted.

### Requirement: Turn hooks cannot bypass policy boundaries

r[agent_harness_runtime.turn_lifecycle_hooks.no_bypass] Turn lifecycle hook outputs MUST respect event-scoped capabilities: `pre_turn` MAY deny before loop-fabric evaluation, while `post_turn` MUST NOT deny completed work, request revalidation of already-used payloads, execute tools, rewrite denied actions, weaken sandbox policy, or suppress approval.

#### Scenario: Post-turn denial is rejected

r[agent_harness_runtime.turn_lifecycle_hooks.no_bypass.post_turn]
- GIVEN a post-turn hook output requests denial or direct tool execution
- WHEN runtime validates the output
- THEN validation rejects the output before binding it to policy
- AND no adapter, tool, sandbox, or approval state is changed.

### Requirement: Turn lifecycle hook verification

r[agent_harness_runtime.turn_lifecycle_hooks.verification] Animus MUST keep turn lifecycle hook docs, focused tests, Cairn validation, and explicit non-claim evidence in sync with the runtime implementation.

#### Scenario: Focused validation names covered and non-claimed surfaces

r[agent_harness_runtime.turn_lifecycle_hooks.verification.focused]
- GIVEN turn lifecycle hook implementation changes
- WHEN maintainers run focused validation
- THEN evidence names the hook/FSM tests and Cairn gates that passed
- AND evidence states that concrete shell hook runners, Steel hook execution, persistence, network exporters, and daemon behavior are not implemented by this change.
