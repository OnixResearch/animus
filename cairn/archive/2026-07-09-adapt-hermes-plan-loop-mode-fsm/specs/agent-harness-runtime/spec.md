# agent-harness-runtime Change Spec: Hermes plan/loop mode FSM contracts

## Requirements

### Requirement: Plan/loop modes are optional typed FSM data

r[agent_harness_runtime.hermes_plan_loop_modes.fsm] Animus MUST represent plan/loop-style modes as optional typed mode descriptors and finite-state data covering mode identity, action posture, capability ceilings, plan extraction rules, selected steps, loop conditions, breakout acknowledgements, retry classifications, and safe receipts.

#### Scenario: Host-defined planning mode validates

r[agent_harness_runtime.hermes_plan_loop_modes.fsm.host_defined]
- GIVEN a host defines a planning mode with read-only posture, explicit plan markers, and a selected-step output contract
- WHEN mode descriptor validation runs
- THEN Animus accepts the descriptor as bounded policy data
- AND reusable logic does not register `/plan`, install shortcuts, change active tools, or enforce read-only behavior by prompt text alone.

### Requirement: Plan extraction is explicit and safe

r[agent_harness_runtime.hermes_plan_loop_modes.plan_extraction] Plan extraction MUST be pure over supplied text or structured records and MUST require configured explicit markers or structured step data; it MUST reject arbitrary numbered-list capture, malformed markers, duplicate step IDs, stale selected-step updates, unsafe text, and unbounded plan bodies.

#### Scenario: Numbered prose is not captured as a plan

r[agent_harness_runtime.hermes_plan_loop_modes.plan_extraction.no_greedy_capture]
- GIVEN an assistant response includes a numbered explanation outside configured plan markers
- WHEN plan extraction runs
- THEN no executable plan steps are captured
- AND the mode state remains unchanged except for a safe no-plan diagnostic.

### Requirement: Loop work uses continuation policy

r[agent_harness_runtime.hermes_plan_loop_modes.continuation] Loop continuation, selected-step execution prompts, validation follow-ups, breakout acknowledgements, cancellation, retry classifications, and completion records MUST flow through typed continuation policy state and safe work IDs.

#### Scenario: Breakout acknowledgement must match active loop work

r[agent_harness_runtime.hermes_plan_loop_modes.continuation.breakout]
- GIVEN a loop work item is active with a safe work ID
- WHEN a host reports breakout success for a different or stale work ID
- THEN acknowledgement validation rejects the completion
- AND the active loop state is not cleared.

### Requirement: Plan/loop product controls are host-owned

r[agent_harness_runtime.hermes_plan_loop_modes.host_owned] Commands, shortcuts, UI selectors, editor integration, tool activation and deactivation, read-only enforcement, delegation strategy, timers, provider error probing, session persistence, and follow-up delivery MUST remain host-owned behavior outside reusable runtime and adapter-contract crates.

#### Scenario: Missing host mode controller does not install commands

r[agent_harness_runtime.hermes_plan_loop_modes.host_owned.no_commands]
- GIVEN an embedder depends on Animus runtime without a mode-controller host service
- WHEN the runtime starts or assembles prompt context
- THEN no mode commands, shortcuts, default loop tools, or tool allowlists are installed
- AND mode behavior is unavailable until the host explicitly injects descriptors and services.

### Requirement: Plan/loop mode verification

r[agent_harness_runtime.hermes_plan_loop_modes.verification] The plan/loop mode adaptation MUST include focused positive and negative tests for mode descriptors, marker-bound extraction, selected-step ordering, stale or duplicate step updates, loop continuation priority, breakout acknowledgement freshness, host-owned command/tool boundaries, unsafe prompt text, no default `/mode` behavior, and receipt leak rejection.

#### Scenario: Closeout evidence separates guidance from enforcement

r[agent_harness_runtime.hermes_plan_loop_modes.verification.closeout]
- GIVEN maintainers intend to archive the plan/loop mode adaptation
- WHEN closeout validation runs
- THEN evidence names FSM DTOs, extraction, continuation, host-owned boundaries, receipt, docs, Cairn validation, and Cairn gate checks
- AND it states that prompt-mode guidance does not replace capability gates, sandbox policy, confirmation, mutation serialization, or host read-only enforcement.
