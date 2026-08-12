## Context

The accepted workspace contract intentionally models BranchFS-like behavior without requiring FUSE. The current implementation can validate workspace intent and adapter capabilities but does not mount BranchFS, create a real candidate branch, or hand a selected result to durable version control.

BranchFS can atomically commit a leaf into its parent, but adopting that operation for the first integration would create a second durable mutation path beside Jujutsu and make conflict/recovery behavior harder to audit. The first host adapter therefore treats every BranchFS branch as disposable and promotes a winner by verified materialization into a fresh `jj` workspace.

## Decisions

### 1. Keep BranchFS optional and disposable

**Choice:** Add a non-default `branchfs-host` feature in `animus-std`, bound to a reviewed BranchFS CLI/FUSE cohort. The feature is not enabled by runtime defaults, and selecting it on a host without the required platform support is an explicit blocked result.

**Rationale:** FUSE/macFUSE availability and behavior are host-specific, while most Animus users should retain portable workspace contracts without the dependency.

### 2. Use a pure operation planner and an injected command shell

**Choice:** A pure core maps normalized workspace descriptors and declared adapter capabilities into ordered mount, candidate-create, execute, manifest, abort, materialize, and cleanup operations. A thin shell resolves paths and invokes the reviewed `branchfs` and `jj` commands through an injected executor.

**Rationale:** Planning and failure classification can be tested without FUSE, a daemon, a repository, or process mocks hidden inside business logic.

### 3. Advertise only the initial capability subset

**Choice:** The adapter supports one mount, root-level sibling candidate branches, virtual candidate paths, abort, cleanup, bounded manifests, and winner materialization. It advertises neither nested workspaces nor `commit_to_parent`. Any plan requesting those capabilities is rejected before mounting.

**Rationale:** Best-of-N speculation needs isolated siblings, not the full upstream branch graph. A smaller advertised surface avoids accidental reliance on untested commit semantics.

### 4. Prohibit BranchFS commit structurally

**Choice:** The executable-operation enum contains no BranchFS commit operation, command rendering rejects `commit`, and a static/fixture guard scans the adapter command templates. Candidates are always aborted after successful loser cleanup or winner materialization. A failed winner materialization retains recoverable candidate state until an explicit cleanup decision.

**Rationale:** Durable mutation should have one owner. Merely documenting “do not commit” is weaker than making commit unrepresentable in the admitted plan.

### 5. Preflight behavior, not just binary presence

**Choice:** Host preflight validates the pinned BranchFS executable identity, FUSE device/helper readiness, mount/unmount support, and a bounded temporary mount/create/read/write/abort smoke. It separately validates a supported Jujutsu executable and source workspace. Missing or partial prerequisites stop before agent execution.

**Rationale:** An installed binary does not prove the daemon, FUSE, mount permissions, abort semantics, or Jujutsu destination are usable.

### 6. Freeze source identity for a candidate set

**Choice:** Candidate-set admission binds the source Jujutsu change/commit identity and a bounded source manifest identity before mount. Every sibling is created from that same BranchFS inherited view. Before promotion, the shell verifies that the durable source identity still matches; drift blocks materialization.

**Rationale:** Selecting a winner against a moving durable base can silently mix unrelated edits or invalidate comparison evidence.

### 7. Promote by fresh Jujutsu materialization

**Choice:** After a host-supplied selection decision, the adapter creates a fresh Jujutsu workspace at the bound source identity, copies the selected BranchFS view through capability-scoped filesystem operations, recomputes the bounded manifest, and accepts promotion only when the copied content identity matches. The copy policy rejects backend/VCS control namespaces such as `.branchfs_ctl`, virtual `@branch` entries, `.jj`, and `.git`, plus escaping links and unsupported special files. It does not write into the operator's current workspace.

**Rationale:** A fresh workspace provides a durable, inspectable handoff without asking BranchFS to mutate the base or current working copy.

### 8. Keep selection outside the adapter

**Choice:** The adapter receives a typed selected candidate identity from the existing orchestration/portfolio layer. It validates that the candidate belongs to the admitted set but does not score outputs or decide which candidate won.

**Rationale:** Filesystem isolation should not absorb model evaluation or portfolio policy.

### 9. Separate receipts by authority role

**Choice:** Candidate receipts bind source, descriptor, adapter cohort, candidate, operation, manifest, outcome, and cleanup identities. Promotion receipts additionally bind the selection decision and new Jujutsu workspace/change identity. Host paths, branch manifests, file bodies, command output, prompts, and credentials remain excluded.

**Rationale:** Reviewers need to distinguish disposable isolation from durable VCS materialization and from authorization.

## Risks / Trade-offs

- Copying the winner into a fresh Jujutsu workspace costs more than BranchFS commit, but it preserves a single durable mutation owner.
- FUSE behavior and cache invalidation can vary across kernels and macFUSE releases. The first behavior rail should be Linux-focused; other platforms remain unsupported until separately evidenced.
- Retaining a failed winner candidate aids recovery but consumes host state. Retention must be bounded and surfaced as cleanup-required evidence.
- Manifest equality proves the bounded copied view, not semantic merge correctness or absence of concurrent external mutation outside the bound roots.

## Non-Goals

- Replacing Jujutsu with BranchFS or using BranchFS as durable history.
- Calling BranchFS commit, supporting nested candidates, or mutating the operator's current workspace in the first adapter.
- General FUSE abstraction, remote workspaces, cross-host mounts, or arbitrary filesystem snapshot support.
- Selecting the winning candidate, authorizing its actions, or claiming repository completion.
