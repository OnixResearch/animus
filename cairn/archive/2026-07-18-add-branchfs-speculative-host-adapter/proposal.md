## Why

Animus already has portable BranchFS-inspired workspace descriptors, pure branch-graph planning, capability matching, explicit outcomes, and safe receipts. It does not yet materialize those plans through a real speculative filesystem. Best-of-N and independent agent work therefore still depend on host-specific temporary-directory or VCS glue.

The first BranchFS integration should be deliberately narrow: create disposable sibling candidate branches, run work in isolation, abort losers, and materialize a selected winner into a fresh Jujutsu workspace. BranchFS direct commit must remain disabled so the durable repository history and conflict boundary stay owned by Jujutsu.

## What Changes

- Add a non-default host feature for a reviewed BranchFS CLI/FUSE cohort, with explicit platform and capability probes. r[agent_harness_runtime.branchfs_host_adapter.feature_gate] r[agent_harness_runtime.branchfs_host_adapter.preflight]
- Translate existing normalized workspace plans into deterministic abort-only candidate operations in a pure core. r[agent_harness_runtime.branchfs_host_adapter.translation]
- Support root-level sibling candidate creation, execution, loser abort, cleanup, and bounded manifest capture without advertising nested or commit-to-parent support. r[agent_harness_runtime.branchfs_host_adapter.candidates]
- Materialize the selected winner into a newly created Jujutsu workspace after source-freshness and content-identity checks; never invoke BranchFS commit. r[agent_harness_runtime.branchfs_host_adapter.promotion]
- Emit redacted receipts that distinguish BranchFS isolation evidence from Jujutsu durable-state evidence and authorization. r[agent_harness_runtime.branchfs_host_adapter.receipts]
- Add fake-executor tests and opt-in FUSE/Jujutsu behavior tests covering successful and failed isolation, selection, promotion, and cleanup. r[agent_harness_runtime.branchfs_host_adapter.verification]

## Impact

- **Rust**: `animus-std` gains an optional BranchFS/Jujutsu host shell; `animus-adapters` remains the portable contract owner.
- **VCS**: Jujutsu remains the only durable workspace/history mechanism. BranchFS branches are disposable execution state.
- **Platforms**: default and non-FUSE environments remain unaffected; explicit selection fails closed when prerequisites are missing.
- **Compatibility**: the initial adapter advertises no nested-workspace or direct-commit capability, even if upstream BranchFS supports both.
- **Claims**: a passing receipt proves bounded candidate isolation and winner materialization for one run, not merge correctness, repository completion, authorization, FUSE correctness, or durable backup.
