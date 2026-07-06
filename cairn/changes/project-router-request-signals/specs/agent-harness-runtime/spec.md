# agent-harness-runtime Change Spec: Project router request signals from typed metadata

## Requirements

### Requirement: Pure request-signal projection from bounded metadata

r[agent_harness_runtime.model_router.request_projection] Animus MUST provide a pure deterministic `project_signals` function that derives the eight router signal bands (task shape, risk, contract pressure, context pressure, latency, cost, privacy/locality, fallback eligibility) from bounded `RequestMetadata` using named threshold constants, without requiring raw prompt bodies, provider payloads, credentials, or unbounded text.

#### Scenario: Reasoning-heavy request projects to a collaborative band

r[agent_harness_runtime.model_router.request_projection.reasoning]
- GIVEN request metadata has a large prompt estimate, no tool calls, and a remote-allowed privacy policy
- WHEN `project_signals` runs
- THEN the task shape is reasoning and risk is high or strict
- AND no raw prompt body is consumed or emitted.

#### Scenario: Missing privacy policy fails closed

r[agent_harness_runtime.model_router.request_projection.missing_privacy]
- GIVEN request metadata omits a privacy policy label
- WHEN `project_signals` runs
- THEN it returns a `MissingPrivacyPolicy` error
- AND no `SignalDraft` is produced.

### Requirement: Pluggable SignalSource adapter seam

r[agent_harness_runtime.model_router.request_projection.signal_source] Animus MUST define an implementation-free `SignalSource` adapter trait in `animus-adapters` that returns a `SignalDraft` from `RequestMetadata`, and the rule-based projector MUST be the default `SignalSource` implementation in `animus-router` so hosts may inject richer classifiers without changing router core.

#### Scenario: Host classifier overrides the rule projector

r[agent_harness_runtime.model_router.request_projection.signal_source.override]
- GIVEN a host injects a custom `SignalSource`
- WHEN the router requests signals
- THEN it calls the injected source's `project`
- AND the rule projector is used only when no source is injected.

### Requirement: Request-signal projection verification

r[agent_harness_runtime.model_router.request_projection.verification] The change MUST include focused positive and negative tests for each band derivation, the default `SignalSource` end-to-end path, malformed metadata, missing privacy policy, unsafe-payload rejection, and bounded-oversized inputs before archive.

#### Scenario: Closeout evidence covers signal projection

r[agent_harness_runtime.model_router.request_projection.verification.closeout]
- GIVEN maintainers intend to archive the request-signal projection change
- WHEN closeout validation runs
- THEN evidence names focused band-derivation, signal-source, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
