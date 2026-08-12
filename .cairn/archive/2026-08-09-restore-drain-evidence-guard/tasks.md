## Phase 1: Contract and implementation

- [x] [serial] Add the bounded metadata-only guard and pure classification core for r[agent_harness_runtime.drain_evidence_guard.teardown].
- [x] [depends:agent_harness_runtime.drain_evidence_guard.fail_closed] Add fail-closed filesystem, symlink, target, and argument handling.

## Phase 2: Verification and lifecycle

- [x] [depends:agent_harness_runtime.drain_evidence_guard.verification] Add positive and negative self-tests plus a focused Nix check.
- [x] [depends:agent_harness_runtime.drain_evidence_guard.evidence] Document safe output, non-claims, and the required cleanup procedure.
- [x] [depends:agent_harness_runtime.drain_evidence_guard.verification] Run focused checks, sync accepted requirements, archive the change, and integrate the verified branch.
