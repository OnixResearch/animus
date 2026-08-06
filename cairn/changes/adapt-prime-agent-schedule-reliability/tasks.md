## Phase 1: Baseline and ownership

- [ ] [serial] B1 Run and record current goose-scheduled-recipes, messaging modes, neutral ledger, safe receipts, no-std, and Wasm baselines before changing reusable behavior. r[agent_harness_runtime.schedule_reliability.verification]
- [ ] [serial] B2 Inventory the exact prime-agent scheduling cohort (revision, MIT license, `/heartbeat`, `rlm_heartbeat`, `prime-agent schedule` semantics) and map surfaces, claim ordering, and coalescing onto Animus ownership. r[agent_harness_runtime.schedule_reliability.boundary]
- [ ] [serial] B3 Map ownership against `goose-scheduled-recipes`, messaging receipts, neutral ledger, continuation policy, and the component catalog; record the seam where schedule decisions enter prompt queues. r[agent_harness_runtime.schedule_reliability.boundary]
- [ ] [depends:agent_harness_runtime.schedule_reliability.profile] B4 Add a typed Nickel schedule policy for surfaces, interval bounds, catch-up rules, pending limits, receipt fields, and required non-claims; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.schedule_reliability.profile]

## Phase 2: Surfaces and durable state

- [ ] [depends:agent_harness_runtime.schedule_reliability.surfaces] S1 Add the focused no-std schedule contract module with job records carrying surface (`user_heartbeat`, `agent_recurring`, `scheduled`) and owner identity, plus mutation admission that rejects cross-surface mutation. r[agent_harness_runtime.schedule_reliability.surfaces]
- [ ] [depends:agent_harness_runtime.schedule_reliability.surfaces] S2 Enforce the one-visible-user-heartbeat rule and the agent surface's inability to replace or clear user-owned jobs. r[agent_harness_runtime.schedule_reliability.surfaces]
- [ ] [depends:agent_harness_runtime.schedule_reliability.persistence] S3 Implement session-scoped durable job state with domain-separated BLAKE3 identity and deterministic recovery replay from injected clocks. r[agent_harness_runtime.schedule_reliability.persistence]

## Phase 3: Claim-before-delivery and coalescing

- [ ] [depends:agent_harness_runtime.schedule_reliability.claim_before_delivery] C1 Implement the two-step plan (claim transition, then delivery instruction) and the recovery rule that claimed-but-undelivered ticks are not re-due. r[agent_harness_runtime.schedule_reliability.claim_before_delivery]
- [ ] [depends:agent_harness_runtime.schedule_reliability.coalesce] C2 Implement missed-tick coalescing with policy-named catch-up decisions (fire once, skip, or pause) and missed-count metadata. r[agent_harness_runtime.schedule_reliability.coalesce]

## Phase 4: Evidence

- [ ] [depends:agent_harness_runtime.schedule_reliability.evidence] E1 Implement metadata-only schedule receipts binding policy identity, job identity, surface, claim/delivery/coalesce classes, clock identity, missed counts, and domain-separated BLAKE3 identities; reject receipts containing prompt bodies. r[agent_harness_runtime.schedule_reliability.evidence]

## Phase 5: Positive and negative conformance

- [ ] [parallel] T1 Add positive fixtures for on-time claim and delivery, crash recovery without replay, downtime coalescing, surface ownership, and deterministic restart recovery. r[agent_harness_runtime.schedule_reliability.verification]
- [ ] [parallel] T2 Add negative fixtures for deliver-then-record ordering, backlog accumulation, agent mutation of user jobs, second visible user heartbeat, and cross-session schedules. r[agent_harness_runtime.schedule_reliability.claim_before_delivery] r[agent_harness_runtime.schedule_reliability.surfaces]
- [ ] [parallel] T3 Add privacy and claim-boundary negatives for receipts containing prompt bodies, exactly-once claims, real-time claims, and authority widening. r[agent_harness_runtime.schedule_reliability.evidence] r[agent_harness_runtime.schedule_reliability.verification]
- [ ] [parallel] T4 Add dependency and source-shape guards proving default, reusable, no-std, and Wasm surfaces gain no clock, timer, filesystem, process, or persistence authority from the schedule module. r[agent_harness_runtime.schedule_reliability.boundary] r[agent_harness_runtime.schedule_reliability.verification]

## Phase 6: Documentation and closeout

- [ ] [depends:agent_harness_runtime.schedule_reliability.evidence] D1 Document the three surfaces, ownership rules, claim-before-delivery ordering, coalescing semantics, recovery behavior, and non-claims; update the contracts index, component catalog, facade inventory, request lifecycle, focused rails, safety contract, and README check index. r[agent_harness_runtime.schedule_reliability.boundary]
- [ ] [depends:agent_harness_runtime.schedule_reliability.verification] D2 Add `scripts/cairn/check-schedule-reliability` and a Nix check covering Nickel drift, fixture parity, claim ordering, coalescing bounds, recovery determinism, safe receipts, and dependency isolation. r[agent_harness_runtime.schedule_reliability.verification]
- [ ] [depends:agent_harness_runtime.schedule_reliability.verification] D3 Run focused runtime/std tests, formatting, Clippy, no-std/Wasm checks, Nickel checks, component/facade/composition/safe-receipt rails, the new focused rail, Cairn validation and proposal/design/tasks gates, and the relevant Nix check; record exact blockers. r[agent_harness_runtime.schedule_reliability.verification]
- [ ] [depends:agent_harness_runtime.schedule_reliability.verification] D4 Review dry-run sync/archive plans, verify every new requirement ID reaches the accepted spec, run evidence-safety checks, and archive only after current deterministic positive and negative evidence passes without widening delivery or authority claims. r[agent_harness_runtime.schedule_reliability.verification]
