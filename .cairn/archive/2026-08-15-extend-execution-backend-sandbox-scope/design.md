## Context

Sandlock owns portable sandbox descriptors and fail-closed admission. The bubblewrap agent guardrail owns an exact mount profile for an interactive agent. Execution-backend selection picks the weakest admitted backend. None of these owns a per-component sandbox scope.

## Decisions

### 1. One sandbox scope per spawned component

**Choice:** The planner extends its decision to name exactly one sandbox descriptor per spawned extension or component, chosen from the admitted descriptor catalog. A component with no declared sandbox need gets a minimal default scope, never a broad whole-host scope.

**Rationale:** Matched scoping preserves the weakest-admitted policy instead of forcing one profile on every component.

### 2. Reuse current mechanisms

**Choice:** The scope reuses sandlock descriptors and the bubblewrap guardrail contracts. No new sandbox mechanism is introduced; the change is to planner scope and admission, not to enforcement.

**Rationale:** Extend the existing mechanism boundary rather than duplicate enforcement authority.

### 3. Capability request participates

**Choice:** The selected scope intersects the current capability request and fails closed when the descriptor cannot satisfy it. The weakest-admitted rule and the current-host-recheck rule stay in force before effects.

**Rationale:** Authority intersections are the security baseline for any sandboxed spawn.

### 4. Fail closed before effects

**Choice:** Unknown components, missing descriptors, unreachable backends, and unsatisfiable capability requests are rejected before any spawn effect. Receipts are metadata-only with selected backend, sandbox identity, and capability-gate facts.

**Rationale:** Deterministic admission and bounded receipts are the evidence baseline for effect planning.

## Risks / Trade-offs

- Per-component scopes multiply configuration surface. The admitted descriptor catalog keeps the surface declarative and reviewed.
- The planner names scopes but does not enforce them. Live enforcement and host rechecks remain outside this change.
- A minimal default scope can be too narrow for an unexpected component. Failure is explicit, not silent.
