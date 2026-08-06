# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Runtime authority units seal before dependent effects

r[agent_harness_runtime.runtime_authority_seal.phase] Animus MUST represent one bounded dispatch as a typed authority unit that moves from preparation to a committed seal before its dependent effect. The unit MUST use explicit terminal states and MUST NOT infer a seal from profile equality, process state, or prior execution.

#### Scenario: Valid unit seals before dispatch

r[agent_harness_runtime.runtime_authority_seal.phase.valid]
- GIVEN one dispatch has an admitted session profile, durable binding, current enforcement observation, operation set, adapter cohort, resource profile, and runtime-view observation
- WHEN the pure planner validates the unit and the host commits its seal receipt
- THEN the unit MUST enter the sealed state with one deterministic identity
- AND the dependent effect MAY start only after that commit succeeds.

#### Scenario: Dispatch is requested before sealing

r[agent_harness_runtime.runtime_authority_seal.phase.early_effect]
- GIVEN a unit is absent, preparing, failed, cancelled, completed, or lacks a committed seal
- WHEN a dependent provider, tool, summarizer, or equivalent effect is requested
- THEN Animus MUST deny dispatch with a typed unit-state issue
- AND it MUST NOT infer success from a matching session profile or available adapter.

### Requirement: A sealed unit binds the exact observed runtime view

r[agent_harness_runtime.runtime_authority_seal.runtime_view] The seal MUST bind safe identities for the current session profile, durable binding, realized enforcement observation, admitted operations and resources, selected adapter cohort, resource profile, and every required observable runtime-view class. Required missing, unexpected, duplicate, stale, unknown, or unsupported observations MUST block the seal.

#### Scenario: Declared and observed views match

r[agent_harness_runtime.runtime_authority_seal.runtime_view.exact]
- GIVEN the host supplies one current observation for every required view class
- AND the observed identities match the declared dispatch inputs exactly
- WHEN runtime-view comparison runs
- THEN Animus MUST produce one canonical domain-separated BLAKE3 unit identity
- AND input order, checkout path, clock, randomness, locale, secrets, and payloads MUST NOT change that identity.

#### Scenario: Runtime view is incomplete or wider

r[agent_harness_runtime.runtime_authority_seal.runtime_view.mismatch]
- GIVEN a required view entry is missing, extra, duplicated, stale, unknown, unsupported, or bound to another profile, adapter, resource, or operation
- WHEN seal admission runs
- THEN Animus MUST reject the unit before receipt commit or effect dispatch
- AND diagnostics MUST identify only safe classes and expected or observed identities.

### Requirement: Sealed unit authority is monotonic

r[agent_harness_runtime.runtime_authority_seal.monotonic] After a unit is sealed, Animus MUST reject capability addition, profile rebind, adapter replacement, resource expansion, ambient discovery, or fallback for that unit. A current recheck MAY narrow or deny execution, but it MUST NOT widen the sealed set.

#### Scenario: Post-seal widening is attempted

r[agent_harness_runtime.runtime_authority_seal.monotonic.denied]
- GIVEN a unit is sealed or running
- WHEN a caller adds a grant, operation, adapter, resource, profile component, fallback, or newly discovered authority to that unit
- THEN the transition MUST fail with a typed post-seal-widening issue
- AND the original unit identity and committed receipt MUST remain unchanged.

#### Scenario: Valid new authority uses a new unit

r[agent_harness_runtime.runtime_authority_seal.monotonic.new_unit]
- GIVEN policy admits a later grant, profile binding, adapter cohort, or resource set for the session
- WHEN a later dispatch needs that authority
- THEN the host MUST prepare and seal a new unit with a new identity
- AND it MUST finish or cancel the prior unit according to policy rather than mutate it.

### Requirement: Seal receipts are safe and bounded

r[agent_harness_runtime.runtime_authority_seal.receipt] Animus MUST emit a metadata-only seal receipt with safe unit, session, profile, binding, enforcement, operation, adapter, resource-view, state, and BLAKE3 receipt identities. The receipt MUST exclude raw grants, descriptors, paths, credentials, prompts, provider payloads, tool bodies, model output, and environment values.

#### Scenario: Passing seal emits safe evidence

r[agent_harness_runtime.runtime_authority_seal.receipt.safe]
- GIVEN a unit seals successfully
- WHEN the host emits its receipt
- THEN the receipt MUST record commit-before-effect ordering and the exact safe identity cohort
- AND it MUST state the bounded observation scope and required non-claims.

#### Scenario: Receipt claims operating-system isolation

r[agent_harness_runtime.runtime_authority_seal.receipt.overclaim]
- GIVEN a receipt or explanation claims complete descriptor closure, operating-system namespace confinement, hidden-authority absence, sandbox correctness, or host integrity without separate evidence
- WHEN receipt validation runs
- THEN Animus MUST reject the artifact with a stable overclaim issue.

### Requirement: Runtime authority seal claims remain bounded

r[agent_harness_runtime.runtime_authority_seal.boundary] The authority-unit contract MUST preserve existing authorization, sandbox, capability, adapter, provider, tool, credential, cancellation, timeout, and cleanup owners. It MUST NOT claim that a passing metadata seal proves operating-system isolation, effect correctness, host trust, or release eligibility.

#### Scenario: Seal is presented as authorization

r[agent_harness_runtime.runtime_authority_seal.boundary.authorization]
- GIVEN a unit has a matching seal receipt
- WHEN current authorization or enforcement fails
- THEN the dependent effect MUST remain denied by the existing owner
- AND the seal MUST NOT override or replace that denial.

### Requirement: Runtime authority sealing has positive and negative verification

r[agent_harness_runtime.runtime_authority_seal.verification] The change MUST include positive and negative core, property, shell, receipt, and boundary tests for seal ordering, exact view matching, deterministic identity, monotonic authority, new-unit reauthorization, failure handling, safe evidence, and non-claims.

#### Scenario: Focused verification closes the change

r[agent_harness_runtime.runtime_authority_seal.verification.closeout]
- GIVEN maintainers intend to sync and archive runtime authority sealing
- WHEN focused and repository checks run
- THEN positive and negative tests, purity checks, Nickel freshness, documentation, Cairn validation, and Cairn gates MUST pass
- AND unavailable optional checks MUST retain an exact blocker and next-best check.
