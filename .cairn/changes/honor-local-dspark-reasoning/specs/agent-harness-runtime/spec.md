## ADDED Requirements

### Requirement: Live planning preserves validated reasoning policy

r[agent_harness_runtime.local_dspark_reasoning_planning.policy] The live Steel session planner MUST derive canonical reasoning controls from the exact validated provider cohort and MUST NOT replace them with a hard-coded reasoning level.

#### Scenario: Local reasoning stays disabled

r[agent_harness_runtime.local_dspark_reasoning_planning.policy.none]
- GIVEN the exact local-dspark configuration validates reasoning effort `none`
- WHEN the live planner creates an initial or continuation request
- THEN canonical reasoning is disabled and the wire request sends explicit `reasoning.effort = "none"`

#### Scenario: Existing low reasoning is preserved

r[agent_harness_runtime.local_dspark_reasoning_planning.policy.low]
- GIVEN an admitted OpenAI or OpenRouter configuration validates reasoning effort `low`
- WHEN the live planner creates a request
- THEN canonical reasoning remains enabled at low effort

#### Scenario: Unknown reasoning fails closed

r[agent_harness_runtime.local_dspark_reasoning_planning.policy.failure]
- GIVEN a reasoning label is not `none` or `low`
- WHEN reasoning projection runs
- THEN planning fails before credential use or provider contact

### Requirement: Reasoning planning has bounded verification

r[agent_harness_runtime.local_dspark_reasoning_planning.verification] Verification MUST include positive disabled and low projections, a negative unknown projection, strict Clippy, formatting, focused Nix checks, and downstream live product evidence.

#### Scenario: Downstream product no longer fails on hidden reasoning

r[agent_harness_runtime.local_dspark_reasoning_planning.verification.live]
- GIVEN Animus Steel pins the fixed revision and selects the exact local cohort
- WHEN one bounded live prompt runs
- THEN the request does not emit unsupported hidden-reasoning events and the product can complete or return another exact bounded provider failure
