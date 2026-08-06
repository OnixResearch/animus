## Phase 1: Baseline and ownership

- [ ] [serial] B1 Run and record current `animus-steel`, runtime composition, subagent facade, compaction branch summaries, mid-turn compaction, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.rlm_kernel.verification]
- [ ] [serial] B2 Inventory the exact prime-agent cohort (revision, MIT license, RLM runtime and architecture docs) and map prompt-as-variable, typed host request, `rlm(...)`, and kernel-survives-compaction semantics onto Animus ownership. r[agent_harness_runtime.rlm_kernel.boundary]
- [ ] [serial] B3 Map ownership against `animus-steel`, `animus-runtime` composition preludes, subagent contracts, compaction owners, tool executor kit, and the component catalog; record the seam where the model-facing profile attaches. r[agent_harness_runtime.rlm_kernel.boundary]
- [ ] [depends:agent_harness_runtime.rlm_kernel.profile] B4 Add a typed Nickel kernel policy for exact cohort identity, fuel/depth/allocation/environment-size bounds, admitted host-operation classes, child-spawn budgets, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.rlm_kernel.profile]

## Phase 2: Kernel environment and host requests

- [ ] [depends:agent_harness_runtime.rlm_kernel.environment] K1 Add the focused no-std kernel contract crate with prompt-as-variable binding DTOs, persistent environment state, pure evaluation results, and bounded structural counts; add compatibility re-exports without changing default authority. r[agent_harness_runtime.rlm_kernel.environment]
- [ ] [depends:agent_harness_runtime.rlm_kernel.host_requests] K2 Define typed host requests for file, shell, tool, subagent, and context operations as data; add pure admission that revalidates every request against the admitted policy and fails closed. r[agent_harness_runtime.rlm_kernel.host_requests]
- [ ] [depends:agent_harness_runtime.rlm_kernel.host_requests] K3 Wire the host shell so kernel evaluation returns requests and results cross only the adapter boundary; keep filesystem, process, network, credential, clock, and persistence effects absent from reusable crates. r[agent_harness_runtime.rlm_kernel.host_requests]

## Phase 3: Bounded child spawn

- [ ] [depends:agent_harness_runtime.rlm_kernel.children] C1 Implement `rlm(...)` as a host request naming a child profile, explicit context slice, depth, fan-out, and result budget; reject over-limit or ambient spawns with typed outcomes. r[agent_harness_runtime.rlm_kernel.children]
- [ ] [depends:agent_harness_runtime.rlm_kernel.children] C2 Return child results as typed values with lineage into the existing subagent facade; keep child lifecycle, cancellation, and parent handoff with their existing owners. r[agent_harness_runtime.rlm_kernel.children]

## Phase 4: Compaction survival

- [ ] [depends:agent_harness_runtime.rlm_kernel.compaction_survival] S1 Add the contract that compaction never clears kernel variables, imports, or definitions; record kernel-held context in compaction plans so summaries reference it. r[agent_harness_runtime.rlm_kernel.compaction_survival]
- [ ] [depends:agent_harness_runtime.rlm_kernel.compaction_survival] S2 Add explicit kernel-clear as a separate host action with its own receipt; reject implicit clears during compaction, goal changes, or child completion. r[agent_harness_runtime.rlm_kernel.compaction_survival]

## Phase 5: Evidence

- [ ] [depends:agent_harness_runtime.rlm_kernel.evidence] E1 Implement metadata-only receipts binding policy identity, operation classes, bounded counts, child lineage, and domain-separated BLAKE3 identities; reject receipts containing variable values, code bodies, or transcripts. r[agent_harness_runtime.rlm_kernel.evidence]

## Phase 6: Positive and negative conformance

- [ ] [parallel] T1 Add positive fixtures for variable persistence across turns, admitted host requests, bounded `rlm(...)` spawn with typed results, and kernel survival across compaction. r[agent_harness_runtime.rlm_kernel.verification]
- [ ] [parallel] T2 Add negative fixtures for direct-effect attempts in model code, unadmitted operation classes, fuel/depth/size bound violations, over-limit or ambient child spawns, and implicit kernel clears. r[agent_harness_runtime.rlm_kernel.host_requests] r[agent_harness_runtime.rlm_kernel.children]
- [ ] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing variable contents, code bodies, child transcripts, sandbox claims, task-success claims, and authority widening. r[agent_harness_runtime.rlm_kernel.evidence] r[agent_harness_runtime.rlm_kernel.verification]
- [ ] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no filesystem, process, network, credential, clock, or persistence authority from the kernel profile. r[agent_harness_runtime.rlm_kernel.boundary] r[agent_harness_runtime.rlm_kernel.verification]

## Phase 7: Documentation and closeout

- [ ] [depends:agent_harness_runtime.rlm_kernel.evidence] D1 Document the model-facing Steel profile, host-request classes, child-spawn budgets, compaction-survival contract, upgrade procedure, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.rlm_kernel.boundary]
- [ ] [depends:agent_harness_runtime.rlm_kernel.verification] D2 Add `scripts/cairn/check-rlm-kernel` and a Nix check covering Nickel drift, fixture parity, admission fail-closed behavior, child budgets, compaction survival, safe receipts, and dependency isolation. r[agent_harness_runtime.rlm_kernel.verification]
- [ ] [depends:agent_harness_runtime.rlm_kernel.verification] D3 Run focused Steel/runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.rlm_kernel.verification]
- [ ] [depends:agent_harness_runtime.rlm_kernel.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening authority claims. r[agent_harness_runtime.rlm_kernel.verification]
