## ADDED Requirements

### Requirement: Bounded serving availability snapshots
r[agent_harness_runtime.serving_admission.snapshot] Animus MUST accept host-supplied `animus.serving-availability-snapshot.v1` snapshots that bind source identity and generation, sequence, logical observation window, and per service the logical service, endpoint identity, deployment and generation, locality, exact model and configuration with claimed or observed attestation, readiness (absent, unknown, loading, ready, draining, unavailable, stale), capabilities, request envelope, and capacity observations with explicit kind, unit, scope, origin, and freshness. Unknown values MUST stay unknown and snapshots MUST be bounded.

#### Scenario: Malformed snapshot
- GIVEN a snapshot with a mismatched unit, inverted window, duplicate service, unsafe token, or noncanonical reference
- WHEN the core validates it
- THEN it is rejected with a typed contract issue

### Requirement: Eligibility before preference
r[agent_harness_runtime.serving_admission.eligibility] Under an explicit capacity-aware profile, Animus MUST exclude stale, out-of-order, non-ready, wrong-model, wrong-configuration, privacy-, capability-, or envelope-incompatible, and insufficiently evidenced routes before recipe selection, rotation, or learned preference. Free memory, queue depth, and host probes MUST NOT count as request capacity.

#### Scenario: Learned preference names an excluded route
- GIVEN a draining route ranked first by a preference
- WHEN the preference is applied
- THEN only eligible routes are considered

### Requirement: Authoritative pre-dispatch gate
r[agent_harness_runtime.serving_admission.gate] Protected dispatch MUST require existing policy decisions, a serving-owner grant for the exact request, deployment generation, model, and configuration, and a current recheck immediately before provider contact. Expired or revoked grants, unavailable admission or recheck services, generation or endpoint replacement, model substitution, and pinned replica changes MUST prevent dispatch.

#### Scenario: Two callers observe the last slot
- GIVEN two callers screen a snapshot reporting one request slot
- WHEN both request owner admission
- THEN at most the granted caller dispatches

### Requirement: Metadata-only usage receipts
r[agent_harness_runtime.serving_admission.receipt] Each invocation MUST produce an `animus.serving-invocation-receipt.v1` receipt keyed to the caller identities and named route identity, recording deployment, model, attestation, capacity evidence, admission, replica change, outcome class distinguishing pre-dispatch refusal from uncertain post-contact results, response provenance, usage provenance, fixture or live label, and non-claims, without prompts, secrets, or raw payloads.

#### Scenario: Usage without dispatch
- GIVEN a receipt input reporting provider usage for an undispatched request
- WHEN the receipt is projected
- THEN projection is rejected as inconsistent

### Requirement: Opt-in rollout version binding
r[agent_harness_runtime.serving_admission.rollout] A route that names a rollout requirement MUST be eligible only with matching qualified rollout facts and an observed policy generation, and its grant, recheck, and response generation MUST match; missing or crossed response generations MUST be propagated unchanged.

#### Scenario: Configuration drift after qualification
- GIVEN rollout facts qualified for another configuration
- WHEN a rollout route is screened
- THEN it is excluded with a qualification mismatch

### Requirement: One injected host path
r[agent_harness_runtime.serving_admission.host_path] The non-default `serving-admission` feature MUST compose screening, existing selection, an injected serving-owner port, the gate, the existing Mesh shell, and receipt projection without discovery, allocation, probing, or retry. The standalone owner MUST fail closed.

#### Scenario: No owner configured
- GIVEN the `UnavailableOwner`
- WHEN an eligible request is invoked
- THEN it is blocked before provider contact
