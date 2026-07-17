## Phase 1: Typed campaign profile and canonical skill

- [x] [serial] Add a typed Nickel `aeneas-proof-campaign` profile referencing existing recipe, semantic role, model runtime, skill, tool, verification-service, budget, output, and receipt contracts. r[agent_harness_runtime.aeneas_proof_campaign.profile]
- [x] [serial] Add one compact canonical campaign skill source, generated safe manifest/catalog projection, BLAKE3 freshness validation, and positive/negative duplicate/drift fixtures. r[agent_harness_runtime.aeneas_proof_campaign.skill_source]
- [x] [parallel] Add profile negatives for unknown provider/service IDs, missing role/output/budget refs, capability widening, unsafe labels, stale hashes, hidden prompt replacement, and weakened non-claims. r[agent_harness_runtime.aeneas_proof_campaign.profile] r[agent_harness_runtime.aeneas_proof_campaign.skill_source]

## Phase 2: Pure workflow and role separation

- [x] [serial] Define pure campaign plan/state/handoff DTOs over statement-author, statement-reviewer, prover, and verifier assignments composed from existing semantic role classes. r[agent_harness_runtime.aeneas_proof_campaign.roles]
- [x] [serial] Enforce immutable subject binding and reviewed distinct assignment/run/context/mutation-lineage dimensions for review and verification without claiming global independence. r[agent_harness_runtime.aeneas_proof_campaign.separation]
- [x] [parallel] Add negative fixtures for self-review, inherited mutable context, verifier mutation, stale subject/model/check identity, role alias bypass, and unioned capabilities. r[agent_harness_runtime.aeneas_proof_campaign.roles] r[agent_harness_runtime.aeneas_proof_campaign.separation]

## Phase 3: Statement, artifact, and trust gates

- [x] [serial] Require statement-review approval binding requirement ID, statement BLAKE3, claim class, source/model/type identities, adequacy/vacuity decision, and complete trust closure before proof execution. r[agent_harness_runtime.aeneas_proof_campaign.statement_gate]
- [x] [serial] Enforce generated-versus-maintained artifact roles, non-overlapping destinations, regeneration freshness, and prover mutation limits. r[agent_harness_runtime.aeneas_proof_campaign.artifact_boundary]
- [x] [parallel] Add negative fixtures for vacuous statements, `sorry`/`admit`, undeclared models/axioms/opaque dependencies, post-review statement drift, generated-file mutation, output overlap, and stale regeneration. r[agent_harness_runtime.aeneas_proof_campaign.statement_gate] r[agent_harness_runtime.aeneas_proof_campaign.artifact_boundary]

## Phase 4: Budgets, outcomes, services, and receipts

- [x] [serial] Implement pure named-budget admission and typed terminal outcomes for proved, blocked, counterexample, invalid statement/model, budget exhaustion, tool failure, denied, cancelled, and abstained. r[agent_harness_runtime.aeneas_proof_campaign.outcomes]
- [x] [serial] Bind host-injected proof/evidence service requests to exact assignment, subject, statement, model, budget, policy, and capability identities without reusable-core effects. r[agent_harness_runtime.aeneas_proof_campaign.host_boundary]
- [x] [serial] Emit metadata-only campaign receipts and require a separate authoritative-completion gate for proved outcomes. r[agent_harness_runtime.aeneas_proof_campaign.receipts]
- [x] [parallel] Add negatives for budget bypass, outcome conflation, missing/changed service evidence, raw prompt/proof/tool/log leakage, and campaign-receipt completion promotion. r[agent_harness_runtime.aeneas_proof_campaign.outcomes] r[agent_harness_runtime.aeneas_proof_campaign.host_boundary] r[agent_harness_runtime.aeneas_proof_campaign.receipts]

## Phase 5: Focused rail and closeout

- [x] [serial] Add campaign documentation and a focused verification rail covering Nickel/manifest freshness, role separation, statement/trust adequacy, artifact boundaries, budgets/outcomes, injected services, safe receipts, and non-claims. r[agent_harness_runtime.aeneas_proof_campaign.verification]
- [x] [serial] Run the focused rail, relevant recipe/role/model/skill/safe-receipt tests, Nickel config checks, workspace checks, Cairn validation/gates, and record blockers plus next-best checks for unavailable external proof services. r[agent_harness_runtime.aeneas_proof_campaign.verification]
