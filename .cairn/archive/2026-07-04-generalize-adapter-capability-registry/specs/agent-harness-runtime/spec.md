# agent-harness-runtime Change Spec: Adapter capability registry

## Requirements

### Requirement: Typed capability descriptors

r[agent_harness_runtime.composability.capability_descriptors] Animus runtime decisions and service-backed helpers MUST represent required host authority as typed capability descriptors rather than implicit model, tool, queue, event, policy, confirmation, cancellation, or provenance assumptions.

#### Scenario: Capability descriptors validate before resolution

r[agent_harness_runtime.composability.capability_descriptors.validation]
- GIVEN a runtime plan declares required host capabilities
- WHEN descriptor validation runs
- THEN valid descriptors are accepted with stable labels and service kinds
- AND duplicate labels, empty labels, unsupported service kinds, or unsafe scope text fail before service resolution.

### Requirement: Host-supplied capability registry

r[agent_harness_runtime.composability.capability_registry] Animus MAY use a host-supplied capability bundle or registry to resolve explicit service references, but reusable runtime code MUST NOT construct providers, tools, queues, stores, event sinks, plugin runtimes, or other ambient backends.

#### Scenario: Multiple services compose through explicit injection

r[agent_harness_runtime.composability.capability_registry.composition]
- GIVEN a host supplies model, tool, event sink, policy, queue, cancellation, confirmation, and provenance capabilities
- WHEN runtime resolves a plan requiring a subset of those capabilities
- THEN only explicitly supplied services are eligible for dispatch
- AND duplicate registrations or missing required capabilities return typed diagnostics without ambient fallback.

### Requirement: Model and tool adapter compatibility

r[agent_harness_runtime.composability.adapter_compatibility] Existing simple model/tool adapter constructors MUST remain available or be replaced by explicit migration helpers that are thin wrappers over the capability bundle.

#### Scenario: Compatibility path preserves behavior

r[agent_harness_runtime.composability.adapter_compatibility.equivalent]
- GIVEN a runtime loop uses the existing model/tool constructor path and another loop uses the explicit capability-bundle path
- WHEN both loops receive equivalent fabric decisions and adapter responses
- THEN they produce equivalent typed reports, events, failures, and receipts
- AND neither path gains additional ambient authority.

### Requirement: Capability calls fail closed

r[agent_harness_runtime.composability.capability_fail_closed] Capability resolution and calls MUST return typed absent, denied, unavailable, malformed, cancelled, timed-out, and completed outcomes, and non-completed outcomes MUST prevent success receipts and later side effects.

#### Scenario: Missing capability cannot fall back to ambient discovery

r[agent_harness_runtime.composability.capability_fail_closed.no_fallback]
- GIVEN a runtime action requires a capability that is absent, denied, unavailable, malformed, cancelled, or timed out
- WHEN dispatch attempts to resolve or call that capability
- THEN runtime returns the matching fail-closed outcome
- AND it does not read environment variables, scan files, load plugins, spawn processes, contact providers, or claim completed effects as a fallback.

### Requirement: Capability registry verification

r[agent_harness_runtime.composability.capability_registry.verification] The capability registry change MUST include focused positive and negative tests for descriptor validation, single-service and multi-service composition, compatibility constructors, fail-closed outcomes, no ambient fallback, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers capability composition

r[agent_harness_runtime.composability.capability_registry.verification.closeout]
- GIVEN maintainers intend to archive the capability registry change
- WHEN closeout validation runs
- THEN evidence names descriptor, registry, compatibility, fail-closed, no-fallback, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
