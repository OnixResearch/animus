# Tasks

## Phase 1: Implementation

- [x] [serial] Add the non-default multi-role smoke feature, coordinator module, and packaged binary. r[agent_harness_runtime.multi_role_host_smoke.contract]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.contract] Load exact role and model-profile projections and activate four separately authorized roles. r[agent_harness_runtime.multi_role_host_smoke.activation]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.activation] Enforce capability intersections and explicit reviewer, verifier, and judge separation. r[agent_harness_runtime.multi_role_host_smoke.separation]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.separation] Admit candidate, finding, verification, and veto outputs through separate scoped grants. r[agent_harness_runtime.multi_role_host_smoke.outputs]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.outputs] Reject reviewer mutation and prove role-output completion non-equivalence through the separate completion gate. r[agent_harness_runtime.multi_role_host_smoke.completion_boundary]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.completion_boundary] Emit metadata-only role records and a source-bound current-head focused receipt. r[agent_harness_runtime.multi_role_host_smoke.receipt]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.receipt] Add positive and negative Rust fixtures plus a focused script and Nix check. r[agent_harness_runtime.multi_role_host_smoke.verification]
- [x] [depends:agent_harness_runtime.multi_role_host_smoke.verification] Update the README and dogfood roadmap with bounded claims and non-claims. r[agent_harness_runtime.multi_role_host_smoke.receipt.non_claims]

## Phase 2: Validation

- [x] [serial] Run focused role, completion, packaged-smoke, Nix, dogfood, and Cairn checks before sync and archive. r[agent_harness_runtime.multi_role_host_smoke.closeout]
