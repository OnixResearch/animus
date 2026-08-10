# Tasks

## Phase 1: Implementation

- [x] [serial] Add the non-default authoritative-completion smoke feature, coordinator module, and packaged binary. r[agent_harness_runtime.authoritative_completion_host_smoke.contract]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.contract] Add a typed four-role advisory seam with explicit judge select and veto decisions. r[agent_harness_runtime.authoritative_completion_host_smoke.role_boundary]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.role_boundary] Load exact role, model-profile, and check-registry projections without ambient discovery. r[agent_harness_runtime.authoritative_completion_host_smoke.loading]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.loading] Compose explicit changed-artifact, subject-hash, check-execution, persistence, and Cairn-validation services. r[agent_harness_runtime.authoritative_completion_host_smoke.host_services]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.host_services] Admit completion only through exact pass-only results and a matching Cairn summary. r[agent_harness_runtime.authoritative_completion_host_smoke.completion]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.completion] Add negative fixtures for veto, self-attestation, dirty state, missing evidence, stale identity, and malformed configuration. r[agent_harness_runtime.authoritative_completion_host_smoke.failure]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.failure] Emit metadata-only output and a source-bound current-head receipt. r[agent_harness_runtime.authoritative_completion_host_smoke.receipt]
- [x] [depends:agent_harness_runtime.authoritative_completion_host_smoke.receipt] Add a focused script, Nix check, README entry, and dogfood-roadmap rung. r[agent_harness_runtime.authoritative_completion_host_smoke.verification]

## Phase 2: Validation

- [x] [serial] Run focused role, completion, packaged-smoke, clippy, Nix, dogfood, workspace, and Cairn checks before sync and archive. r[agent_harness_runtime.authoritative_completion_host_smoke.closeout]
