# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Animus policy families share hardened primitive contracts

r[agent_harness_runtime.nickel_policy_contracts.shared_primitives] Animus MUST provide shared Nickel policy primitives for exact schema literals, integer bounds, closed vocabularies, lowercase BLAKE3 identities, safe repository-relative paths, uniqueness, and deterministic validator diagnostics, while keeping family-specific semantics in their owning policy modules.

#### Scenario: Primitive behavior cannot drift by policy family

- GIVEN two policy families require the same BLAKE3 identity or positive-budget rule
- WHEN their Nickel contracts are evaluated
- THEN both MUST use the shared primitive contract
- AND a malformed value MUST fail with the same stable issue class.

### Requirement: Model-routing source is semantically coherent before export

r[agent_harness_runtime.nickel_policy_contracts.model_routing] The model-routing Nickel contract MUST validate exact schema identity, unique roles, valid and acyclic downgrade references, ordered thresholds and role minima, integer non-negative weights, closed missing-price and budget behaviors, and policy identity shape.

#### Scenario: Downgrade graph is invalid

- GIVEN a routing policy contains a missing downgrade target, self-cycle, multi-role cycle, or role minimum inconsistent with its threshold band
- WHEN Nickel validates the reviewed source
- THEN export MUST fail deterministically before a routing artifact is generated.

### Requirement: Process-job source expresses an admissible bounded profile

r[agent_harness_runtime.nickel_policy_contracts.process_jobs] Process-job Nickel contracts MUST validate unique profile identity, admitted backend, exactly one execution form, path and environment policy, unique identity metadata, and positive resource limits no greater than declared maxima.

#### Scenario: Job profile combines unsafe or ambiguous inputs

- GIVEN a profile supplies conflicting execution forms, a disallowed backend, an unconfined working or writable path, an environment key outside admitted prefixes, duplicate metadata keys, or an excessive resource value
- WHEN Nickel validates the manifest
- THEN export MUST fail without constructing an admitted process-job DTO.

### Requirement: Steel-runtime source has one safe default and bound scripts

r[agent_harness_runtime.nickel_policy_contracts.steel_runtime] Steel-runtime Nickel contracts MUST require unique profiles, exactly one enabled default, positive turn budgets, safe script locators, lowercase BLAKE3 script identities, unique admitted host actions, and closed fallback and receipt-redaction modes.

#### Scenario: Steel profile selection is ambiguous

- GIVEN no enabled profile is default or multiple enabled profiles are default
- WHEN the Steel-runtime policy is evaluated
- THEN Nickel MUST reject the whole policy
- AND runtime code MUST NOT infer a default by array order.

### Requirement: Tool descriptors are constrained by catalog vocabulary

r[agent_harness_runtime.nickel_policy_contracts.tool_catalog] The tool-catalog Nickel contract MUST validate every descriptor against declared runtime-kind, capability, side-effect, approval, redaction, and schema-reference vocabularies and MUST enforce the catalog's dangerous-effect and secret-adjacent cross-field rules.

#### Scenario: Descriptor uses an unsafe combination

- GIVEN a descriptor names an unknown vocabulary value, duplicates a capability, requests a dangerous effect without required approval, or is secret-adjacent without safe redaction
- WHEN the catalog is evaluated
- THEN export MUST fail with deterministic descriptor and invariant context.

### Requirement: Generated policy projections are identity-bound and DTO-compatible

r[agent_harness_runtime.nickel_policy_contracts.projection_freshness] Every generated runtime policy artifact MUST be produced by an explicit checked export that binds the Nickel source, shared prelude, family contract, evaluator/profile, and output identities, and the Rust consumer MUST verify normalized DTO compatibility without invoking Nickel at runtime.

#### Scenario: Contract and Rust consumer drift

- GIVEN a Nickel contract accepts a generated field shape that the Rust DTO rejects, or generated JSON no longer represents the checked source
- WHEN policy freshness and parity validation runs
- THEN validation MUST fail before packaging or runtime admission
- AND it MUST identify whether source, contract, evaluator, projection, or DTO parity drifted.

### Requirement: Policy hardening includes positive and negative fixtures

r[agent_harness_runtime.nickel_policy_contracts.fixtures] Each migrated policy family MUST maintain valid fixtures and negative fixtures for its primitive, vocabulary, identity, reference, bound, uniqueness, path, and cross-field invariants.

#### Scenario: A new invariant lacks failure evidence

- GIVEN a policy change adds or weakens an authoring-time invariant
- WHEN the policy fixture rail runs
- THEN validation MUST require a passing valid case and a deterministic rejecting case for that invariant class.

### Requirement: Nickel policy validation does not claim runtime authority

r[agent_harness_runtime.nickel_policy_contracts.authority_boundary] Nickel MUST own human-authored policy shape and checked projection inputs, while Rust MUST retain runtime admission, external-artifact validation, provider and tool execution, process and filesystem effects, clocks, and receipt authority.

#### Scenario: Contract validation succeeds

- GIVEN a policy source and generated artifact pass all Nickel and parity checks
- WHEN the result is summarized
- THEN the summary MUST state that it does not prove runtime authorization, provider truth, process confinement, tool safety, successful execution, or receipt correctness.
