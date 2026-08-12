## Why

Animus has a pure BLAKE3 hashed-edit plan. It does not yet provide a concrete capability-relative host that applies the plan to one file.

The current contract also does not require proof that edited lines were visible under the supplied revision. This permits valid hashes over lines the caller never observed.

Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` provides ISC-licensed revision and visibility scenarios. Animus will adapt them behind Cap Root, capability-relative I/O, and explicit publication states.

## What Changes

- Add a separate non-default workspace-edit host with a pure admission core and thin filesystem shell. r[agent_harness_runtime.hashed_edit_host.boundary]
- Extend snapshots with visible tagged spans, line-ending state, final-newline state, and exact capability-root identity. r[agent_harness_runtime.hashed_edit_host.snapshot]
- Admit mutations only within lines or insertion boundaries visible under the exact supplied revision. r[agent_harness_runtime.hashed_edit_host.visibility]
- Re-read and revalidate exact content immediately before one publication attempt. r[agent_harness_runtime.hashed_edit_host.freshness]
- Serialize Animus-owned writers and report explicit pre-publication, published, durability-pending, and durability-complete outcomes. r[agent_harness_runtime.hashed_edit_host.publication]
- Add metadata-only receipts and paired positive and negative filesystem fixtures. r[agent_harness_runtime.hashed_edit_host.evidence] r[agent_harness_runtime.hashed_edit_host.verification]

## Success Criteria

- Stale, hidden, overlapping, malformed, or over-budget edits fail before publication.
- Accepted edits preserve selected line endings and final-newline state.
- The host uses capability-relative authority and rejects absolute or escaped paths.
- A failure after publication reports uncertainty or incomplete durability without claiming rollback.
- Receipts contain hashes and bounded metadata, not file bodies or private paths.

## False Completion

The change is incomplete if it wraps the existing validator with unrestricted path-based file writes.

A lock for Animus writers does not exclude another process. The host must retain that race as an explicit non-claim.

## Impact

- **New host surface**: separate non-default workspace-edit crate or authority-scoped std block.
- **Reusable logic**: snapshot admission, visibility, freshness, publication planning, and receipts.
- **Host effects**: capability-relative reads, synchronization, staging, rename, and durability operations.
- **Testing**: positive edit, stale revision, hidden span, path escape, concurrent writer, publication failure, and leak fixtures.

## Out of Scope

- Multi-file transactions, arbitrary binary edits, VCS commits, merge conflict resolution, or cross-process compare-and-swap guarantees.
- Sandbox, authorization, store correctness, release readiness, or semantic correctness claims.

## Affected Specs

- `agent-harness-runtime`: concrete hashed-edit host boundary, snapshot, visibility, freshness, publication, evidence, and verification.
