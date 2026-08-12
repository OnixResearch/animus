## Phase 1: Feature, cohort, and pure plan

- [x] [serial] Verify upstream license/source provenance, add the non-default `branchfs-host` feature, record reviewed BranchFS/Jujutsu cohort metadata, and prove default/non-FUSE dependency isolation. r[agent_harness_runtime.branchfs_host_adapter.feature_gate]
- [x] [serial] Define pure candidate-set, operation-plan, source-freshness, selection-binding, promotion, retention, and receipt inputs using existing portable workspace DTOs. r[agent_harness_runtime.branchfs_host_adapter.translation] r[agent_harness_runtime.branchfs_host_adapter.promotion]
- [x] [serial] Implement deterministic abort-only operation planning and make BranchFS commit and nested-candidate operations unrepresentable or rejected. r[agent_harness_runtime.branchfs_host_adapter.translation] r[agent_harness_runtime.branchfs_host_adapter.candidates]

## Phase 2: Host preflight and candidate lifecycle

- [x] [serial] Implement pinned executable, FUSE, mount/create/read/write/abort/unmount, Jujutsu, and source-workspace preflight with typed blocked outcomes. r[agent_harness_runtime.branchfs_host_adapter.preflight]
- [x] [serial] Implement the thin BranchFS command shell for one mount, root-level sibling creation, candidate execution handoff, bounded manifest capture, loser abort, retention, and cleanup. r[agent_harness_runtime.branchfs_host_adapter.candidates]
- [x] [serial] Implement source-freshness recheck and winner materialization into a fresh Jujutsu workspace with post-copy manifest equality and no current-workspace mutation. r[agent_harness_runtime.branchfs_host_adapter.promotion]
- [x] [serial] Project separate disposable-candidate and durable-promotion receipts with domain-separated BLAKE3 identities and safe retention diagnostics. r[agent_harness_runtime.branchfs_host_adapter.receipts]

## Phase 3: Positive and negative evidence

- [x] [parallel] Add pure positive tests for sibling plans, deterministic command plans, selection binding, source freshness, matching winner materialization, loser abort, cleanup, and redacted receipts. r[agent_harness_runtime.branchfs_host_adapter.verification]
- [x] [parallel] Add negative tests for disabled/unsupported hosts, missing FUSE or `jj`, nested or commit requests, source drift, unknown winner, backend/VCS control paths, escaping links, special files, copy mismatch, command failure, non-leaf state, failed abort/unmount, retention overflow, and receipt leaks. r[agent_harness_runtime.branchfs_host_adapter.verification]
- [x] [parallel] Add a static and fixture guard proving no admitted operation or rendered command can invoke BranchFS commit. r[agent_harness_runtime.branchfs_host_adapter.candidates] r[agent_harness_runtime.branchfs_host_adapter.verification]
- [x] [serial] Add an opt-in Linux behavior rail that runs sibling candidates, proves isolation, aborts a loser, materializes a winner into a fresh Jujutsu workspace, verifies identities, and cleans the mount. r[agent_harness_runtime.branchfs_host_adapter.preflight] r[agent_harness_runtime.branchfs_host_adapter.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document prerequisites, the abort-only rule, Jujutsu ownership, recovery/retention, supported cohort, operator cleanup, and evidence non-claims. r[agent_harness_runtime.branchfs_host_adapter.feature_gate] r[agent_harness_runtime.branchfs_host_adapter.receipts]
- [x] [serial] Run focused workspace and `animus-std` tests, default/no-default feature checks, command-template guards, behavior smoke when available, formatting, clippy, and receipt checks. r[agent_harness_runtime.branchfs_host_adapter.verification]
- [x] [serial] Run Cairn validation and proposal/design/tasks gates; sync and archive only with positive and negative evidence and explicit blockers for unavailable FUSE behavior rails. r[agent_harness_runtime.branchfs_host_adapter.verification]
