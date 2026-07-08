# Design: Add subagent fan-out/fan-in orchestration contracts

Fan-out/fan-in should be a pure planning and aggregation surface. Hosts still own scheduling and concurrency; runtime owns typed plans, safety invariants, and deterministic aggregation.

## Decisions

### 1. Fan-out plan is declarative

**Choice:** A plan declares child-run drafts or admissions, concurrency policy label, shared budget, cancellation policy, aggregation schema, and mutation-arbitration policy.

**Rationale:** Runtime can validate intent while hosts choose concrete schedulers.

### 2. Fan-in aggregation is deterministic

**Choice:** Aggregation is a pure reducer over validated child outcomes, reports, budget usage, and mutation intents.

**Rationale:** Tests can cover partial success, timeouts, malformed outputs, and conflicts without standing up workers.

### 3. Cancellation and budget exhaustion are visible

**Choice:** Budget exhaustion and cancellation produce terminal child outcomes and safe aggregate diagnostics.

**Rationale:** A parent should not mistake missing child output for success.

### 4. Mutations never apply during fan-in

**Choice:** Child mutation intents are preserved, denied, or marked unresolved. Effects require later normal policy and mutation coordination.

**Rationale:** Parallel children should not silently race or bypass parent authority.
