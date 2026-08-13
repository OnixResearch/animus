# Tasks

## B. Boundary and capability packs

- [x] [serial] B1: Inventory concrete candidate components, licenses, authority, errors, and unsupported operations. r[agent_harness_runtime.rlm_host_executor.verification]
  - Evidence: `docs/rlm-host-effect-executor.md` records all seven owners, authority, limits, failures, AGPL isolation, and unsupported fallback behavior.
- [x] [serial] B2: Define the non-default executor construction contract, route bindings, result DTOs, failure classes, and named bounds. r[agent_harness_runtime.rlm_host_executor.boundary]
  - Evidence: `crates/animus-std-rlm-host/src/{contract,executor}.rs` defines sealed bindings, capability packs, bounds, typed failures, and private-safe results.
- [x] [depends:B2] B3: Add pure route selection and exact binding validation. r[agent_harness_runtime.rlm_host_executor.dispatch]
  - Evidence: `RouteClass`, `RouteScope`, `BindingSet`, `validate_packs`, and conformance fixtures reject missing, duplicate, mismatched, or stale bindings.

## H. Host routes

- [x] [depends:B3] H1: Add the explicit capability-relative file-read pack and bind the existing revision-gated edit host. r[agent_harness_runtime.rlm_host_executor.workspace]
  - Evidence: `FileReadPack` uses lexical admission, caller-opened capability roots, descriptor no-follow, bounds, and identity rechecks; `RevisionGatedEditRoute` calls `WorkspaceEditHost` with exact binder-owned facts.
- [x] [depends:B3] H2: Bind exact command profiles to bounded process execution without a shell fallback. r[agent_harness_runtime.rlm_host_executor.process]
  - Evidence: `ProcessPack` resolves one named native profile, requires absolute program and working directory, clears the environment, rejects unsupported CPU or memory claims, and calls `bounded_exec::run_with_cancellation`.
- [x] [depends:B3] H3: Bind named tools and context resolvers through their existing application ports. r[agent_harness_runtime.rlm_host_executor.services]
  - Evidence: `ToolKitRoute` and `ResolverContextRoute` use the existing tool-executor kit, strict request binder, `ContextResolverSet`, and explicit budgets.
- [x] [depends:B3] H4: Bind subagent requests to the injected child service and kernel clear to the bridge owner. r[agent_harness_runtime.rlm_host_executor.services]
  - Evidence: `ConcreteChildRoute` uses `ConcreteChildSessionService`; `SteelKernelClearRoute` delegates to one injected `SteelRlmKernel` owner.
- [x] [depends:H1] H5: Recheck current seal, generation, cancellation, scope, and binding identity before each effect. r[agent_harness_runtime.rlm_host_executor.dispatch]
  - Evidence: `CurrentAuthorityObservation::validate` calls the existing authority-seal recheck and checks complete request scope, pack scope, binding identity, generation, cancellation, and timeout before dispatch.
- [x] [depends:H5] H6: Add bounded result translation and metadata-only execution observations. r[agent_harness_runtime.rlm_host_executor.boundary]
  - Evidence: `RlmHostResult` separates private model-visible text from metadata-only BLAKE3 observations and redacts its `Debug` view.

## T. Tests and closeout

- [x] [depends:H6] T1: Add positive tests for every operation class through the production executor. r[agent_harness_runtime.rlm_host_executor.verification]
  - Evidence: `routes_every_operation_to_its_exact_owner` executes file read, edit, process, tool, child, context, and kernel clear through one executor.
- [x] [depends:H6] T2: Add negative tests for missing packs, denied scope, stale seals, path races, edit conflicts, unknown commands, tool errors, context leaks, child failures, cancellation, timeout, oversized output, fallback attempts, and receipt leaks. r[agent_harness_runtime.rlm_host_executor.verification]
  - Evidence: executor conformance tests cover missing packs, wrong scope, stale generation, cancellation, timeout, traversal, symlink, bounds, unknown profiles, tool errors, context limits, and debug leak rejection. Existing owner suites cover edit conflicts, child failures, and route-owner race semantics.
- [x] [depends:T2] T3: Add feature, dependency, and license-isolation checks for reusable and default graphs. r[agent_harness_runtime.rlm_host_executor.boundary]
  - Evidence: the focused rail verifies AGPL-3.0-or-later, non-default root and workspace graphs, no-std/Wasm boundaries, and absence of ambient executor authority.
- [x] [depends:T3] T4: Update docs and component metadata, add a focused rail, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.rlm_host_executor.verification]
  - Evidence: README, component catalog, facade inventory, focused rail, and focused Nix check are updated. Focused, workspace, Clippy, catalog, facade, and Nix checks pass.
