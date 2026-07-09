# Design: Adapt Hermes scoped validation planner contracts

## Context

Animus has process-job profiles, headless validation plans, tool contracts, and continuation policy. A scoped validation planner can sit above those contracts to recommend which validations a host should run for changed artifacts.

## Decisions

### 1. Changed artifacts are host-supplied summaries

**Choice:** The planner consumes changed-artifact summaries with safe labels, kinds, package/crate hints, workspace labels, and content identities. It does not diff VCS, walk the filesystem, or parse manifests directly.

**Rationale:** File discovery and VCS state are host authority.

### 2. Plans reference profile labels, not shell commands

**Choice:** Validation plan items name process-job profile IDs, validation gate IDs, or host action labels plus arguments that pass pure validation. Raw shell commands are rejected unless already encapsulated in a trusted host profile.

**Rationale:** This keeps command authority behind existing process-job contracts.

### 3. Follow-up is continuation data, not auto-fix

**Choice:** A failed validation result can produce a safe follow-up intent with bounded failure summaries. Hosts decide whether to show it, enqueue it, ignore it, or ask for confirmation.

**Rationale:** Automatic fix loops are product policy and can mutate code unexpectedly.

### 4. Planner results are safe evidence

**Choice:** Receipts contain plan IDs, profile labels, artifact counts, status classes, truncation markers, and hashes, never raw logs or file bodies.

**Rationale:** Validation evidence must be reviewable without leaking host data.

## Risks / Trade-offs

- Hosts must do more setup by supplying profiles and artifact summaries. This is necessary to keep Animus reusable.
- Over-scoping can miss regressions. Plans should support conservative fallback labels supplied by host policy.
- Raw command compatibility is tempting. Reject it in reusable logic and require host profile wrapping.
