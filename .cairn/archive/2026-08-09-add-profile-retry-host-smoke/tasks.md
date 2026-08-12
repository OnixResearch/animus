# Tasks

## Phase 1: Implementation

- [x] [serial] Add the non-default profile-retry smoke feature, pure composition module, and packaged binary. r[agent_harness_runtime.profile_retry_host_smoke.contract]
- [x] [depends:agent_harness_runtime.profile_retry_host_smoke.contract] Bind an exact loaded profile resolution into one same-target retry invocation before fake dispatch. r[agent_harness_runtime.profile_retry_host_smoke.binding]
- [x] [depends:agent_harness_runtime.profile_retry_host_smoke.binding] Emit metadata-only profile, retry, dispatch, receipt, and non-claim records. r[agent_harness_runtime.profile_retry_host_smoke.receipt]
- [x] [depends:agent_harness_runtime.profile_retry_host_smoke.receipt] Add positive and negative Rust fixtures plus a focused script and Nix check. r[agent_harness_runtime.profile_retry_host_smoke.verification]
- [x] [depends:agent_harness_runtime.profile_retry_host_smoke.verification] Update the README and dogfood roadmap with the new bounded packaged evidence and non-claims. r[agent_harness_runtime.profile_retry_host_smoke.receipt.non_claims]

## Phase 2: Validation

- [x] [serial] Run focused profile, retry, packaged-smoke, Nix, and Cairn checks before sync and archive. r[agent_harness_runtime.profile_retry_host_smoke.closeout]
