# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Animus pilots the accepted Kamacite binding contract

r[agent_harness_runtime.application_host_binding_pilot.kamacite_contract] Animus MUST consume one exact accepted Kamacite `ApplicationHostBinding` schema identity, producer revision, canonical fixture cohort, and validation receipt before claiming pilot conformance.

#### Scenario: Exact upstream contract is available

r[agent_harness_runtime.application_host_binding_pilot.kamacite_contract.available]
- GIVEN an accepted Kamacite binding schema and frozen producer fixture are available
- WHEN Animus admits the pilot contract
- THEN Animus MUST bind the exact schema, revision, canonical bytes, receipt identity, and required non-claims.

#### Scenario: Upstream contract is missing or stale

r[agent_harness_runtime.application_host_binding_pilot.kamacite_contract.stale]
- GIVEN the Kamacite schema, producer revision, fixture bytes, or receipt identity is missing or differs from the reviewed cohort
- WHEN pilot conformance runs
- THEN Animus MUST report an exact blocker
- AND it MUST NOT claim binding compatibility.

### Requirement: The pilot projects existing Animus identities

r[agent_harness_runtime.application_host_binding_pilot.projection] Animus MUST project one non-default deterministic fake-backed headless application entrypoint into the Kamacite binding shape by referencing existing application, entrypoint, input/output schema, effect-summary, session runtime authority profile, adapter-cohort, resource-profile, and policy identities.

#### Scenario: Existing composition projects canonically

r[agent_harness_runtime.application_host_binding_pilot.projection.valid]
- GIVEN the pilot entrypoint and every existing Animus composition identity are valid
- WHEN the pure projection is derived
- THEN the projection MUST match the frozen Kamacite contract
- AND it MUST NOT copy prompts, policy bodies, tool payloads, credentials, endpoints, environment values, or physical host paths.

### Requirement: The pilot selects one host profile

r[agent_harness_runtime.application_host_binding_pilot.host_selection] The pilot binding MUST reference exactly one existing session runtime authority profile and MUST deny missing, multiple, implicit, alternative, or fallback host-profile selection.

#### Scenario: One admitted session profile is selected

r[agent_harness_runtime.application_host_binding_pilot.host_selection.valid]
- GIVEN the host has admitted one exact session runtime authority profile
- WHEN the pilot binding is prepared
- THEN that profile identity MUST be the binding's only host-profile identity.

#### Scenario: Host profile is ambiguous

r[agent_harness_runtime.application_host_binding_pilot.host_selection.ambiguous]
- GIVEN the pilot has no host profile or has multiple, implicit, alternative, or fallback profiles
- WHEN binding preparation runs
- THEN dispatch MUST remain denied with a deterministic host-selection failure.

### Requirement: Pilot effects are closed and authority-free

r[agent_harness_runtime.application_host_binding_pilot.effects] The pilot MUST export one closed Kamacite-compatible symbolic effect summary and MUST reject open, unknown, undeclared, or stale operations before dispatch. The summary MUST NOT grant model, tool, provider, filesystem, process, network, credential, sandbox, or mutation authority.

#### Scenario: Closed fake-backed effects are visible

r[agent_harness_runtime.application_host_binding_pilot.effects.closed]
- GIVEN the deterministic pilot uses only its declared fake-backed model and tool operations
- WHEN the binding is explained or validated
- THEN every symbolic operation MUST appear by exact identity in the closed effect summary.

#### Scenario: Unknown operation is requested

r[agent_harness_runtime.application_host_binding_pilot.effects.unknown]
- GIVEN the entrypoint requests an operation absent from the closed summary
- WHEN dispatch admission runs
- THEN dispatch MUST remain denied
- AND no existing host capability MUST be inferred from the operation name.

### Requirement: Binding is committed before dependent effects

r[agent_harness_runtime.application_host_binding_pilot.pre_effect_binding] The pilot host MUST commit the exact application-host binding beside the existing session authority binding before the first dependent model or tool effect.

#### Scenario: First dependent effect follows binding commit

r[agent_harness_runtime.application_host_binding_pilot.pre_effect_binding.order]
- GIVEN a valid pilot binding and session authority binding
- WHEN the host prepares the first model or tool dispatch
- THEN durable ordering evidence MUST place both bindings before the dependent effect.

#### Scenario: Effect occurs before binding commit

r[agent_harness_runtime.application_host_binding_pilot.pre_effect_binding.denied]
- GIVEN no exact application-host binding is committed for the current pilot unit
- WHEN model or tool dispatch is requested
- THEN the pilot path MUST deny dispatch rather than backfill the binding after the effect.

### Requirement: Current enforcement remains authoritative

r[agent_harness_runtime.application_host_binding_pilot.current_enforcement] A matching pilot binding MUST NOT replace current authorization, capability probes, adapter-cohort checks, tool and provider readiness, credential admission, sandbox preflight, cancellation, timeout, or per-run resource enforcement.

#### Scenario: Binding matches but current enforcement fails

r[agent_harness_runtime.application_host_binding_pilot.current_enforcement.failure]
- GIVEN every binding identity matches
- AND a current authorization, readiness, sandbox, cancellation, timeout, or resource check fails
- WHEN dispatch admission runs
- THEN dispatch MUST remain denied by the existing Animus enforcement path.

### Requirement: Pilot explanation is read-only and safe

r[agent_harness_runtime.application_host_binding_pilot.explain] Animus MUST provide a read-only pilot explanation that displays safe application, entrypoint, effect, host, adapter, resource, policy, compatibility, and binding identities with explicit non-claims.

#### Scenario: Operator inspects the effective binding

r[agent_harness_runtime.application_host_binding_pilot.explain.safe]
- GIVEN a valid or denied pilot binding attempt
- WHEN the operator requests the explanation
- THEN the output MUST show safe identity and reason fields plus the next focused check
- AND it MUST NOT expose raw prompts, credentials, provider payloads, tool payloads, environment values, or physical host paths.

### Requirement: Pilot claims remain bounded

r[agent_harness_runtime.application_host_binding_pilot.boundary] Animus documentation, types, fixtures, explanations, and receipts MUST state that pilot conformance proves a bounded projection over one measured cohort and does not prove effect completeness, application correctness, authorization, host behavior, sandboxing, provider correctness, tool safety, runtime success, or release eligibility.

#### Scenario: Binding is described as execution authority

r[agent_harness_runtime.application_host_binding_pilot.boundary.overclaim]
- GIVEN a fixture, receipt, explanation, or document describes a matching binding as sufficient execution or authorization evidence
- WHEN boundary validation runs
- THEN the artifact MUST fail with a stable overclaim issue.

### Requirement: Pilot verification covers success and failure

r[agent_harness_runtime.application_host_binding_pilot.verification] The pilot MUST include focused positive and negative tests for contract freshness, canonical projection, one-host selection, closed effects, pre-effect ordering, current enforcement, safe explanation, stale cohorts, unsafe fields, and non-claims.

#### Scenario: Focused pilot rail closes the change

r[agent_harness_runtime.application_host_binding_pilot.verification.closeout]
- GIVEN maintainers intend to sync and archive the pilot
- WHEN focused and repository validation runs
- THEN exact Kamacite conformance, positive cases, negative cases, safe output, current enforcement, documentation, Cairn validation, and Cairn gates MUST pass
- AND any unavailable upstream or optional check MUST remain an explicit blocker rather than assumed success.
