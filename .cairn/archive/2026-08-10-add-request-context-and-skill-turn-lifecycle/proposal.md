## Why

Animus composes prompts, skills, memory, and tools through several typed contracts. It lacks one request-local reducer for priority, supersession, deduplication, conflicts, and finite lifetimes.

The missing reducer also leaves a host-flow gap for selected skills. A host can load a skill body without proving that one later provider response consumed it.

Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` provides ISC-licensed context and skill-flow reference scenarios. Animus will adapt the behaviors without copying its prompt store or S-expression formats.

## What Changes

- Add typed request-context contribution descriptors and finite lifetime classes. r[agent_harness_runtime.request_context_contributions.contract]
- Add a pure deterministic reducer for priority, budgets, deduplication, supersession, conflicts, and mandatory contributions. r[agent_harness_runtime.request_context_contributions.reducer]
- Integrate the reducer below existing model-profile composition instead of creating another prompt system. r[agent_harness_runtime.request_context_contributions.composition]
- Bind explicit skill selection to an exact body identity and a required next-provider-response barrier. r[agent_harness_runtime.request_context_contributions.skill_turn]
- Prevent request-local or sensitive contribution bodies from entering conversation, compaction, memory, or receipt storage. r[agent_harness_runtime.request_context_contributions.retention]
- Add safe receipts and paired positive and negative conformance fixtures. r[agent_harness_runtime.request_context_contributions.evidence] r[agent_harness_runtime.request_context_contributions.verification]

## Success Criteria

- Equal inputs produce one deterministic selected contribution order.
- Missing or conflicting mandatory contributions block provider dispatch.
- A selected skill body reaches only the exact next admitted provider request.
- Logical turn completion waits for the provider response that consumed the selected skill.
- Receipts expose identities and outcomes without exposing context bodies.

## False Completion

The change is incomplete if it creates a second prompt assembler or lets source order decide conflicts.

Skill loading alone is not completion. The selected body must bind one later provider request and response.

## Impact

- **Reusable runtime**: contribution validation, reduction, lifetime transitions, skill-turn gates, and safe receipt projection.
- **Host shell**: body loading, source effects, provider calls, persistence decisions, and disposal.
- **Existing owners**: prompt assembly, model runtime profiles, skill manifests, memory, and tool catalogs keep their current authority.
- **Testing**: priority, conflict, budget, lifetime, stale body, provider barrier, leak, and fallback fixtures.

## Out of Scope

- A new memory store, transcript format, prompt language, skill discovery service, or automatic skill activation.
- Durable retention of sensitive instructions or arbitrary multi-turn prompt state.

## Affected Specs

- `agent-harness-runtime`: request-context contribution contract, reducer, composition, skill turn, retention, evidence, and verification.
