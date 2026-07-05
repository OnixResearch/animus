# agent-harness-runtime Change Spec: Model capability matrix negotiation

## Requirements

### Requirement: Provider/model capability matrix

r[agent_harness_runtime.model_capability_matrix.registry] Provider/model metadata MUST expose a typed capability matrix for canonical model request compatibility, including supported message roles, content part classes, tool-call behavior, structured-output modes, stream channels, reasoning controls, context and output budgets, privacy or locality posture, and capability schema identity.

#### Scenario: Registry metadata declares compatibility features

r[agent_harness_runtime.model_capability_matrix.nickel_config]
- GIVEN Animus-owned provider registry configuration describes one or more selectable models
- WHEN Nickel export and Rust-side capability validation run
- THEN every selectable model has bounded typed compatibility metadata for the canonical request features it claims
- AND missing, unknown, contradictory, or malformed required capability fields fail before runtime can select that model.

### Requirement: Pure capability negotiation

r[agent_harness_runtime.model_capability_matrix.negotiation] Animus MUST negotiate canonical model request requirements against the selected model capability matrix through a pure deterministic function that returns admitted, downgraded, or denied compatibility decisions with safe evidence.

#### Scenario: Supported or optional features resolve deterministically

r[agent_harness_runtime.model_capability_matrix.negotiation.positive]
- GIVEN a canonical model request declares required and optional feature requirements
- AND the selected model capability matrix supports every required feature and supports or policy-downgrades optional features
- WHEN capability negotiation runs
- THEN the result is admitted or explicitly downgraded with safe feature labels, policy labels, and bounded budget evidence
- AND no provider call, credential read, network probe, filesystem scan, or router fallback occurs during negotiation.

### Requirement: Required unsupported features fail closed

r[agent_harness_runtime.model_capability_matrix.fail_closed] Capability negotiation MUST deny dispatch when a canonical request requires a role, content part, image, tool call, structured-output mode, stream channel, reasoning level, token budget, privacy class, or provider family behavior that the selected model does not support or whose support is unknown.

#### Scenario: Unsupported required feature blocks dispatch

r[agent_harness_runtime.model_capability_matrix.fail_closed.unsupported]
- GIVEN a canonical model request requires a compatibility feature that the selected model metadata lacks, rejects, or marks unknown
- WHEN capability negotiation evaluates the request
- THEN dispatch is denied with a typed unsupported-feature diagnostic
- AND runtime does not silently drop the feature, switch providers, widen authority, claim success receipts, or call a provider adapter.

### Requirement: Capability matrix verification

r[agent_harness_runtime.model_capability_matrix.verification] Model capability matrix work MUST include focused positive and negative tests for Nickel config validation, Rust metadata validation, pure negotiation, optional downgrade evidence, unsupported required features, budget overflow, denied fallback, safe receipts, docs, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers capability negotiation

r[agent_harness_runtime.model_capability_matrix.verification.closeout]
- GIVEN maintainers intend to archive capability matrix negotiation
- WHEN closeout validation runs
- THEN evidence names Nickel config validation, Rust metadata validation, pure negotiation, optional downgrade evidence, unsupported required features, budget overflow, denied fallback, safe receipts, docs, facade inventory, workspace checks, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
