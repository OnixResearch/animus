# Design: Add ResearchSpec promotion DAG ordering

## Context

The current ResearchSpec promotion model can tell whether groups are independent, dependent, or blocked, but it does not expose a reusable topological order or a receipt-friendly dependency explanation. A promotion DAG makes the proposed sequence explicit while preserving the existing rule that actual VCS/file mutation stays host-owned.

## Decisions

### 1. Promotion groups become DAG nodes

**Choice:** Represent each promotion group as a safe DAG node with group ID, kept run IDs, touched-file labels, primary metric evidence labels, guardrail status, and host-held diff or patch hashes.

**Rationale:** Group-level nodes align with current promotion grouping and avoid raw patch contents in reusable runtime state.

### 2. Dependency validation is stricter than dependency detection

**Choice:** Validate duplicate group IDs, unknown dependencies, cycles, self-dependencies, overlapping touched files, stale run evidence, failed or unkept runs, unresolved hypothesis decisions, and guarded-file conflicts before topological planning.

**Rationale:** A topological order is only meaningful when every node is independently eligible for promotion.

### 3. Batches explain safe promotion order

**Choice:** Produce deterministic topological batches where independent groups can be promoted separately and dependent groups name the exact safe prerequisites that block them.

**Rationale:** Maintainers need reviewable sequencing without the core applying changes.

### 4. Receipts summarize graph evidence, not patches

**Choice:** Promotion DAG receipts include safe group IDs, dependency edges, counts, run evidence hashes, guarded-file status labels, issue codes, and BLAKE3 receipt identities. Raw diffs, benchmark output, paths, and logs stay outside reusable receipts.

**Rationale:** Promotion evidence must be durable and safe to include in lifecycle closeout summaries.

## Risks / Trade-offs

- Some promotion dependencies are human judgment, not file-level constraints. Allow explicit host-supplied dependency edges, but require safe reasons and reject cycles.
- Overlap detection may be conservative when touched-file summaries are coarse. Block with a reviewable diagnostic rather than silently promoting.
- Hosts may later want live branch integration. That should remain a host adapter on top of these pure plans.
