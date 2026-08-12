## Why

Prime Agent's Continual Harness lets `/refine` review the current trajectory and apply small, evidence-backed updates to supplemental prompts, memories, skill descriptions, and reusable subagent specifications. Two constraints make it safe: the base system prompt is immutable, and recorded snapshots support rollback. Animus has checkpoint-rollback contracts and skill manifest descriptors, but no refinement loop over harness state. Lessons learned inside a session cannot persist as reviewed harness state today.

A naive adaptation would let the model rewrite its own operating instructions without review or rollback. Animus needs refinement as a pure admission problem: typed proposals, explicit evidence, fail-closed validation, deterministic snapshots, and session-local scope by default.

## What Changes

- Add typed refinement proposals for supplemental prompts, memories, skill descriptions, and subagent specifications, each carrying explicit evidence references. r[agent_harness_runtime.continual_harness.proposals]
- Add pure fail-closed admission with size, count, conflict, and authority bounds; the base system prompt remains immutable and out of scope for every proposal. r[agent_harness_runtime.continual_harness.admission]
- Add deterministic snapshots with BLAKE3 identity before every admitted update, plus bounded rollback to any retained snapshot. r[agent_harness_runtime.continual_harness.snapshots]
- Add scope rules: refinement state is session-local by default, and promotion to durable project or personal state is an explicit host action. r[agent_harness_runtime.continual_harness.scope]
- Add a Nickel-authored refinement policy for admitted target classes, bounds, evidence requirements, retention limits, and non-claims. r[agent_harness_runtime.continual_harness.profile]
- Add metadata-only refinement-history receipts. r[agent_harness_runtime.continual_harness.evidence]
- Add positive and negative conformance fixtures, a focused rail, and dependency-isolation guards. r[agent_harness_runtime.continual_harness.verification]

## Impact

- **Files**: new focused adapter-family crate for refinement contracts, Nickel policy, fixtures, focused check script, docs; composition seams in `animus-runtime` prompt/skill/memory composition.
- **Reusable boundary**: reusable surfaces gain no filesystem, VCS, or persistence authority; snapshot storage and durable promotion stay host-owned.
- **Runtime composition**: `model-runtime-profiles` composition consumes admitted supplemental state; the immutable base prompt stays with its existing owner.
- **Testing**: positive fixtures for admitted proposals, snapshots, and rollback; negative fixtures for base-prompt mutation, evidence-free proposals, bound violations, and silent promotion; no-std and Wasm checks; a dedicated Nix check.

## Out of Scope

- Mutating the immutable base system prompt, repo-owned skill packages, or accepted specs.
- Automatic promotion of session refinements into durable project state or release artifacts.
- Learning from trajectories without explicit evidence, or claiming that refinement improves model quality.
- Proving lesson correctness, generalization, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: continual-harness boundary, policy, proposals, admission, snapshots, scope, evidence, and verification.
