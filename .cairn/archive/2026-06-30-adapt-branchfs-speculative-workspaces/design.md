# Design: BranchFS-inspired speculative workspaces

## Context

BranchFS provides file-level copy-on-write branches over a base directory, with nested branches, commit-to-parent, abort, and virtual `@branch` paths that allow multiple agents to work against isolated views through one mount. It is a strong reference for parallel agent speculation because workspace creation and abort are cheap, and successful work can be promoted through a parent chain.

Animus needs those semantics as portable runtime intent rather than a direct filesystem API. Reusable crates should reason about workspace descriptors, parentage, conflict policy, and receipt-safe evidence without mounting filesystems or depending on Linux/macOS FUSE support. The imperative shell should own platform capability probing, daemon lifecycle, mount/unmount operations, storage cleanup, filesystem copying, and host mutation.

## Decisions

### 1. BranchFS is a bounded workspace reference

**Choice:** Use BranchFS as a reference for copy-on-write speculative workspaces, nested branch parentage, leaf-only commit/abort discipline, virtual branch addressing, and cache/mmap invalidation awareness. Do not vendor BranchFS, require it as a default runtime dependency, or copy its CLI/storage layout as the Animus contract.

**Rationale:** The semantics are valuable, but the implementation depends on platform FUSE support and a daemon. Animus needs an adapter boundary that can also support Jujutsu, Git worktrees, tempdir snapshots, or future filesystem backends.

### 2. Workspace descriptors are VCS-neutral runtime intent

**Choice:** Define speculative workspace descriptors in terms of source identity, workspace identity, parent workspace, writable roots, ignored/local-file inclusion policy, commit target, abort policy, conflict policy, cleanup policy, and required adapter capabilities.

**Rationale:** These fields express what the runtime needs without exposing BranchFS paths, ioctls, storage tombstones, or mount commands to core crates.

### 3. Pure planning precedes platform mutation

**Choice:** Descriptor parsing, normalization, branch graph validation, conflict policy checks, and receipt-safe projection should be pure deterministic logic over typed inputs. The std shell consumes a normalized plan and performs backend-specific setup, commit, abort, cleanup, and error handling.

**Rationale:** Functional-core tests can cover valid and invalid workspace plans without requiring FUSE. Platform smoke tests can remain optional and focused.

### 4. Required workspace isolation fails closed

**Choice:** If a capability request or policy requires isolated workspace execution, dispatch must fail before model/tool/process execution when no selected adapter can enforce the requested workspace semantics.

**Rationale:** Isolation requirements are only useful if unsupported or partial implementations cannot silently fall back to the host checkout.

### 5. Commit and abort are explicit adapter outcomes

**Choice:** Workspace adapters should report typed outcomes for created, committed, aborted, conflict, unavailable, failed-cleanup, and ambiguous states. Commit should be leaf-scoped when the adapter uses nested branches, and abort should preserve parent and sibling workspaces.

**Rationale:** BranchFS highlights the importance of leaf-only commit/abort and zero-cost discard. Animus receipts need the same state clarity even when a backend is not BranchFS.

### 6. Receipts carry redacted workspace evidence

**Choice:** Workspace receipts should record descriptor hash, adapter labels, source identity hash, workspace identity, parent identity, commit/abort outcome, conflict status, cleanup status, bounded manifest counts or BLAKE3 hashes, and receipt hash. They must omit raw prompts, credentials, absolute path dumps, raw file contents, raw process output, and unbounded file inventories.

**Rationale:** Workspace evidence is needed for review and handoff, but raw filesystem details can leak secrets and local paths.

## Risks / Trade-offs

- FUSE availability varies across Linux, containers, CI, and macOS. Capability probing and unsupported-backend diagnostics must be deterministic and fail closed when isolation is required.
- BranchFS commit-to-parent semantics differ from VCS merge semantics. Animus should model commit outcomes and conflict policy separately from source-control commits.
- Virtual `@branch` paths are ergonomic but backend-specific. Keep them adapter-local and expose only stable workspace identities to core.
- Workspace manifests can leak filenames. Receipts should prefer counts, bounded labels, and BLAKE3 hashes unless policy explicitly permits more detail.
- Combining sandboxing and workspace isolation can create confusing responsibility boundaries. Authorization, sandbox enforcement, and workspace isolation should each emit separate outcome evidence.
