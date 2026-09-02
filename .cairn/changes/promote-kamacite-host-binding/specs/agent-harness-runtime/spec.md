# Agent Harness Runtime Specification Delta

## Purpose

Promote the Kamacite application-host binding from a fake-backed pilot into one real Animus composition path without transferring host authority.

## ADDED Requirements

### Requirement: Live binding uses one immutable Kamacite cohort

r[agent_harness_runtime.kamacite_live_binding.cohort] Animus MUST pin one immutable Kamacite application-host binding, semantic operation, receipt, codec, and identity-domain cohort for the selected live path.

#### Scenario: Exact cohort is admitted

- GIVEN every required revision, schema, codec, receipt, and identity-domain value matches the reviewed cohort
- WHEN live binding admission starts
- THEN Animus MUST admit the cohort for the selected composition

#### Scenario: Upstream cohort drifts

- GIVEN any required Kamacite identity differs from the reviewed cohort
- WHEN live binding admission starts
- THEN Animus MUST fail before application or host dispatch

### Requirement: Kamacite values remain in a narrow adapter

r[agent_harness_runtime.kamacite_live_binding.adapter] Animus MUST decode and validate Kamacite artifacts in a focused adapter that returns Animus-owned nominal binding and operation types.

#### Scenario: Canonical binding enters the adapter

- GIVEN a bounded canonical binding artifact matches the reviewed schema
- WHEN the adapter validates and converts it
- THEN downstream Animus logic MUST receive only validated Animus-owned types

#### Scenario: Generic value reaches runtime policy

- GIVEN a generic Kamacite or Preserves value is passed toward core, session, or ordinary runtime APIs
- WHEN dependency and API validation runs
- THEN validation MUST fail before the value can influence a decision

### Requirement: One real live-agent composition binds the complete unit

r[agent_harness_runtime.kamacite_live_binding.composition] The selected `animus-live-steel-agent` path MUST bind exact application, entrypoint, input schema, output schema, effect summary, host profile, adapter cohort, resource profile, and policy identities.

#### Scenario: Complete live unit matches

- GIVEN the current product composition matches every identity in the admitted binding
- WHEN composition admission runs
- THEN Animus MAY prepare the existing runtime authority unit

#### Scenario: Correct application has the wrong host

- GIVEN the application identity matches but the host, adapter, resource, schema, effect, or policy identity differs
- WHEN composition admission runs
- THEN Animus MUST deny before provider or tool contact

### Requirement: Closed effects use exact semantic operation identities

r[agent_harness_runtime.kamacite_live_binding.effects] The live binding MUST use exact Kamacite `effect-operation` identities for every selected model and tool requirement. Names and local action labels MUST NOT substitute for identity.

#### Scenario: Declared operation matches

- GIVEN one model or tool operation matches an exact identity in the closed effect summary
- WHEN dispatch preparation runs
- THEN Animus MAY continue to its existing authority and adapter gates

#### Scenario: Effect set is open or stale

- GIVEN the effect summary is open, missing, duplicated, stale, unknown, or name-only
- WHEN dispatch preparation runs
- THEN Animus MUST deny the dependent effect

### Requirement: Existing pre-effect authority remains final

r[agent_harness_runtime.kamacite_live_binding.preeffect] Binding admission MUST occur before the current runtime authority seal. It MUST NOT bypass current authorization, capability, adapter, credential, sandbox, cancellation, timeout, or resource checks.

#### Scenario: Binding matches but capability is stale

- GIVEN the binding matches and a current capability or adapter recheck fails
- WHEN `start_runtime_authority_effect` evaluates the unit
- THEN the effect MUST remain denied

### Requirement: Rollout is explicit and has no silent fallback

r[agent_harness_runtime.kamacite_live_binding.rollout] The Kamacite-bound path MUST be non-default until its packaged gates pass. Selection MUST be typed, explicit, and mechanically removable without state reinterpretation.

#### Scenario: Bound path fails admission

- GIVEN an operator selected the Kamacite-bound path and binding admission fails
- WHEN startup or dispatch continues
- THEN Animus MUST return a bounded failure and MUST NOT use the legacy path

#### Scenario: Legacy path is selected

- GIVEN typed configuration explicitly selects the retained legacy composition
- WHEN the host starts
- THEN existing session and runtime identities MUST retain their prior meaning

### Requirement: Binding evidence is safe metadata

r[agent_harness_runtime.kamacite_live_binding.evidence] Selected receipts MUST bind application-host binding and semantic operation identities without prompts, responses, tool bodies, credentials, endpoints, environment values, or private paths.

#### Scenario: Safe dispatch receipt is emitted

- GIVEN a selected bound dispatch reaches a recorded terminal observation
- WHEN receipt projection runs
- THEN it MUST retain binding and operation roles with existing authority and execution non-claims

### Requirement: Verification uses the real composition path

r[agent_harness_runtime.kamacite_live_binding.verification] Verification MUST exercise the real composition and pre-effect code with controlled adapters. It MUST cover accepted and denied cohort, composition, effect, authority, privacy, rollout, and removal cases.

#### Scenario: Packaged bound host rail runs

- GIVEN the exact reviewed cohort and controlled model and tool adapters
- WHEN the packaged host rail runs
- THEN accepted requests MUST reach only admitted adapters and every denied case MUST produce no dependent effect
