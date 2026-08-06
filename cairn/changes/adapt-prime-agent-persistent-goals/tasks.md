## Phase 1: Baseline and ownership

- [ ] [serial] B1 Run and record current controller-continuation-policy, ralph-loop, neutral ledger, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.persistent_goals.verification]
- [ ] [serial] B2 Inventory the exact prime-agent goal cohort (revision, MIT license, `/goal` semantics, kernel-side completion surface) and map objective state, accounting, and lifecycle onto Animus ownership. r[agent_harness_runtime.persistent_goals.boundary]
- [ ] [serial] B3 Map ownership against `controller-continuation-policy`, `ralph-loop`, neutral ledger, and the component catalog; record the seam where goal state enters continuation decisions. r[agent_harness_runtime.persistent_goals.boundary]
- [ ] [depends:agent_harness_runtime.persistent_goals.profile] B4 Add a typed Nickel goal policy for budget classes, accounting fields, admitted transitions, one-active-goal rule, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.persistent_goals.profile]

## Phase 2: Goal state and lifecycle

- [ ] [depends:agent_harness_runtime.persistent_goals.state] G1 Add the focused no-std goal contract module with the durable goal record, status machine (`active`, `paused`, `completed`, `budget_limited`, `errored`, `cleared`), and pure transition functions over host-injected clocks and counters. r[agent_harness_runtime.persistent_goals.state]
- [ ] [depends:agent_harness_runtime.persistent_goals.state] G2 Enforce the one-active-goal rule, creation provenance, and accounting updates for tokens, elapsed time, and continuations with named bounds. r[agent_harness_runtime.persistent_goals.state]
- [ ] [depends:agent_harness_runtime.persistent_goals.ownership] G3 Restrict create, pause, resume, and clear to explicit user or host actions; add the narrow model surface for goal inspection and completion requests with admission. r[agent_harness_runtime.persistent_goals.ownership]

## Phase 3: Continuation and budget

- [ ] [depends:agent_harness_runtime.persistent_goals.continuation] C1 Wire active goal state into `controller-continuation-policy` as a typed input that raises continuation priority after ordinary turns; keep prompt injection with the continuation owner. r[agent_harness_runtime.persistent_goals.continuation]
- [ ] [depends:agent_harness_runtime.persistent_goals.continuation] C2 Enforce that only an admitted explicit completion transition produces `completed`, and that detach, compaction, and turn limits never do. r[agent_harness_runtime.persistent_goals.continuation]
- [ ] [depends:agent_harness_runtime.persistent_goals.budget] C3 Enforce token, time, and continuation budgets with a typed `budget_limited` outcome that downstream owners must not report as success. r[agent_harness_runtime.persistent_goals.budget]

## Phase 4: Evidence

- [ ] [depends:agent_harness_runtime.persistent_goals.evidence] E1 Implement metadata-only goal receipts binding policy identity, lifecycle transition, accounting counters, budget class, status, and domain-separated BLAKE3 identities; reject receipts containing objective text or success overclaims. r[agent_harness_runtime.persistent_goals.evidence]

## Phase 5: Positive and negative conformance

- [ ] [parallel] T1 Add positive fixtures for explicit creation, pause, resume, clear, accounting updates, admitted completion, and goal-aware continuation priority. r[agent_harness_runtime.persistent_goals.verification]
- [ ] [parallel] T2 Add negative fixtures for model-inferred goals, model-initiated pause or clear, invalid transitions, second active goal, budget-as-success confusion, and completion without an active goal. r[agent_harness_runtime.persistent_goals.ownership] r[agent_harness_runtime.persistent_goals.budget]
- [ ] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing objective text, trajectory spans, success overclaims, and authority widening. r[agent_harness_runtime.persistent_goals.evidence] r[agent_harness_runtime.persistent_goals.verification]
- [ ] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no clock, filesystem, process, or persistence authority from the goal module. r[agent_harness_runtime.persistent_goals.boundary] r[agent_harness_runtime.persistent_goals.verification]

## Phase 6: Documentation and closeout

- [ ] [depends:agent_harness_runtime.persistent_goals.evidence] D1 Document the goal record, status machine, explicit-completion rule, budget outcomes, one-active-goal rule, ownership split, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.persistent_goals.boundary]
- [ ] [depends:agent_harness_runtime.persistent_goals.verification] D2 Add `scripts/cairn/check-persistent-goals` and a Nix check covering Nickel drift, fixture parity, transition integrity, budget enforcement, ownership rules, safe receipts, and dependency isolation. r[agent_harness_runtime.persistent_goals.verification]
- [ ] [depends:agent_harness_runtime.persistent_goals.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.persistent_goals.verification]
- [ ] [depends:agent_harness_runtime.persistent_goals.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening authority or success claims. r[agent_harness_runtime.persistent_goals.verification]
