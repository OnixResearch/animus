## B. Baseline and ownership

- [x] [serial] B1: Record the exact reviewed `PrimeIntellect-ai/prime-agent` revision, RLM/kernel source paths, architecture docs, and MIT license provenance used by the adaptation. r[agent_harness_runtime.rlm_kernel.boundary]
- [x] [serial] B2: Map upstream semantics to Animus owners: persistent interpreter in `animus-steel`, pure contracts in a focused adapter crate, typed runtime admission in `animus-runtime`, and host effects in injected shells. r[agent_harness_runtime.rlm_kernel.boundary]
- [x] [serial] B3: Define the minimum parity slice: persistent variables, model-written helper functions, tool-shaped entry, typed host requests, bounded `rlm(...)` child spawn, and compaction survival. r[agent_harness_runtime.rlm_kernel.boundary.selected]
- [x] [serial] B4: Define explicit resource policy for environment size, CPU/fuel, recursion depth, allocation, child depth/fan-out, token/time budgets, and result size. r[agent_harness_runtime.rlm_kernel.profile]

## K. Kernel implementation

- [x] [serial] K1: Add a no-std/Wasm-compatible RLM kernel contract surface for named bindings, evaluation requests, typed results, state transitions, and bounded resource policy. r[agent_harness_runtime.rlm_kernel.boundary]
- [x] [serial] K2: Add a model-facing persistent Steel environment that can define helpers, read named prompt/context variables, and retain admitted state across turns. r[agent_harness_runtime.rlm_kernel.environment]
- [x] [serial] K3: Add a concrete `steel_repl`-style tool adapter or equivalent focused interface while preserving the existing host-owned `SteelLoopFabric` behavior. r[agent_harness_runtime.rlm_kernel.boundary.selected]
- [x] [serial] K4: Make evaluation atomic: failed code, parse errors, bound violations, and denied actions must not partially mutate persistent state. r[agent_harness_runtime.rlm_kernel.environment.bounded]
- [x] [serial] K5: Add explicit clear/reset behavior as a host action; implicit turn boundaries and prompt compaction must not clear kernel state. r[agent_harness_runtime.rlm_kernel.compaction_survival.no_implicit_clear]

## H. Typed host requests

- [x] [serial] H1: Define typed kernel-to-host requests for file read/edit, shell, generic tool call, subagent spawn, and context read without granting ambient effect authority. r[agent_harness_runtime.rlm_kernel.host_requests]
- [x] [serial] H2: Add runtime admission that revalidates operation class, target/scope, policy identity, bounds, correlation, and current-state freshness before dispatch. r[agent_harness_runtime.rlm_kernel.host_requests.admitted]
- [x] [serial] H3: Add an injected host executor seam that performs exactly the admitted request and returns a typed bounded result. r[agent_harness_runtime.rlm_kernel.host_requests.admitted]
- [x] [serial] H4: Prove negative paths for direct filesystem/process/network access, unadmitted operations, malformed requests, stale requests, and oversized results. r[agent_harness_runtime.rlm_kernel.host_requests.rejected]

## R. Recursive model calls

- [x] [serial] R1: Add a typed `rlm(...)` child request with explicit profile, context slice, depth, fan-out, token budget, wall-clock budget, result limit, and lineage. r[agent_harness_runtime.rlm_kernel.children]
- [x] [serial] R2: Reuse the existing subagent/runtime facade rather than introducing an ambient model-spawn path. r[agent_harness_runtime.rlm_kernel.children.spawn]
- [x] [serial] R3: Add deterministic fake-backed positive tests for admitted child calls and fail-closed tests for depth, fan-out, budget, unavailable executor, cancellation, and lineage mismatch. r[agent_harness_runtime.rlm_kernel.children.rejected]

## C. Compaction survival

- [x] [serial] C1: Extend Pi-compaction and Zerostack-midturn contracts so kernel state is a separately owned surface that survives transcript compaction. r[agent_harness_runtime.rlm_kernel.compaction_survival]
- [x] [serial] C2: Bind prompt/context variables into the kernel at run or session start, and record their identities without copying raw values into receipts. r[agent_harness_runtime.rlm_kernel.compaction_survival.preserved]
- [x] [serial] C3: Add tests that force compaction between Steel cells and prove helpers and variables remain available afterward. r[agent_harness_runtime.rlm_kernel.compaction_survival.preserved]
- [x] [serial] C4: Add negative tests that reject implicit kernel clearing, stale restored state, and cross-session state reuse without explicit authority. r[agent_harness_runtime.rlm_kernel.compaction_survival.no_implicit_clear]

## P. Policy, evidence, and receipts

- [x] [serial] P1: Add a typed Nickel RLM-kernel profile with positive and negative fixtures and deterministic generated runtime JSON. r[agent_harness_runtime.rlm_kernel.profile]
- [x] [serial] P2: Revalidate the generated profile in Rust before any model-facing interpreter starts. r[agent_harness_runtime.rlm_kernel.profile.valid]
- [x] [serial] P3: Add metadata-only receipts for evaluation, host-request admission, child calls, clear/reset, and compaction preservation. r[agent_harness_runtime.rlm_kernel.evidence]
- [x] [serial] P4: Reject variable values, model-written code bodies, raw child transcripts, credentials, raw paths, and unsupported whole-system claims from receipts. r[agent_harness_runtime.rlm_kernel.evidence.rejects]
- [x] [serial] P5: Add component-catalog and runtime-facade entries with explicit non-claims for sandboxing, model correctness, task completion, production readiness, and release eligibility. r[agent_harness_runtime.rlm_kernel.evidence.safe]

## T. Conformance and regression

- [x] [serial] T1: Add the exact upstream RLM fixture cohort and verify the admitted semantic subset with explicit revision and license provenance. r[agent_harness_runtime.rlm_kernel.verification]
- [x] [serial] T2: Add positive tests for state persistence, helper reuse, context bindings, typed host requests, bounded child calls, and compaction survival. r[agent_harness_runtime.rlm_kernel.verification.positive]
- [x] [serial] T3: Add negative tests for every operation and resource-bound family, including atomic failure and receipt leak fixtures. r[agent_harness_runtime.rlm_kernel.verification.negative]
- [x] [serial] T4: Preserve all existing `animus-steel`, runtime, adapter, Pi-compaction, Zerostack, conformance, safe-receipt, no-std, and Wasm tests. r[agent_harness_runtime.rlm_kernel.verification.closeout]

## D. Documentation and closeout

- [x] [serial] D1: Document the model-facing Steel workflow, kernel lifecycle, effect boundary, child-call limits, compaction interaction, examples, and troubleshooting. r[agent_harness_runtime.rlm_kernel.verification]
- [x] [serial] D2: Run focused Rust/Nickel/Steel checks, workspace tests, clippy, composition conformance, Pi/Zerostack checks, safe-receipt rails, no-std/Wasm builds, and the relevant Nix checks. r[agent_harness_runtime.rlm_kernel.verification.closeout]
- [x] [serial] D3: Record branch evidence and adversarial review results, update accepted specs, and archive this change. r[agent_harness_runtime.rlm_kernel.verification.closeout]
- [x] [serial] D4: Re-run lifecycle validation and strict archive gates after sync, then close the change only when every task and proof hook is complete. r[agent_harness_runtime.rlm_kernel.verification.closeout]
