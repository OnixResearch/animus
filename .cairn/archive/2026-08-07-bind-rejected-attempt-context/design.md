## Context

Animus creates fresh model contexts and keeps effects host-owned. Cairn's accepted semantic-review specification already supports revision handoff receipts with prior findings and dispositions. The missing seam is a safe, bounded projection from those facts and an optional VCS attempt into the next fresh task context.

Riveter's `jj` rejection workflow is the design reference. Its `prd.toml`, commit subject format, raw diff reuse, and agent-owned pass flag are not compatibility targets.

## Decisions

### Decision: Cairn owns rejection disposition

**Choice:** Require a validated Cairn revision handoff or a policy-declared local review equivalent before Animus labels an attempt rejected. Animus only binds and projects supplied evidence.

**Rationale:** Animus must not create a second review truth or reinterpret free-form commit text as a rejection decision.

### Decision: VCS identity is optional and typed

**Choice:** Model VCS provider, revision identity, parent or workspace identity, content hash, and availability as supplied facts. Provide an optional `jj` host adapter, but keep reusable contracts VCS-neutral.

**Rationale:** The useful behavior is preserved attempt identity, not a mandatory VCS product.

### Decision: Summary before raw diff

**Choice:** The pure planner selects metadata and a bounded redacted summary by default. Raw diff inclusion requires explicit policy, capability, classification, and byte admission.

**Rationale:** Rejected diffs can contain secrets, generated files, large binaries, or irrelevant mistakes.

### Decision: Rejection grants no completion or retry authority

**Choice:** A rejected-attempt descriptor can inform a later run. It cannot mark tasks open or complete, create a new iteration, mutate VCS state, or authorize effects.

**Rationale:** Lifecycle and execution owners must make those decisions separately.

## Risks / Trade-offs

- A short summary can omit the decisive error. The receipt must record omissions and truncation.
- A raw diff can expose sensitive data. Default policy must deny it.
- Rebased or rewritten VCS attempts can become stale. Fresh identity checks must fail closed.
- The Riveter source must be pinned to an exact reviewed revision before implementation.
