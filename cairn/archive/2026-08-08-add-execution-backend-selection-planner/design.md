## Context

Animus can describe process jobs, Wasm-compatible reusable cores, host bindings, sandbox profiles, and runtime authority. Consumers still choose execution routes through product-specific conditionals.

The selected reference is agentOS's distinction between lightweight mediated execution and escalation to a full sandbox for workloads such as native compilation, browsers, or stronger operating-system compatibility. Its VM implementation and security claims are outside this change.

## Decisions

### Decision: Select the weakest admitted backend

**Choice:** The pure planner selects the least-authoritative backend class that satisfies all declared requirements and policy. Selection order is policy data, not an implicit numeric ranking in code.

**Rationale:** A broader backend increases host exposure, cost, and trusted computing base. The planner must not escalate merely because a preferred backend is unavailable.

### Decision: Keep requirements explicit

**Choice:** Workloads declare required execution formats, filesystem semantics, network posture, process model, terminal needs, browser or device needs, resource limits, persistence shape, and isolation claims. Unknown requirements deny.

**Rationale:** Command names and agent labels are not reliable backend requirements.

### Decision: Separate plan from current enforcement

**Choice:** The plan binds reviewed identities. A host must recheck current backend availability, permission policy, authority, resources, application-host binding, and sandbox enforcement before effects.

**Rationale:** A valid historical plan cannot establish current host enforcement.

### Decision: No hidden fallback

**Choice:** A failed or unavailable selected backend returns a bounded denial or reselection request. It never silently moves to a broader or weaker backend.

**Rationale:** Hidden fallback invalidates the original authority and isolation review.

## Risks / Trade-offs

- A generic requirement vocabulary can become too broad. Keep backend-specific details in adapters.
- The weakest admitted backend can differ by deployment policy. Policy must define selection order explicitly.
- Consumers can misuse plan receipts as sandbox evidence. Receipts must include strong non-claims.
- Exact upstream and consumer revisions must be pinned before implementation.
