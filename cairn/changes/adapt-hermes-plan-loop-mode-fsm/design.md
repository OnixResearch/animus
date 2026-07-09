# Design: Adapt Hermes plan/loop mode FSM contracts

## Context

Animus already has prompt profiles, policy profiles, and continuation policy. Plan/loop mode behavior should be an optional finite-state layer that hosts can use to build UI/command workflows without turning prompt text into implicit authority.

## Decisions

### 1. Mode descriptors are host-provided policy data

**Choice:** Modes declare intent, posture, allowed capability ceilings, output markers, extraction rules, stop rules, and continuation behavior as validated descriptors.

**Rationale:** This keeps modes configurable and avoids hard-coded Hermes mode names.

### 2. Plan extraction is explicit and marker-bound

**Choice:** Reusable parsing accepts only configured explicit markers or structured step records. It rejects arbitrary numbered-list capture.

**Rationale:** Accidental plan extraction from prose or code examples is unsafe.

### 3. Selected-step execution is state, not delegation magic

**Choice:** The FSM records selected steps, completion markers, stale step updates, and completion acknowledgements. It does not choose a worker, call tools, or force delegation.

**Rationale:** Execution strategy belongs to the host and existing worker/tool contracts.

### 4. Loop continuation uses continuation policy

**Choice:** Loop conditions and breakout acknowledgements become continuation work items with safe IDs and receipts. Provider rate-limit or transient-error classification is supplied by host adapters.

**Rationale:** Runtime should not probe providers or schedule timers by itself.

## Risks / Trade-offs

- Mode behavior can be confused with permission enforcement. Specs must keep policy guidance separate from runtime capability gates.
- Marker parsing may be too strict for loose workflows. Hosts can define additional structured sources, but they must validate explicitly.
- Loop modes can run forever. Continuation policy must include budgets, cancellation, and acknowledgement freshness.
