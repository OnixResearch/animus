# agent-harness-runtime Change Spec: Model/provider dispatch planning

## Requirements

### Requirement: Pure model dispatch plan

r[agent_harness_runtime.model_dispatch.plan] Model/provider dispatch MUST have a pure planning DTO that records selected capability identity, request contract identity, prompt or content budget, stream policy, receipt policy, cancellation or timeout posture, and fallback policy before adapter execution.

#### Scenario: Model dispatch plan validates before provider effects

r[agent_harness_runtime.model_dispatch.plan.validation]
- GIVEN runtime prepares a model/provider call from validated request metadata and host policy inputs
- WHEN model dispatch planning runs
- THEN valid dispatch plans are admitted with safe identity and budget evidence
- AND missing providers, denied capability, budget overflow, unsupported stream policy, malformed request shape, missing receipt policy, cancellation, timeout, and fallback-denied cases fail before provider effects.

### Requirement: Runtime gates model adapter calls on admitted plans

r[agent_harness_runtime.model_dispatch.runtime_gate] Runtime MUST call model adapters only after an admitted model dispatch plan and MUST map denied plans to typed fail-closed outcomes.

#### Scenario: Denied model plan prevents adapter call

r[agent_harness_runtime.model_dispatch.runtime_gate.fail_closed]
- GIVEN a model dispatch plan is denied because capability, budget, stream, receipt, cancellation, timeout, or fallback policy fails
- WHEN runtime processes the model decision
- THEN the model adapter is not called
- AND runtime returns a typed fail-closed outcome without completed receipts, provider discovery, credential reads, filesystem scans, process spawning, plugin loading, or network fallback.

### Requirement: Model dispatch compatibility defaults

r[agent_harness_runtime.model_dispatch.compatibility] Existing simple model adapter behavior MUST remain available through compatibility defaults for request identity, provider-neutral capability, prompt budget, stream policy, receipt policy, and fallback denial.

#### Scenario: Existing model adapter path matches planned dispatch

r[agent_harness_runtime.model_dispatch.compatibility.equivalent]
- GIVEN one runtime loop uses the existing simple model adapter path and another loop uses explicit model dispatch planning with compatibility defaults
- WHEN both receive equivalent fabric decisions and deterministic model responses
- THEN both produce equivalent successful reports and events
- AND neither path gains additional provider, filesystem, process, network, credential, or plugin authority.

### Requirement: Model dispatch planning verification

r[agent_harness_runtime.model_dispatch.verification] Model/provider dispatch planning MUST include focused positive and negative tests for planning validation, runtime gating, compatibility defaults, no-fallback behavior, provider-contract docs, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers model dispatch planning

r[agent_harness_runtime.model_dispatch.verification.closeout]
- GIVEN maintainers intend to archive model/provider dispatch planning
- WHEN closeout validation runs
- THEN evidence names planning validation, runtime gating, compatibility defaults, fail-closed, no-fallback, provider-contract docs, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
