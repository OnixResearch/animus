# agent-harness-runtime Change Spec: Add the Supra-Router signal adapter

## Requirements

### Requirement: Optional Supra-Router signal source

r[agent_harness_runtime.model_router.signal_adapter] Animus MUST offer an optional, non-default `SupraRouterSignalSource` in `animus-std` behind a `supra-router` feature that implements the `SignalSource` trait by mapping Supra-Router classification output to `SignalDraft` bands, and routing correctness MUST NOT depend on it; the rule-based projector MUST remain the default `SignalSource`.

#### Scenario: Supra-Router classification maps to bands

r[agent_harness_runtime.model_router.signal_adapter.mapping]
- GIVEN a Supra-Router response classifies a prompt as high-complexity coding with a big-model route decision
- WHEN `SupraRouterSignalSource::project` runs
- THEN it produces a `SignalDraft` with coding task shape, high risk, and remote-allowed privacy
- AND the bands are forwarded to `Signals::project` without raw prompt bodies.

#### Scenario: Adapter is off by default

r[agent_harness_runtime.model_router.signal_adapter.optional]
- GIVEN an embedder builds `animus-std` without the `supra-router` feature
- WHEN the runtime selects a `SignalSource`
- THEN the rule-based projector is used
- AND no Supra-Router network dependency is compiled in.

### Requirement: Supra-Router adapter fails closed

r[agent_harness_runtime.model_router.signal_adapter.fail_closed] The adapter MUST fall back to the rule-based `RuleSignalSource` or deny per Nickel policy when Supra-Router is missing, unavailable, timed out, cancelled, or returns malformed output, and MUST NOT block routing on classifier availability.

#### Scenario: Unavailable endpoint falls back

r[agent_harness_runtime.model_router.signal_adapter.fail_closed.unavailable]
- GIVEN the Supra-Router endpoint is unavailable or times out
- WHEN `project` runs
- THEN the adapter falls back to the rule projector (or denies per policy)
- AND no routing decision is blocked on the classifier.

#### Scenario: Malformed classification rejected

r[agent_harness_runtime.model_router.signal_adapter.fail_closed.malformed]
- GIVEN Supra-Router returns a malformed or unknown classification
- WHEN the adapter evaluates the response
- THEN it treats the response as unusable and falls back or denies
- AND no malformed band reaches recipe selection.

### Requirement: Supra-Router adapter verification

r[agent_harness_runtime.model_router.signal_adapter.verification] The change MUST include focused positive and negative tests for classification mapping, the stub client, unavailable/malformed/timeout/cancelled responses, policy-deny fallback, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers the Supra-Router adapter

r[agent_harness_runtime.model_router.signal_adapter.verification.closeout]
- GIVEN maintainers intend to archive the Supra-Router adapter change
- WHEN closeout validation runs
- THEN evidence names focused mapping, fail-closed, negative, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
