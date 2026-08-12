## Context

Cairn drain worktrees can contain ignored `.pi/` data that does not appear in Git status. Worktree removal can delete that data. The workflow already names a required guard, but Animus has no implementation.

## Decisions

### Decision: Use a standalone bounded Rust guard

**Choice:** Add `scripts/check-drain-evidence-guard.rs` as a pinned Cargo script with a pure classification core and a thin filesystem and Git shell.

**Rationale:** The guard needs typed option parsing, bounded recursive observation, deterministic results, and fail-closed filesystem handling. A standalone Rust script matches repository automation policy and can run before repository builds.

### Decision: Inspect metadata only

**Choice:** Count bounded `.pi/` entries without reading names or contents. Report only worktree labels, bounded counts, classifications, and verdicts.

**Rationale:** The guard must detect evidence without copying secrets, WIDs, receipts, private paths, or evidence bodies into logs.

### Decision: Require explicit safe worktree classification

**Choice:** Treat a `.git` directory as the persistent primary worktree, a `.git` file as a disposable linked worktree, and all missing, malformed, unreadable, or ambiguous targets as blocking errors. A root `.pi` symlink is safe only when it resolves outside the linked worktree. Nested symlinks count as evidence.

**Rationale:** Successful guard execution must not result from a typo, an unreadable tree, or an unsupported filesystem shape.

## Risks / Trade-offs

- The guard can block cleanup for harmless local `.pi/` files. Operators must move or delete those files explicitly.
- An external `.pi` symlink proves only that worktree removal does not delete the target. It does not prove target durability, confidentiality, or ownership.
