# Tasks

- [x] [serial] Add a generic packaged completion entrypoint that accepts injected check execution and Cairn-summary services. r[agent_harness_runtime.real_check_completion_host_smoke.composition]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.composition] Add a separate AGPL authority crate with no default or reusable-runtime authority increase. r[agent_harness_runtime.real_check_completion_host_smoke.isolation]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.isolation] Execute only exact reviewed registry argv through bounded-exec with explicit root, environment, limits, and process-group teardown. r[agent_harness_runtime.real_check_completion_host_smoke.execution]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.execution] Build metadata-only observations from real child facts and reject command drift, unsupported checks, spawn failure, timeout, nonzero exit, and output overflow. r[agent_harness_runtime.real_check_completion_host_smoke.failure]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.failure] Validate the same selected rows and observations through the pinned Cairn core before completion. r[agent_harness_runtime.real_check_completion_host_smoke.cairn]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.cairn] Add positive and negative fixture-child tests plus bounded output leak checks. r[agent_harness_runtime.real_check_completion_host_smoke.verification]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.verification] Add a focused fixture rail, clean-current-head live mode, Nix check, and BLAKE3-bound receipts. r[agent_harness_runtime.real_check_completion_host_smoke.receipt]
- [x] [depends:agent_harness_runtime.real_check_completion_host_smoke.receipt] Update README and dogfood readiness documentation with the new rung and non-claims. r[agent_harness_runtime.real_check_completion_host_smoke.docs]
- [x] [serial] Run focused, boundary, workspace, Tiger Style, Nix, Cairn, clean-head live, sync, and archive checks before publication. r[agent_harness_runtime.real_check_completion_host_smoke.closeout]
