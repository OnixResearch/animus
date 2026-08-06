## Phase 1: Baseline and ownership

- [ ] [serial] B1 Run and record current controller-continuation-policy, ralph-loop, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.autonomous_gates.verification]
- [ ] [serial] B2 Inventory the exact prime-agent autonomous cohort (revision, MIT license, `/autonomous` and gate CLI semantics) and map limits, gates, bounded output, and change detection onto Animus ownership. r[agent_harness_runtime.autonomous_gates.boundary]
- [ ] [serial] B3 Map ownership against `controller-continuation-policy`, `ralph-loop`, tool executor kit, bounded output contracts, and the component catalog; record the seam where gate results enter continuation decisions. r[agent_harness_runtime.autonomous_gates.boundary]
- [ ] [depends:agent_harness_runtime.autonomous_gates.profile] B4 Add a typed Nickel autonomous policy for continuation/turn/token/wall-clock limits, gate descriptors, output bounds, fingerprint input identity, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.autonomous_gates.profile]

## Phase 2: Continuation limits and gates

- [ ] [depends:agent_harness_runtime.autonomous_gates.continuation] A1 Add the focused no-std autonomous contract module with explicit continuation, turn, token, and wall-clock limits as typed inputs to `controller-continuation-policy`. r[agent_harness_runtime.autonomous_gates.continuation]
- [ ] [depends:agent_harness_runtime.autonomous_gates.gates] A2 Define gate descriptors and typed results (pass, fail with class, timeout) as host-injected executor outputs; keep process execution outside reusable crates. r[agent_harness_runtime.autonomous_gates.gates]
- [ ] [depends:agent_harness_runtime.autonomous_gates.gates] A3 Implement bounded failed-gate output projection (byte bound, output class, head/tail rules) before output enters the prompt. r[agent_harness_runtime.autonomous_gates.gates]

## Phase 3: Change detection

- [ ] [depends:agent_harness_runtime.autonomous_gates.change_detection] F1 Implement (gate identity, fingerprint, outcome) recording and the deterministic rerun denial when a failed gate's fingerprint is unchanged. r[agent_harness_runtime.autonomous_gates.change_detection]
- [ ] [depends:agent_harness_runtime.autonomous_gates.change_detection] F2 Define the domain-separated BLAKE3 fingerprint input contract over the admitted file set and the explicit non-claim for uncovered paths. r[agent_harness_runtime.autonomous_gates.change_detection]

## Phase 4: Success boundary and evidence

- [ ] [depends:agent_harness_runtime.autonomous_gates.no_success_claim] S1 Add typed `verified_scope` on gate results and explicit `limit_reached` / `gates_passed` outcome classes; add receipt validation that rejects success projections of these outcomes. r[agent_harness_runtime.autonomous_gates.no_success_claim]
- [ ] [depends:agent_harness_runtime.autonomous_gates.evidence] E1 Implement metadata-only gate and continuation receipts binding policy identity, gate identity, verified scope, outcome class, fingerprint identity, limit class, and domain-separated BLAKE3 identities; reject receipts containing raw gate output or success overclaims. r[agent_harness_runtime.autonomous_gates.evidence]

## Phase 5: Positive and negative conformance

- [ ] [parallel] T1 Add positive fixtures for gate pass completion of autonomous mode, bounded retry after failure, fingerprint-skip of unchanged reruns, and clean limit stops. r[agent_harness_runtime.autonomous_gates.verification]
- [ ] [parallel] T2 Add negative fixtures for reusable-crate execution attempts, unbounded output, stale fingerprints, rerun attempts without change, and limit-as-success confusion. r[agent_harness_runtime.autonomous_gates.gates] r[agent_harness_runtime.autonomous_gates.change_detection]
- [ ] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing raw gate output, workspace paths outside the admitted set, success overclaims, and authority widening. r[agent_harness_runtime.autonomous_gates.evidence] r[agent_harness_runtime.autonomous_gates.verification]
- [ ] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no process, filesystem, clock, or persistence authority from the autonomous module. r[agent_harness_runtime.autonomous_gates.boundary] r[agent_harness_runtime.autonomous_gates.verification]

## Phase 6: Documentation and closeout

- [ ] [depends:agent_harness_runtime.autonomous_gates.evidence] D1 Document limit classes, gate descriptors, bounded output rules, fingerprint semantics and their coverage non-claim, the success boundary, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.autonomous_gates.boundary]
- [ ] [depends:agent_harness_runtime.autonomous_gates.verification] D2 Add `scripts/cairn/check-autonomous-gates` and a Nix check covering Nickel drift, fixture parity, rerun denial determinism, output bounding, success-boundary validation, safe receipts, and dependency isolation. r[agent_harness_runtime.autonomous_gates.verification]
- [ ] [depends:agent_harness_runtime.autonomous_gates.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.autonomous_gates.verification]
- [ ] [depends:agent_harness_runtime.autonomous_gates.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening success or authority claims. r[agent_harness_runtime.autonomous_gates.verification]
