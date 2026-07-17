## Context

Animus's accepted recipe workflow can describe typed activities, validation gates, subrecipes, outputs, retries, and trust. Capability-bound roles can constrain implementers, reviewers, verifiers, and judges with explicit separation. Skill manifests provide progressive disclosure, and focused rails provide deterministic positive/negative evidence. The missing surface is a canonical proof-campaign composition of these existing contracts.

The campaign consumes already-admitted Octet/Aeneas provider and evidence-service descriptors. Animus does not become a proof checker and does not embed Aeneas-specific process logic in reusable crates.

## Decisions

### 1. Compose existing contracts instead of adding a proof runtime

**Choice:** Define an `aeneas-proof-campaign` recipe profile that references existing recipe workflow, semantic role, per-model runtime, skill manifest, tool catalog, verification-service, budget, and safe-receipt identities. New pure DTOs cover only campaign-specific stage and handoff invariants.

**Rationale:** Proof work is a workflow specialization; authority and execution should remain in existing Animus boundaries.

### 2. Use typed role specializations with explicit separation

**Choice:** Map statement author and prover to narrow implementation-class profiles, statement reviewer to a read-only review profile, and final verifier to a verification profile bound to named Octet/Aeneas/Lean evidence services. Review and verification require immutable subject identity and reviewed distinct assignment/run/context/mutation-lineage dimensions. A role label never grants tools or proof authority.

**Rationale:** Writing a theorem and checking its statement are different mutation and evidence duties.

### 3. Approve the theorem before proof execution

**Choice:** The statement-review handoff must bind requirement ID, approved theorem-statement BLAKE3, claim class, exact source and generated-model identities, type/overflow model, declared assumptions/models/axioms/opaque definitions, and an adequacy/vacuity decision. The prover cannot silently change those fields; changes return to statement review.

**Rationale:** Tactic success over an unreviewed or vacuous theorem is not campaign success.

### 4. Separate generated and maintained artifacts

**Choice:** Campaign plans classify source, LLBC/manifest/generated definitions, maintained theorem source, review records, proof receipts, and logs by role. Generation requests may write only generated destinations. Prover mutation is limited to maintained theorem locations. Direct generated-file edits, output-path overlap, or stale generation identities block verification.

**Rationale:** Regeneration and proof maintenance need different ownership and freshness evidence.

### 5. Keep one canonical compact skill source

**Choice:** The repository owns one compact outcome-first Aeneas campaign skill source. A typed Nickel profile references its BLAKE3 identity and generated safe manifest projection. Drift checks reject duplicate canonical bodies, stale generated manifests/catalogs, hidden extra directives, or project-local prompt replacement. The skill omits Git-specific cloning, magic numeric thresholds, user-confirmation pauses, and upstream tactic folklore.

**Rationale:** One source prevents prompt/manifest drift while keeping runtime policy authoritative.

### 6. Budgets and terminal outcomes are first-class

**Choice:** The campaign binds named finite proof-time, stage/tool-call, output, retrieval, log, and retained-artifact budgets. Pure state transitions distinguish `proved`, `blocked`, `counterexample`, `invalid_statement`, `invalid_model`, `budget_exhausted`, `tool_failure`, `denied`, `cancelled`, and `abstained`. Only `proved` with required evidence can request completion; every other terminal outcome remains useful evidence without success promotion.

**Rationale:** Proof campaigns need bounded operation and honest failure semantics.

### 7. Host services own all effects and proof truth

**Choice:** Reusable cores validate already-loaded plans, observations, handoffs, and reports. Hosts own provider calls, proof-tool processes, filesystem/network/clock access, workspaces, credentials, cancellation, storage, and measurement. Verification consumes named service evidence and cannot claim verifier soundness.

**Rationale:** This preserves functional-core/imperative-shell boundaries and keeps Animus provider-neutral.

### 8. Receipts are metadata-only and completion remains separate

**Choice:** Campaign receipts bind profile/skill/role/assignment/subject/statement/model/budget/stage/outcome/evidence identities, bounded counts, issue classes, and redaction markers. They omit raw prompts, hidden reasoning, credentials, private paths, source/proof bodies, tool arguments/output, provider payloads, and unbounded logs. Authoritative completion remains a separate host gate.

**Rationale:** Workflow conformance is not theorem truth or task completion.

## Risks / Trade-offs

- Role separation is only as strong as supplied identities and policy dimensions; it does not prove human or statistical independence.
- A statement may pass mechanical adequacy checks and still encode the wrong product requirement. Receipts preserve the approved requirement and non-claim rather than asserting sufficiency.
- Proof providers evolve. Campaign profiles bind exact service/profile identities and fail closed on drift.
- Strong artifact separation may block convenient manual fixes. Regeneration is required to preserve evidence lineage.
