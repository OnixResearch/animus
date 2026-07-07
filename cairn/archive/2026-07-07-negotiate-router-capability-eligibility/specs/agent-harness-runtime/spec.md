# agent-harness-runtime Change Spec: Negotiate router capability eligibility

## Requirements

### Requirement: Capability-aware router role eligibility

r[agent_harness_runtime.model_router.capability_eligibility] Router role eligibility MUST be resolved by a pure function over the role, required `OutputContract`, `CatalogEntry`, and `ModelCapabilityMatrix` that delegates to the accepted capability negotiation and returns `Admitted`, `Denied(reason)`, or `Downgraded(to)`, and entries lacking a resolvable capability-matrix identity MUST be denied as `unknown_capability`.

#### Scenario: Unsupported output contract denied

r[agent_harness_runtime.model_router.capability_eligibility.unsupported_contract]
- GIVEN a `Synthesizer` role requires a `Json` output contract
- AND the candidate model's capability matrix lacks structured-output support
- WHEN `resolve_role_eligibility` runs
- THEN it returns `Denied` with an unsupported-contract reason
- AND no provider call is made for that role.

#### Scenario: Policy-authorized downgrade recorded

r[agent_harness_runtime.model_router.capability_eligibility.downgrade]
- GIVEN a role's required `Artifact` contract is unsupported
- AND host policy authorizes a downgrade to `Text`
- WHEN eligibility runs
- THEN it returns `Downgraded(Text)`
- AND the selected plan records the original and downgraded contract labels and the authorizing policy hash.

### Requirement: Selection uses capability eligibility

r[agent_harness_runtime.model_router.capability_eligibility.selection] `select_entries` MUST use capability eligibility rather than label-only matching, returning `SelectionDenial::NoEligibleModel` when no catalog entry is eligible for a required role, and unsupported required features MUST fail closed before any provider call.

#### Scenario: Missing capability metadata denied

r[agent_harness_runtime.model_router.capability_eligibility.selection.missing_metadata]
- GIVEN a catalog entry has no resolvable capability-matrix identity
- WHEN selection runs for that role
- THEN the entry is denied as `unknown_capability`
- AND if no other eligible entry exists, selection returns `NoEligibleModel`.

### Requirement: Capability eligibility verification

r[agent_harness_runtime.model_router.capability_eligibility.verification] The change MUST include focused positive and negative tests for supported roles, authorized downgrades, unsupported contracts, missing tool-call support, privacy mismatch, unknown capability metadata, denied fallback, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers capability eligibility

r[agent_harness_runtime.model_router.capability_eligibility.verification.closeout]
- GIVEN maintainers intend to archive the capability-eligibility change
- WHEN closeout validation runs
- THEN evidence names focused eligibility, downgrade, selection, fail-closed, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
