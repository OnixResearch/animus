# Tasks

## B. Boundary and capability packs

- [ ] [serial] B1: Inventory concrete candidate components, licenses, authority, errors, and unsupported operations. r[agent_harness_runtime.rlm_host_executor.verification]
- [ ] [serial] B2: Define the non-default executor construction contract, route bindings, result DTOs, failure classes, and named bounds. r[agent_harness_runtime.rlm_host_executor.boundary]
- [ ] [depends:B2] B3: Add pure route selection and exact binding validation. r[agent_harness_runtime.rlm_host_executor.dispatch]

## H. Host routes

- [ ] [depends:B3] H1: Add the explicit capability-relative file-read pack and bind the existing revision-gated edit host. r[agent_harness_runtime.rlm_host_executor.workspace]
- [ ] [depends:B3] H2: Bind exact command profiles to bounded process execution without a shell fallback. r[agent_harness_runtime.rlm_host_executor.process]
- [ ] [depends:B3] H3: Bind named tools and context resolvers through their existing application ports. r[agent_harness_runtime.rlm_host_executor.services]
- [ ] [depends:B3] H4: Bind subagent requests to the injected child service and kernel clear to the bridge owner. r[agent_harness_runtime.rlm_host_executor.services]
- [ ] [depends:H1] H5: Recheck current seal, generation, cancellation, scope, and binding identity before each effect. r[agent_harness_runtime.rlm_host_executor.dispatch]
- [ ] [depends:H5] H6: Add bounded result translation and metadata-only execution observations. r[agent_harness_runtime.rlm_host_executor.boundary]

## T. Tests and closeout

- [ ] [depends:H6] T1: Add positive tests for every operation class through the production executor. r[agent_harness_runtime.rlm_host_executor.verification]
- [ ] [depends:H6] T2: Add negative tests for missing packs, denied scope, stale seals, path races, edit conflicts, unknown commands, tool errors, context leaks, child failures, cancellation, timeout, oversized output, fallback attempts, and receipt leaks. r[agent_harness_runtime.rlm_host_executor.verification]
- [ ] [depends:T2] T3: Add feature, dependency, and license-isolation checks for reusable and default graphs. r[agent_harness_runtime.rlm_host_executor.boundary]
- [ ] [depends:T3] T4: Update docs and component metadata, add a focused rail, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.rlm_host_executor.verification]
