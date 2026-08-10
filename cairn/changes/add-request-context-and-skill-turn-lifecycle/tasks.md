## Phase 1: Baseline and ownership

- [ ] [serial] B1 Run current prompt assembly, model-profile, skill-manifest, context-reference, memory, and provider-dispatch baselines. r[agent_harness_runtime.request_context_contributions.verification]
- [ ] [serial] B2 Inventory every current request-context producer, conflict rule, budget owner, lifetime, and persistence path. r[agent_harness_runtime.request_context_contributions.contract]
- [ ] [parallel] B3 Record the pinned Autolith revision, ISC license, selected scenarios, rejected storage behavior, and no-runtime-dependency boundary. r[agent_harness_runtime.request_context_contributions.evidence]

## Phase 2: Contribution core

- [ ] [serial] C1 Add nominal contribution, payload, plan, provider-request, provider-response, and lifetime identities. r[agent_harness_runtime.request_context_contributions.contract]
- [ ] [serial] [depends:C1] C2 Add validated mandatory posture, priority, budget cost, deduplication, supersession, conflict-group, sensitivity, and lifetime fields. r[agent_harness_runtime.request_context_contributions.contract]
- [ ] [serial] [depends:C2] C3 Add the order-independent reducer and canonical tie-break rules. r[agent_harness_runtime.request_context_contributions.reducer]
- [ ] [serial] [depends:C3] C4 Add mandatory-budget reservation, advisory omission, and bounded conflict diagnostics. r[agent_harness_runtime.request_context_contributions.reducer]
- [ ] [serial] [depends:C3] C5 Add pure current-request, next-provider-response, and current-logical-turn transitions. r[agent_harness_runtime.request_context_contributions.retention]

## Phase 3: Skill turn and invocation composition

- [ ] [serial] [depends:C3] [depends:C5] H1 Adapt existing prompt, skill, memory, project-context, MCP, and runtime owners into contribution producers without moving their authority. r[agent_harness_runtime.request_context_contributions.composition]
- [ ] [serial] [depends:H1] H2 Integrate one selected contribution plan below model invocation composition and bind fallback, retry, resume, and replay drift. r[agent_harness_runtime.request_context_contributions.composition]
- [ ] [serial] [depends:C5] H3 Add exact skill-selection binding for session, turn, target, profile, manifest, body, and selection identities. r[agent_harness_runtime.request_context_contributions.skill_turn]
- [ ] [serial] [depends:H3] H4 Load the exact current skill body through a host service and emit one next-provider-response contribution. r[agent_harness_runtime.request_context_contributions.skill_turn]
- [ ] [serial] [depends:H2] [depends:H4] H5 Block logical turn completion until a correlated provider request consumes the contribution and a terminal response observation arrives. r[agent_harness_runtime.request_context_contributions.skill_turn]
- [ ] [serial] [depends:H2] H6 Prevent request-local payloads from entering ledger, compaction, memory, quality, and receipt sinks. r[agent_harness_runtime.request_context_contributions.retention]

## Phase 4: Conformance and evidence

- [ ] [parallel] [depends:H5] [depends:H6] T1 Add positive tests for priority, deduplication, supersession, conflicts, mandatory reservation, finite lifetimes, and deterministic ordering. r[agent_harness_runtime.request_context_contributions.verification]
- [ ] [parallel] [depends:H5] [depends:H6] T2 Add negative tests for source-order drift, duplicate identities, mandatory conflict, over-budget mandatory context, stale payloads, and target fallback inheritance. r[agent_harness_runtime.request_context_contributions.verification]
- [ ] [parallel] [depends:H5] T3 Add skill-turn tests for exact body load, metadata-only selection result, provider barrier, cancellation, retry, stale body, and unconsumed expiration. r[agent_harness_runtime.request_context_contributions.skill_turn] r[agent_harness_runtime.request_context_contributions.verification]
- [ ] [parallel] [depends:H6] T4 Add receipt and retention leak tests for prompt bodies, skill bodies, MCP instructions, memory records, paths, credentials, and hidden directives. r[agent_harness_runtime.request_context_contributions.evidence] r[agent_harness_runtime.request_context_contributions.verification]

## Phase 5: Documentation and closeout

- [ ] [serial] [depends:T1] [depends:T2] [depends:T3] [depends:T4] D1 Document the reducer, contribution producers, skill logical-turn flow, retention, ownership, and non-claims. r[agent_harness_runtime.request_context_contributions.contract] r[agent_harness_runtime.request_context_contributions.evidence]
- [ ] [serial] [depends:D1] D2 Update model-profile, skill, request-lifecycle, component, facade, roadmap, and README indexes. r[agent_harness_runtime.request_context_contributions.composition]
- [ ] [serial] [depends:D2] D3 Add a focused conformance rail and Nix check with positive and negative fixture families. r[agent_harness_runtime.request_context_contributions.verification]
- [ ] [serial] [depends:D3] D4 Run focused tests, formatting, Clippy, no-std and Wasm checks where applicable, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.request_context_contributions.verification]
- [ ] [serial] [depends:D4] D5 Review dry-run sync and archive plans, validate accepted requirement promotion, and archive only with current evidence. r[agent_harness_runtime.request_context_contributions.verification]
