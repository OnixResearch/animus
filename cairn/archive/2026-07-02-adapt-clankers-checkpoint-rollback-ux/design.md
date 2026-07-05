# Design: Adapt Clankers checkpoint rollback UX for Animus

## Context

Clankers uses checkpoints to make file mutations reviewable and rollbackable. Animus' provenance roadmap already covers version-control-neutral observations around mutating tools. This change adds the rollback UX and host boundary needed to turn those observations into an explicit recoverability contract.

## Decisions

### 1. Checkpoints are VCS-neutral identities

**Choice:** Define checkpoint DTOs over workspace identity, backend kind, checkpoint ID, safe labels, before/after content hashes, changed-file counts, and optional session/tool attribution.

**Rationale:** Animus should support jj, git, in-memory fixtures, BranchFS-style workspaces, or other adapters without changing runtime provenance logic.

### 2. A host-injected checkpoint service owns mutation

**Choice:** Create, list, inspect, rollback, and garbage-collect operations go through an injected checkpoint service. Reusable crates validate requests and receipts but do not run VCS commands or mutate files.

**Rationale:** Rollback is a side effect. It belongs in the imperative shell or host adapter.

### 3. Pre-mutation checkpoints are policy-driven

**Choice:** Mutating-tool dispatch may require a checkpoint before executing a protected write/edit/patch-like action. Policy decides whether checkpoint failure blocks mutation or is best-effort.

**Rationale:** Some hosts require strict safety; others may run in ephemeral or non-VCS contexts. The behavior must be explicit and testable.

### 4. Rollback requires confirmation and freshness checks

**Choice:** Rollback execution requires an explicit confirmation decision and verifies that the current workspace state still matches the expected post-checkpoint or pre-rollback identity before mutating.

**Rationale:** Rollback must not clobber operator edits made after the checkpoint or after an earlier application.

### 5. Receipts are safe summaries

**Choice:** Checkpoint and rollback receipts record operation, status, backend kind, safe workspace label/hash, checkpoint ID, changed-file counts, confirmation status, before/after hashes, and error class. Raw diffs, file contents, credentials, environment variables, and full path dumps are excluded.

**Rationale:** Operators need enough metadata to choose a rollback target without exposing repository contents in receipts.

## Risks / Trade-offs

- Rollback UX can tempt core crates to know too much about VCS backends. Keep backend-specific details behind adapter labels and opaque IDs.
- Pre-mutation checkpoint failures can block useful work. Policy must name strict versus best-effort behavior explicitly.
- Changed-file counts can be misleading without diff details. Receipts should make clear that they are safe summaries, not full review artifacts.
