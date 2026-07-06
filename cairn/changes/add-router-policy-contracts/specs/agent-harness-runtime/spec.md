# agent-harness-runtime Change Spec: Add router policy and conformance contracts

## Requirements

### Requirement: Nickel-authored router recipe policy

r[agent_harness_runtime.model_router.policy_contracts] Animus router recipe policy MUST be authored as Nickel contracts defining allowed route families, valid role sets per family, output-contract compatibility per role, bounds floors and ceilings, allowed fallback policies, and backend allowlist labels, and runtime Rust MUST consume checked-in generated policy JSON without executing Nickel at runtime.

#### Scenario: Generated policy stays in sync

r[agent_harness_runtime.model_router.policy_contracts.drift]
- GIVEN a maintained router-recipe Nickel source and generated runtime fixture
- WHEN drift validation runs
- THEN generated data matches the Nickel source
- AND schema-invalid recipes fail before runtime use.

### Requirement: Router conformance-index fixtures

r[agent_harness_runtime.model_router.policy_contracts.fixtures] Animus MUST maintain a router conformance-index fixture set with one positive recipe per family and negative fixtures for unknown family, invalid role combo, out-of-range bound, disallowed fallback, and unknown backend.

#### Scenario: Invalid role combo rejected

r[agent_harness_runtime.model_router.policy_contracts.fixtures.invalid_role]
- GIVEN a recipe fixture pairs roles disallowed for its family by Nickel policy
- WHEN conformance validation runs
- THEN the fixture is rejected with a typed diagnostic
- AND no runtime recipe is produced from it.

### Requirement: Router policy verification

r[agent_harness_runtime.model_router.policy_contracts.verification] The change MUST include focused positive and negative tests for Nickel validation, fixture drift, role-combo rejection, bound-range enforcement, fallback policy, backend allowlists, runtime sync, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers router policy

r[agent_harness_runtime.model_router.policy_contracts.verification.closeout]
- GIVEN maintainers intend to archive the router policy change
- WHEN closeout validation runs
- THEN evidence names focused Nickel, fixture, drift, runtime-sync, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
