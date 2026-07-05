# agent-harness-runtime Change Spec: Claude Code headless surface adaptation

## Requirements

### Requirement: Bounded headless automation reference

r[agent_harness_runtime.claude_code_headless.scope] Animus MUST treat Claude Code CLI, CI, and SDK automation as a bounded reference for noninteractive agent runs, and MUST NOT copy Claude Code flags, require cloud sessions, create pull requests by default, or make networked providers mandatory.

#### Scenario: CI run does not open a pull request by default

r[agent_harness_runtime.claude_code_headless.scope.no_default_pr]
- GIVEN a headless run completes with a patch summary or commit-message proposal
- WHEN final automation policy is evaluated
- THEN Animus does not open, merge, ready, or reopen a pull request by default
- AND any VCS or hosting side effect requires an explicit host adapter and policy grant.

### Requirement: Typed headless run request

r[agent_harness_runtime.claude_code_headless.request] Headless run requests MUST declare objective, session or workspace identity, input message or stdin payload references, policy profile, allowed capabilities, output contract, validation plan, and noninteractive behavior before runtime execution.

#### Scenario: Malformed headless request fails before execution

r[agent_harness_runtime.claude_code_headless.request.invalid]
- GIVEN a headless request is missing objective, identity, policy profile, output contract, or noninteractive behavior
- WHEN request validation runs
- THEN validation returns typed diagnostics
- AND no model, tool, shell, network, filesystem, or worker service is called.

### Requirement: Stable headless event DTOs

r[agent_harness_runtime.claude_code_headless.events] Headless runs MUST emit stable event DTOs for lifecycle, prompt assembly, model requests and responses, tool dispatch, permission decisions, background work, validation, receipts, and final response using bounded safe payloads.

#### Scenario: Oversized event payload is bounded

r[agent_harness_runtime.claude_code_headless.events.bounded]
- GIVEN a model, tool, validation, or background event contains large output
- WHEN the headless event is prepared
- THEN model-visible and event-visible payloads are bounded according to policy
- AND safe metadata records truncation state, sizes, and BLAKE3 hashes without embedding raw full bodies.

### Requirement: JSONL and Preserves projections

r[agent_harness_runtime.claude_code_headless.serialization] Headless event DTOs SHOULD provide JSONL and Preserves boundary projections that preserve event identity, ordering metadata, status classes, redaction markers, and BLAKE3 hashes without making either format the core state model.

#### Scenario: Unsupported projection fails safely

r[agent_harness_runtime.claude_code_headless.serialization.unsupported]
- GIVEN a host requests an unsupported headless event projection
- WHEN serialization is selected
- THEN runtime returns a typed unsupported-projection error
- AND the run does not fall back to lossy or unbounded debug text.

### Requirement: Noninteractive completion statuses

r[agent_harness_runtime.claude_code_headless.completion] Headless completion MUST use typed noninteractive status classes for success, denied, prompt-required, input-required, validation-failed, service-unavailable, cancelled, and execution-failed outcomes with safe evidence references.

#### Scenario: Permission ask becomes prompt-required

r[agent_harness_runtime.claude_code_headless.completion.prompt_required]
- GIVEN a noninteractive headless run reaches a permission decision that requires user confirmation
- WHEN policy does not provide a valid preauthorized prompt-decision token
- THEN the run completes with prompt-required status
- AND no guarded action is dispatched after the unresolved prompt.
