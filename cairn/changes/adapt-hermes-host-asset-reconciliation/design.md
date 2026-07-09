# Design: Adapt Hermes host asset reconciliation contracts

## Context

Animus already validates skill manifests, plugin manifests, and host-provided resource descriptors. A host asset reconciliation layer can compare versioned manifests with installed observations to report drift and plan safe actions, without owning discovery or mutation.

## Decisions

### 1. Reconciliation operates over host-supplied observations

**Choice:** Hosts supply manifest entries and installed observations with safe store labels, asset IDs, content identities, enablement state, trust state, and provenance. Reusable logic classifies drift.

**Rationale:** Filesystem roots, symlinks, and installed state are host-specific.

### 2. Drift classes are portable

**Choice:** Use generic classes such as `installed-matches-manifest`, `installed-disabled`, `installed-drifted`, `host-only`, `manifest-only`, `identity-mismatch`, and `unsupported-store` rather than Hermes/Pi-specific names.

**Rationale:** The pattern should work for any host shell.

### 3. Plans are dry-run by default

**Choice:** Reconciliation produces a plan with safe actions and diagnostics. Mutation requires a separate host-owned request and capability admission.

**Rationale:** Asset installation changes prompt/tool behavior and must be explicit.

### 4. Paths remain labels or hashes

**Choice:** Receipts and diagnostics use safe store labels, relative labels when host-approved, and BLAKE3 content identities. Full private paths are rejected.

**Rationale:** Asset reports are likely durable evidence and must be safe.

## Risks / Trade-offs

- Store-neutral classes may lose host-specific nuance. Hosts can attach safe extra labels.
- Without automatic mutation, reconciliation is advisory. That is correct for SDK boundaries.
- Installed observations can be stale. Include freshness and observation identity.
