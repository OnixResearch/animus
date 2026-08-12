# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: BranchFS host adaptation is optional

r[agent_harness_runtime.branchfs_host_adapter.feature_gate] Animus MUST expose the BranchFS speculative host adapter only through a non-default `animus-std` feature bound to reviewed BranchFS and Jujutsu compatibility cohorts, and default builds MUST retain the portable workspace contract without requiring FUSE, BranchFS, or Jujutsu executables.

#### Scenario: Feature is not selected

- GIVEN Animus is built or run without the BranchFS host feature
- WHEN workspace adapter discovery runs
- THEN the BranchFS host adapter MUST be absent or explicitly unavailable
- AND other workspace adapters and pure workspace planning MUST remain usable.

### Requirement: BranchFS host operations are planned purely

r[agent_harness_runtime.branchfs_host_adapter.translation] Animus MUST derive deterministic mount, root-level candidate-create, execution-handoff, manifest, abort, materialize, retention, and cleanup operations from normalized workspace intent and declared host facts without filesystem, process, environment, clock, FUSE, BranchFS, or VCS effects.

#### Scenario: Sibling candidate plan is admitted

- GIVEN several candidate descriptors share one bound source identity, have no nested parent, and request only advertised create, abort, manifest, and cleanup capabilities
- WHEN pure host-operation planning runs
- THEN it MUST produce one deterministic sibling candidate set and ordered operation identities
- AND every candidate MUST inherit the same source snapshot identity.

#### Scenario: Unsupported graph or operation is requested

- GIVEN a descriptor requests nested parentage, direct commit, commit-to-parent, current-workspace mutation, or another unadvertised capability
- WHEN operation planning runs
- THEN it MUST reject the plan before mounting or invoking a command
- AND it MUST NOT downgrade the request to a weaker isolation mode.

### Requirement: BranchFS and Jujutsu prerequisites fail closed

r[agent_harness_runtime.branchfs_host_adapter.preflight] The host adapter MUST validate the reviewed executable cohort, FUSE readiness, bounded mount/create/read/write/abort/unmount behavior, Jujutsu readiness, and source-workspace identity before any agent process runs.

#### Scenario: Functional preflight succeeds

- GIVEN the selected host can mount BranchFS, isolate and abort a temporary branch, unmount cleanly, and create a Jujutsu workspace from the source identity
- WHEN adapter preflight runs
- THEN it MAY admit candidate execution for the proven cohort
- AND the result MUST bind the exact functional capability set.

#### Scenario: Binary exists but behavior is unavailable

- GIVEN BranchFS is installed but FUSE mounting, branch abort, clean unmount, Jujutsu workspace creation, or source identity inspection fails
- WHEN preflight runs
- THEN the adapter MUST return a typed blocked result before agent execution
- AND it MUST NOT fall back to the operator checkout or an unisolated temporary directory.

### Requirement: Initial BranchFS candidates are abort only

r[agent_harness_runtime.branchfs_host_adapter.candidates] The initial BranchFS host adapter MUST support only root-level disposable sibling candidates, MUST advertise no nested or commit-to-parent capability, and MUST never invoke BranchFS commit for winner or loser handling.

#### Scenario: Loser is discarded

- GIVEN a completed candidate is not selected and has no child branch
- WHEN loser cleanup runs
- THEN the adapter MUST abort only that candidate and record its cleanup outcome
- AND the source, selected candidate, and sibling candidates MUST remain unchanged.

#### Scenario: BranchFS commit reaches the command boundary

- GIVEN an internal or external request attempts to render or execute a BranchFS commit operation
- WHEN command admission or static command-template validation runs
- THEN it MUST reject the operation as forbidden
- AND no commit command or control-file write may be issued.

### Requirement: Winner promotion materializes into fresh Jujutsu state

r[agent_harness_runtime.branchfs_host_adapter.promotion] A selected winner MUST be promoted only by verifying source freshness, creating a fresh Jujutsu workspace at the bound source identity, materializing the winner through capability-scoped filesystem operations that reject backend/VCS control namespaces, escaping links, and unsupported special files, and proving the post-copy bounded manifest identity matches before the BranchFS candidate is aborted.

#### Scenario: Winner materializes successfully

- GIVEN the selection names an admitted candidate, the durable source identity is unchanged, a fresh Jujutsu workspace is created, and copied content matches the winner manifest
- WHEN promotion completes
- THEN the adapter MUST return the new Jujutsu workspace and change identities as durable handoff evidence
- AND it MUST abort the disposable winner only after that evidence is complete.

#### Scenario: Source drifts or copied content differs

- GIVEN the Jujutsu source changed after candidate admission or the fresh workspace manifest differs from the selected BranchFS view
- WHEN promotion validation runs
- THEN promotion MUST fail without mutating the operator's current workspace or invoking BranchFS commit
- AND the winner MUST remain in a bounded cleanup-required or recovery state until explicitly handled.

### Requirement: BranchFS host receipts separate evidence roles

r[agent_harness_runtime.branchfs_host_adapter.receipts] Candidate receipts MUST bind disposable isolation facts, while promotion receipts MUST separately bind selection and fresh Jujutsu workspace facts; both MUST use domain-separated BLAKE3 identities and MUST exclude host paths, raw manifests, file bodies, prompts, credentials, raw command output, daemon logs, and unbounded events.

#### Scenario: Promotion receipt is reviewable

- GIVEN a winner was materialized and all disposable branches and mounts reached known cleanup states
- WHEN safe receipts are projected
- THEN they MUST identify the source, candidate set, selected candidate, adapter cohort, manifests, outcomes, cleanup, and Jujutsu handoff using safe labels and hashes
- AND they MUST state that workspace evidence is not authorization, merge correctness, repository completion, or durable backup.

#### Scenario: Receipt contains raw host material

- GIVEN receipt input includes an absolute mount or workspace path, raw branch manifest, file content, command output, daemon log, prompt, credential, or unbounded event list
- WHEN receipt validation runs
- THEN it MUST reject the receipt without persisting the unsafe material.

### Requirement: BranchFS host behavior has positive and negative evidence

r[agent_harness_runtime.branchfs_host_adapter.verification] The adapter MUST include pure positive and negative tests, a structural no-commit guard, and an opt-in real FUSE/Jujutsu rail covering prerequisite checks, sibling isolation, selection binding, loser abort, winner materialization, source drift, copy mismatch, retention, cleanup, and receipt safety.

#### Scenario: Host-adapter closeout is reviewed

- GIVEN maintainers intend to archive the BranchFS host-adapter change
- WHEN closeout evidence is assembled
- THEN it MUST include default-feature isolation, positive and negative pure tests, no-commit proof, focused adapter checks, and Cairn gates
- AND an unavailable FUSE behavior rail MUST be recorded as blocked with prerequisites and a next-best check rather than reported as passed.
