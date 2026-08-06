## Phase 1: Baseline, source cohort, and ownership

- [ ] [serial] B1 Run and record current positive and negative Harbor planning, batch resume, ledger replay, role separation, evaluation-feedback, dataset-export, reward, safe-receipt, no-std, and Wasm baselines. r[agent_harness_runtime.native_evaluation_core.verification]
- [ ] [serial] B2 Inventory every generic task, dataset, trial, episode, reward, resume, receipt, and trace-reference owner under Harbor, batch, runtime, ledger, roles, dataset export, and evaluation feedback. r[agent_harness_runtime.native_evaluation_core.boundary]
- [ ] [serial] B3 Record Verifiers commit `1a997814e85a2e4f409add960aa852737388dbd8`, MIT provenance, exact v1 task/taskset/env/episode/trace/resume source files, and explicit adopt or reject decisions. r[agent_harness_runtime.native_evaluation_core.boundary] r[agent_harness_runtime.native_evaluation_core.evidence]

## Phase 2: Policy and focused crate

- [ ] [depends:agent_harness_runtime.native_evaluation_core.policy] P1 Add a typed Nickel evaluation profile for taskset, recipe, reward, resume, redaction, resource, evidence, and non-claim policy with named positive bounds and deny-by-default fields. r[agent_harness_runtime.native_evaluation_core.policy]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.policy] P2 Export checked generated profile data and add strict Rust revalidation plus domain-separated BLAKE3 source, profile, and normalized-policy identities. r[agent_harness_runtime.native_evaluation_core.policy]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.boundary] C1 Add the focused `animus-eval-core` `#![no_std]` plus `alloc` crate with no filesystem, process, network, environment, credential, clock, scheduler, provider, tool, persistence, Harbor, Python, or container dependency. r[agent_harness_runtime.native_evaluation_core.boundary]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.boundary] C2 Add stable typed identifiers, bounded diagnostics, canonical BLAKE3 builders, and compatibility re-exports through non-default runtime and root features. r[agent_harness_runtime.native_evaluation_core.boundary] r[agent_harness_runtime.native_evaluation_core.evidence]

## Phase 3: Tasks, episodes, and recipes

- [ ] [depends:agent_harness_runtime.native_evaluation_core.tasks] T1 Add immutable task-data, task-behavior, scorer, taskset, finite-membership, lazy-generator, selection, and ordering descriptors with explicit versions, seeds, finiteness, and bounds. r[agent_harness_runtime.native_evaluation_core.tasks]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.tasks] T2 Add pure validation and canonical identity for exact and lazy tasksets, including duplicate, unstable, unbounded, unsupported-shuffle, and missing-selection failures. r[agent_harness_runtime.native_evaluation_core.tasks]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.episodes] E1 Add pure environment-plan, seat, run, ledger-trace-reference, episode, terminal-observation, and safe-summary contracts without transcript storage. r[agent_harness_runtime.native_evaluation_core.episodes]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.recipes] E2 Add deterministic single-agent, best-of-N/pass-at-N, user-simulation, isolated-agentic-judge, and explicit shared-advisory-judge planners. r[agent_harness_runtime.native_evaluation_core.recipes]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.recipes] E3 Bind recipe plans to exact role, model, harness, tool, environment, verifier, rubric, resource, cancellation, redaction, and policy identities without dispatching effects. r[agent_harness_runtime.native_evaluation_core.recipes]

## Phase 4: Rewards, resume, and evidence

- [ ] [depends:agent_harness_runtime.native_evaluation_core.rewards] R1 Add finite typed reward dimensions, explicit domains and missing-value policy, plan-order reduction, contribution counts, and scorer/verifier/rubric attribution. r[agent_harness_runtime.native_evaluation_core.rewards]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.rewards] R2 Keep agent and LLM judge values advisory by default and reject NaN, infinity, duplicate dimensions, unknown domains, identity mismatch, silent clamping, and completion promotion. r[agent_harness_runtime.native_evaluation_core.rewards]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.resume] R3 Add full content-addressed resume keys and pure keep, rerun, block, and redo-whole-task decisions over caller-supplied evidence. r[agent_harness_runtime.native_evaluation_core.resume]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.resume] R4 Reject malformed, torn, failed, missing, stale, duplicated, conflicting, over-budget, wrong-policy, wrong-verifier, wrong-rubric, and wrong-output resume evidence. r[agent_harness_runtime.native_evaluation_core.resume]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.evidence] R5 Add metadata-only taskset, plan, episode, reward, resume, and evaluation receipts with leak and overclaim rejection. r[agent_harness_runtime.native_evaluation_core.evidence]

## Phase 5: Migration and conformance

- [ ] [depends:agent_harness_runtime.native_evaluation_core.boundary] M1 Migrate generic `animus-runtime::harbor_eval` planning and reduction behavior to consume native evaluation values while Harbor keeps only exact interoperability and backend semantics. r[agent_harness_runtime.native_evaluation_core.boundary]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.episodes] M2 Compose ledger references, batch ordering, role separation, evaluation feedback, dataset export, ATIF, safe receipts, and authoritative completion without transferring ownership. r[agent_harness_runtime.native_evaluation_core.episodes] r[agent_harness_runtime.native_evaluation_core.evidence]
- [ ] [parallel] V1 Add positive tests for exact and lazy tasks, every recipe, multi-agent episodes, finite rewards, exact resume, stable identities, safe receipts, and Harbor compatibility. r[agent_harness_runtime.native_evaluation_core.verification]
- [ ] [parallel] V2 Add negative tests for unstable generators, missing bounds, duplicate tasks, unsafe shuffle, transcript duplication, role widening, shared-judge promotion, reward faults, stale resume, identity drift, leaks, and overclaims. r[agent_harness_runtime.native_evaluation_core.verification]
- [ ] [parallel] V3 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no host authority or Verifiers/Python runtime dependency. r[agent_harness_runtime.native_evaluation_core.boundary] r[agent_harness_runtime.native_evaluation_core.verification]

## Phase 6: Documentation and closeout

- [ ] [depends:agent_harness_runtime.native_evaluation_core.evidence] D1 Document native evaluation ownership, API examples, recipe semantics, taskset determinism, resume binding, Harbor migration, privacy defaults, and non-claims. r[agent_harness_runtime.native_evaluation_core.boundary] r[agent_harness_runtime.native_evaluation_core.evidence]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.verification] D2 Update contracts, component, facade, request-lifecycle, safety, focused-rail, roadmap, README check, and bounded Verifiers reference indexes. r[agent_harness_runtime.native_evaluation_core.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.verification] D3 Add `scripts/cairn/check-native-evaluation-core` and a Nix check for Nickel drift, pure behavior, migration, no-std/Wasm, safe receipts, and dependency isolation. r[agent_harness_runtime.native_evaluation_core.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.verification] D4 Run focused tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition rails, Cairn validation and gates, and the focused Nix check. r[agent_harness_runtime.native_evaluation_core.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_core.verification] D5 Review dry-run sync and archive plans, verify every new requirement reaches the accepted spec, and archive only after current positive and negative evidence passes. r[agent_harness_runtime.native_evaluation_core.verification]
