# agent-harness-runtime Change Spec: Hermes ROI recommendation contracts

## Requirements

### Requirement: ROI recommendations are typed structured reports

r[agent_harness_runtime.hermes_roi.recommendation] Animus MUST represent ROI recommendations as optional typed reports containing focus, best next step, score dimensions, first actions, runner-ups, lifecycle status, recommended action, reason, missing context, safe evidence references, and receipt metadata.

#### Scenario: Valid ROI report names one best next step

r[agent_harness_runtime.hermes_roi.recommendation.valid]
- GIVEN a host supplies a structured ROI report with focus, best next step, score dimensions, runner-ups, lifecycle status, and missing-context fields
- WHEN report validation runs
- THEN Animus accepts the report as typed recommendation data
- AND no repository scan, model call, UI action, or lifecycle mutation occurs during validation.

### Requirement: ROI report validation bounds score and evidence fields

r[agent_harness_runtime.hermes_roi.report_validation] ROI report validation MUST reject missing best-next-step fields, unsupported recommended actions, malformed score dimensions, unbounded prose, raw prompts, provider payloads, credentials, full private paths, raw diff bodies, raw tool outputs, and hidden reasoning text.

#### Scenario: Unsafe evidence reference is rejected

r[agent_harness_runtime.hermes_roi.report_validation.unsafe]
- GIVEN an ROI report embeds raw provider JSON or a full private path as evidence
- WHEN validation runs
- THEN validation returns a typed unsafe-evidence diagnostic
- AND no action descriptor is admitted from that report.

### Requirement: ROI actions require host admission

r[agent_harness_runtime.hermes_roi.action_descriptors] ROI follow-ups such as begin-now, lifecycle-first, investigate, delegate, dismiss, or no-op MUST be represented as action descriptors that require host admission before implementation, planning, delegation, validation, lifecycle mutation, or other side effects begin.

#### Scenario: Lifecycle-first hint does not create a Cairn by itself

r[agent_harness_runtime.hermes_roi.action_descriptors.no_auto_cairn]
- GIVEN an ROI report recommends lifecycle-first work
- WHEN action descriptor reduction runs
- THEN Animus returns a lifecycle-route descriptor with safe reason and suggested objective
- AND it does not create a Cairn change, edit tasks, write specs, or start implementation without a host action.

### Requirement: ROI evidence collection and execution are host-owned

r[agent_harness_runtime.hermes_roi.host_owned] Evidence collection, repository inspection, diff reading, active lifecycle detection, model scoring, UI selection, task execution, delegation, and lifecycle mutation for ROI workflows MUST remain host-owned behavior outside reusable runtime and adapter-contract crates.

#### Scenario: Missing ROI host service returns unavailable

r[agent_harness_runtime.hermes_roi.host_owned.missing_service]
- GIVEN a session requests an ROI recommendation
- AND no ROI evidence/scoring service is injected
- WHEN the host attempts to start ROI workflow through reusable contracts
- THEN Animus returns a service-unavailable diagnostic
- AND it does not inspect files, call a model, or choose a default prioritization policy.

### Requirement: ROI verification

r[agent_harness_runtime.hermes_roi.verification] The ROI adaptation MUST include focused positive and negative tests for valid recommendation reports, malformed reports, score dimension bounds, unknown recommended actions, action descriptor admission, lifecycle-route non-mutation, missing host services, no default `/roi` command, and receipt leak rejection.

#### Scenario: Closeout evidence records prioritizer non-claims

r[agent_harness_runtime.hermes_roi.verification.closeout]
- GIVEN maintainers intend to archive the ROI recommendation adaptation
- WHEN closeout validation runs
- THEN evidence names report DTO, parser, action descriptor, host-owned boundary, receipt, docs, Cairn validation, and Cairn gate checks
- AND it records built-in prioritization, repository scanning, model scoring, automatic Cairn creation, and automatic implementation as non-claims.
