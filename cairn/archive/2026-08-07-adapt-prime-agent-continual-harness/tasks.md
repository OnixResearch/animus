## Phase 1: Baseline and ownership

- [x] [serial] B1 Run and record current model-runtime-profiles, skill manifests, memory contracts, checkpoint-rollback, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.continual_harness.verification]
- [x] [serial] B2 Inventory the exact prime-agent continual-harness cohort (revision, MIT license, `/refine` semantics, arXiv:2605.09998 claims) and map supplemental prompt, memory, skill-description, and subagent-spec targets onto Animus ownership. r[agent_harness_runtime.continual_harness.boundary]
- [x] [serial] B3 Map ownership against `animus-runtime` composition preludes, `model-runtime-profiles`, `hermes-skill-manifests`, checkpoint-rollback, neutral ledger, and the component catalog; record the seam where admitted supplemental state enters composition. r[agent_harness_runtime.continual_harness.boundary]
- [x] [depends:agent_harness_runtime.continual_harness.profile] B4 Add a typed Nickel refinement policy for admitted target classes, size/count/retention bounds, evidence requirements, conflict rules, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.continual_harness.profile]

## Phase 2: Proposals and admission

- [x] [depends:agent_harness_runtime.continual_harness.proposals] P1 Add the focused no-std refinement contract crate with typed proposal DTOs for the four target classes, explicit evidence references, and add/update/retract operations; add compatibility re-exports without changing default authority. r[agent_harness_runtime.continual_harness.proposals]
- [x] [depends:agent_harness_runtime.continual_harness.admission] P2 Implement pure fail-closed admission with size, count, conflict, and authority bounds; reject base-prompt, repo-owned skill, accepted-spec, and policy targets with typed outcomes. r[agent_harness_runtime.continual_harness.admission]
- [x] [depends:agent_harness_runtime.continual_harness.admission] P3 Reject evidence-free or malformed-evidence proposals and proposals that request durable promotion or ambient persistence. r[agent_harness_runtime.continual_harness.admission] r[agent_harness_runtime.continual_harness.scope]

## Phase 3: Snapshots and rollback

- [x] [depends:agent_harness_runtime.continual_harness.snapshots] S1 Implement deterministic snapshot planning with domain-separated BLAKE3 identity over pre-update supplemental state and named retention bounds. r[agent_harness_runtime.continual_harness.snapshots]
- [x] [depends:agent_harness_runtime.continual_harness.snapshots] S2 Implement bounded rollback to any retained admitted snapshot with its own receipt; reject rollback to unknown, evicted, or forged snapshot identities. r[agent_harness_runtime.continual_harness.snapshots]

## Phase 4: Scope and promotion

- [x] [depends:agent_harness_runtime.continual_harness.scope] C1 Implement session-local state ownership and the explicit host promotion path with separate admission and receipt; prove proposals cannot trigger promotion. r[agent_harness_runtime.continual_harness.scope]

## Phase 5: Evidence

- [x] [depends:agent_harness_runtime.continual_harness.evidence] E1 Implement metadata-only refinement-history receipts binding policy identity, target class, operation, snapshot identities, status, and non-claims; reject receipts containing lesson bodies, prompt text, memory contents, or trajectory spans. r[agent_harness_runtime.continual_harness.evidence]

## Phase 6: Positive and negative conformance

- [x] [parallel] T1 Add positive fixtures for admitted proposals in each target class, snapshot-then-update ordering, exact rollback, and explicit host promotion. r[agent_harness_runtime.continual_harness.verification]
- [x] [parallel] T2 Add negative fixtures for base-prompt targets, evidence-free proposals, size/count/conflict bound violations, forged snapshot identities, and implicit promotion attempts. r[agent_harness_runtime.continual_harness.admission] r[agent_harness_runtime.continual_harness.snapshots]
- [x] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing lesson bodies, prompt text, memory contents, trajectory spans, improvement claims, and authority widening. r[agent_harness_runtime.continual_harness.evidence] r[agent_harness_runtime.continual_harness.verification]
- [x] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no filesystem, VCS, process, or persistence authority from the refinement core. r[agent_harness_runtime.continual_harness.boundary] r[agent_harness_runtime.continual_harness.verification]

## Phase 7: Documentation and closeout

- [x] [depends:agent_harness_runtime.continual_harness.evidence] D1 Document the four target classes, admission rules, snapshot and rollback semantics, session-local default, explicit promotion path, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.continual_harness.boundary]
- [x] [depends:agent_harness_runtime.continual_harness.verification] D2 Add `scripts/cairn/check-continual-harness` and a Nix check covering Nickel drift, fixture parity, admission fail-closed behavior, snapshot integrity, rollback exactness, scope rules, safe receipts, and dependency isolation. r[agent_harness_runtime.continual_harness.verification]
- [x] [depends:agent_harness_runtime.continual_harness.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.continual_harness.verification]
- [x] [depends:agent_harness_runtime.continual_harness.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening authority or improvement claims. r[agent_harness_runtime.continual_harness.verification]
