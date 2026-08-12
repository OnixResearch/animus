# Design: Negotiate router capability eligibility

## Context

`entry_allowed` is a three-check predicate: role match, backend allowlist, privacy-vs-local. It ignores whether the selected model can actually produce the role's required output contract or support its tool-call/structured-output needs. The `model_capability_matrix` adaptation already defines pure negotiation over canonical request requirements returning admitted, downgraded, or denied decisions with safe evidence. Connecting the two makes router eligibility a capability decision, not a label match.

## Decisions

### 1. Eligibility is a pure function over role, contract, entry, and capability matrix

**Choice:** `resolve_role_eligibility(role, output_contract, entry, capability)` is pure. It maps the role's required features (output contract, tool-call behavior for Worker/Finalizer, structured-output for Json, privacy posture, budget class) to canonical request requirements and delegates to the existing capability negotiation, returning `Admitted`, `Denied(reason)`, or `Downgraded(to)`.

**Rationale:** Reuses the accepted `model_capability_matrix.negotiation` core rather than duplicating capability logic in the router. Keeps the decision testable without providers.

### 2. Catalog entries carry a capability-matrix identity

**Choice:** Extend `CatalogEntry` (or its draft) with a capability-matrix identity label the router resolves against an injected `ModelCapabilityMatrix`. Entries without a resolvable matrix identity are treated as `Denied(unknown_capability)`.

**Rationale:** The router must not guess capabilities; an explicit identity keeps the lookup fail-closed and connects to the accepted `model_capability_matrix.fail_closed` requirement.

### 3. Downgrades are policy-authorized and recorded

**Choice:** When negotiation returns `Downgraded(to)`, the selected plan records the downgrade with the original and downgraded contract labels and the policy hash that authorized it. Unauthorized downgrades (no policy admission) become `Denied`.

**Rationale:** Silent downgrades can hide capability gaps; recording them keeps receipts auditable and connects to the accepted `model_capability_matrix.negotiation.positive` requirement.

### 4. select_entries uses eligibility, not just labels

**Choice:** Replace the role/backend/privacy predicate in `select_entries` with a call to `resolve_role_eligibility`. A role with no eligible entry returns `SelectionDenial::NoEligibleModel`. The first policy-authorized eligible entry is selected deterministically.

**Rationale:** Makes unsupported required features fail closed at selection time, before any provider call, satisfying the accepted `vllm_semantic_router_recipes.selection` requirement.

## Risks / Trade-offs

- Capability matrix identity on catalog entries couples the router to capability metadata. Mitigate by making the identity optional and failing closed when absent rather than defaulting to "supports everything".
- Downgrade recording can grow the selected plan. Keep the recorded downgrade to a bounded label pair plus policy hash.
- Overlapping with `gate-router-budget-and-fallback` is a risk. This change owns *capability* eligibility; budget/fallback enforcement belongs to that sibling cairn.
