## Why

Prime Agent's Recursive Language Model (RLM) treats context as variables inside a persistent model-facing REPL. Tools and subagents are function calls from model-written code. This pattern has no equivalent in Animus today: `animus-steel` is a host-owned loop fabric, and the model never receives a persistent programmable environment. Long tasks must re-serialize context into every prompt instead of holding it as named values.

Adapting RLM naively would hand model-generated code direct filesystem, process, or network effects. Prime Agent itself documents that its kernel is not a security sandbox. Animus needs the same mechanism behind its own rule: every model-selected action crosses a typed host request that Rust revalidates at the adapter boundary before any effect.

## What Changes

- Add a non-default model-facing Steel environment contract where prompt context binds to named variables that persist across turns. r[agent_harness_runtime.rlm_kernel.environment]
- Add typed host requests as the only effect channel for model code; Rust admission revalidates every file, shell, tool, subagent, and context operation before execution. r[agent_harness_runtime.rlm_kernel.host_requests]
- Add a bounded `rlm(...)` child-spawn contract with depth, fan-out, and result-size limits plus typed child results and lineage. r[agent_harness_runtime.rlm_kernel.children]
- Add the invariant that environment state survives compaction: compaction shrinks prompt context but never destroys kernel variables. r[agent_harness_runtime.rlm_kernel.compaction_survival]
- Add a Nickel-authored kernel policy for exact cohort identity, resource bounds, admitted host operations, and required non-claims. r[agent_harness_runtime.rlm_kernel.profile]
- Add metadata-only receipts for environment, host-request, child, and compaction events. r[agent_harness_runtime.rlm_kernel.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.rlm_kernel.verification]

## Impact

- **Files**: new focused adapter-family crate for kernel contracts, `animus-steel` and `animus-runtime` composition seams, Nickel policy, fixtures, focused check script, docs.
- **Reusable boundary**: reusable, default, no-std, and Wasm surfaces gain no filesystem, process, network, credential, or persistence authority; the host shell owns all effects.
- **Runtime composition**: `animus-steel` stays the canonical fabric; the RLM kernel is a model-facing profile over the same adapter boundaries, not a second runtime.
- **Testing**: positive fixtures for variable persistence, typed host requests, bounded child spawn, and compaction survival; negative fixtures for direct-effect attempts, bound violations, and overclaims; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Executing model code with ambient operating-system authority, or claiming the kernel is a security sandbox.
- IPython or Python compatibility; the model-facing language is Steel.
- Unbounded recursion, ambient subagent spawning, or cross-session kernel sharing.
- Proving model output correctness, task completion, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: RLM kernel boundary, policy, environment, host requests, children, compaction survival, evidence, and verification.
