# agent-harness-runtime Change Spec: BranchFS-inspired speculative workspaces

## Requirements

### Requirement: Bounded BranchFS adaptation scope

r[agent_harness_runtime.branchfs_workspace.scope] Animus MUST treat BranchFS as a bounded reference for speculative workspace patterns, including copy-on-write branch creation, nested parentage, leaf-only commit and abort discipline, commit-to-parent promotion, zero-cost discard intent, virtual branch addressing, and mmap or cache invalidation awareness, and MUST NOT vendor BranchFS, require FUSE or macFUSE for all users, copy the BranchFS CLI or storage layout as the Animus contract, or expose BranchFS-specific ioctls, tombstones, daemon paths, or mount paths from `animus-core`.

#### Scenario: Scope review rejects backend coupling

r[agent_harness_runtime.branchfs_workspace.scope.review]
- GIVEN a proposed implementation imports BranchFS into reusable core crates, shells out to `branchfs` from pure logic, stores BranchFS mount paths in core DTOs, or requires FUSE for non-workspace runtime paths
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until BranchFS behavior remains adapter-local reference evidence
- AND portable Animus workspace descriptors remain independent of BranchFS implementation details.

### Requirement: VCS-neutral speculative workspace descriptors

r[agent_harness_runtime.branchfs_workspace.descriptor] Animus speculative workspace descriptors MUST be typed portable runtime intent covering source identity, workspace identity, parent workspace identity, writable roots, ignored or local-file inclusion policy, commit target, abort policy, conflict policy, cleanup policy, and required adapter capability classes.

#### Scenario: Descriptor captures workspace intent without FUSE details

r[agent_harness_runtime.branchfs_workspace.descriptor.intent]
- GIVEN a runtime action requests an isolated speculative workspace
- WHEN Animus validates the workspace descriptor
- THEN the descriptor names portable workspace intent rather than BranchFS CLI flags, FUSE mount paths, daemon socket paths, storage directories, ioctls, or tombstone files
- AND missing, duplicate, malformed, or unknown required fields fail validation before any workspace backend is invoked.

### Requirement: Pure workspace planning

r[agent_harness_runtime.branchfs_workspace.planning] Workspace descriptor parsing, normalization, branch graph validation, conflict policy validation, unsupported-capability diagnostics, and receipt-safe projection MUST be pure deterministic logic over typed inputs and MUST NOT read the filesystem, inspect the environment, spawn processes, consult clocks, probe kernels, inspect VCS state, start daemons, mount filesystems, or mutate host state.

#### Scenario: Invalid branch graph fails before platform setup

r[agent_harness_runtime.branchfs_workspace.planning.invalid_graph]
- GIVEN a workspace descriptor graph contains a parent cycle, duplicate workspace identity, missing parent identity, non-leaf commit request, or incompatible commit and abort policy
- WHEN pure workspace planning runs
- THEN it returns typed diagnostics naming the invalid graph or policy condition
- AND no FUSE operation, VCS operation, process executor, filesystem operation, daemon request, or platform probe is invoked.

### Requirement: Workspace adapter capability model

r[agent_harness_runtime.branchfs_workspace.adapter] Animus workspace adapters SHOULD expose an explicit capability model for translating normalized workspace plans into enforceable backend behavior, including labels for BranchFS-like copy-on-write branches, Jujutsu or Git worktrees, temporary snapshot directories, nested workspaces, commit-to-parent support, abort support, conflict detection, cleanup guarantees, and unsupported platform states.

#### Scenario: Backend reports unsupported nested commit

r[agent_harness_runtime.branchfs_workspace.adapter.unsupported]
- GIVEN a normalized workspace plan requires nested workspaces and commit-to-parent behavior
- AND the selected adapter can create a temporary snapshot but cannot represent nested parentage or commit-to-parent semantics
- WHEN adapter capability matching runs
- THEN it reports unsupported capability diagnostics for the missing nested and commit behavior
- AND the runtime does not silently downgrade the plan to a weaker workspace mode.

### Requirement: Required workspace isolation fails closed

r[agent_harness_runtime.branchfs_workspace.fail_closed] Runtime dispatch MUST fail closed before model, tool, or process execution when required workspace isolation is malformed, denied by policy, unsupported by the selected adapter, unavailable on the current platform, or only partially enforceable, and MUST NOT treat workspace isolation as authorization proof.

#### Scenario: Required workspace does not fall back to host checkout

r[agent_harness_runtime.branchfs_workspace.fail_closed.no_host_fallback]
- GIVEN a capability request is authorized by its normal contract and grant checks
- AND its local caveat requires speculative workspace isolation
- AND the selected adapter cannot start, mount, snapshot, clean up, or enforce the requested writable-root and commit/abort semantics
- WHEN dispatch evaluates the action
- THEN dispatch is denied before host execution with a typed workspace-unavailable or partial-enforcement issue
- AND the receipt records workspace isolation failure separately from authorization success.

### Requirement: Explicit commit and abort outcomes

r[agent_harness_runtime.branchfs_workspace.commit_abort] Workspace adapters MUST report explicit outcomes for created, committed, aborted, conflict, unavailable, failed-cleanup, and ambiguous states, and nested branch backends MUST preserve leaf-scoped commit and abort behavior so commit promotes only to the immediate parent and abort discards only the selected leaf workspace while preserving parent and sibling workspaces.

#### Scenario: Leaf commit promotes without affecting siblings

r[agent_harness_runtime.branchfs_workspace.commit_abort.leaf_commit]
- GIVEN a parent workspace has two child workspaces and one child is selected for commit
- WHEN the adapter commits the selected leaf workspace
- THEN the committed changes are promoted only to the selected parent target
- AND sibling workspaces remain isolated unless conflict policy explicitly blocks or invalidates them.

#### Scenario: Abort discards selected leaf only

r[agent_harness_runtime.branchfs_workspace.commit_abort.leaf_abort]
- GIVEN a nested workspace tree has a leaf workspace with uncommitted changes
- WHEN the adapter aborts that leaf workspace
- THEN only the selected leaf workspace is discarded
- AND the parent workspace, sibling workspaces, and base source identity remain available or are reported with explicit adapter diagnostics.

### Requirement: Redacted workspace receipts

r[agent_harness_runtime.branchfs_workspace.receipts] Workspace receipts MUST carry safe metadata only, including workspace status, issue code, normalized descriptor hash, source identity hash, adapter capability labels, parent and workspace identity labels, commit or abort outcome, conflict status, cleanup status, bounded manifest counts or BLAKE3 hashes, and BLAKE3 receipt hash, and MUST NOT include raw prompts, credentials, absolute path dumps, raw file contents, raw process output, raw branch manifests, raw network payloads, or unbounded event streams.

#### Scenario: Successful commit receipt is reviewable

r[agent_harness_runtime.branchfs_workspace.receipts.success]
- GIVEN an isolated workspace is created, used, committed to its parent, and cleaned up successfully
- WHEN the workspace receipt is emitted
- THEN it records descriptor, source, manifest, commit, cleanup, and receipt hashes plus bounded adapter capability labels and outcome status
- AND sensitive path, prompt, credential, file-content, process-output, branch-manifest, and payload material is omitted or represented only by approved hashes and redaction markers.

#### Scenario: Receipt validation rejects workspace leaks

r[agent_harness_runtime.branchfs_workspace.receipts.rejects_leaks]
- GIVEN a workspace receipt fixture contains raw prompts, credential-looking values, absolute path dumps, raw file contents, raw process output, raw branch manifests, raw network payloads, or an unbounded event stream
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the receipt cannot be used as review or release evidence.

### Requirement: Focused verification for speculative workspaces

r[agent_harness_runtime.branchfs_workspace.verification] The BranchFS-inspired adaptation MUST include focused positive and negative tests for descriptor normalization, branch graph validation, adapter capability translation, fail-closed dispatch, commit and abort outcomes, cleanup handling, and receipt redaction.

#### Scenario: Positive and negative workspace fixtures gate closeout

r[agent_harness_runtime.branchfs_workspace.verification.fixtures]
- GIVEN the BranchFS-inspired speculative workspace adaptation is ready for review
- WHEN focused verification runs
- THEN positive fixtures show valid descriptor normalization, nested workspace planning, successful create, commit, abort, cleanup, and redacted receipts
- AND negative fixtures show malformed descriptors, invalid branch graphs, unsupported FUSE or backend environments, partial enforcement, commit conflicts, failed cleanup, ambiguous state, and receipt leak attempts fail closed.

r[agent_harness_runtime.branchfs_workspace.closeout] Closeout evidence MUST include successful focused workspace descriptor, adapter, commit/abort, cleanup, receipt, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates before this adaptation is archived.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.branchfs_workspace.closeout.evidence]
- GIVEN maintainers intend to archive the BranchFS-inspired speculative workspace adaptation
- WHEN they collect closeout evidence
- THEN the evidence names every successful focused rail, workspace check, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.
