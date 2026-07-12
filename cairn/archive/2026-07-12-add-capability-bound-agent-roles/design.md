## Context

Animus currently uses several unrelated meanings of “role”:

- `animus-router::Role` orders provider-neutral recipe steps such as worker, synthesizer, judge, finalizer, repair, and fallback;
- agent definitions request model roles, prompt fragments, capability ceilings, and toolset ceilings;
- subagent delegation carries a free role label and a recipe/profile capability ceiling;
- review, verification, trust-boundary, and authoritative-completion contracts consume bounded evidence but do not share one role-assignment identity; and
- hosts may describe personas in prompts without an enforceable role policy.

The capability core already provides `CapabilityRequest`, Nickel-authored policy contracts, verified UCAN grants, local caveats, sandbox constraints, confirmations, tool admission, and safe receipts. The unified subagent facade already computes an intersection across parent policy, recipe/profile, workspace scope, and host admission. The missing step is to make semantic role a separate typed ceiling and bind role outputs and separation policy to the same subject and authority identities.

## Decisions

### 1. Semantic agent roles are a separate axis

**Choice:** Introduce `AgentRoleProfile` and `AgentRoleAssignment` as semantic policy data. They are distinct from router roles, model/provider roles, agent-definition names, prompt modes, workflow step kinds, and display labels.

A router judge step may request a semantic judge profile, but it does not receive that profile automatically. A semantic verifier may run on a router primary or worker step. Every mapping is explicit, validated, and included in the assignment identity.

**Rationale:** Equal role words do not imply equal authority, evidence meaning, or lifecycle behavior.

### 2. Nickel owns a role catalog with constrained classes

**Choice:** Add a versioned typed Nickel catalog whose profiles contain role ID, semantic class, purpose, source/trust/admission metadata, required and ceiling capability references, model-runtime-profile eligibility, prompt/skill/toolset requests, workspace and context posture, mutation posture, input/output/evidence contracts, separation-policy reference, named budgets, policy identity, and BLAKE3 profile identity.

The supported semantic classes are implementation, review, verification, judgment, coordination, and observation. The repo ships reviewed baseline profiles for `implementer`, `reviewer`, `verifier`, and `judge`. Additional role IDs must choose a supported class and satisfy that class's invariants; an unknown “custom” class cannot bypass them. Every required capability must also appear in that profile's ceiling set, so “required” means activation eligibility and never an implicit grant.

Hosts load and export Nickel. Reusable crates validate in-memory DTOs only and do not discover files, evaluate Nickel, inspect providers, load tools, or execute roles.

**Rationale:** Operators get extensibility without making arbitrary role labels an authority escape hatch.

### 3. Role activation uses the existing capability architecture

**Choice:** Extend the guarded-action vocabulary with an agent-role/delegation action class and Nickel-reviewed abilities for assignment, activation, handoff, implementation-candidate emission, review-report emission, verification-evidence emission, judgment-report emission, and narrower judgment selection/veto over scoped role and subject resources. An activation request binds role profile, subject/task, actor, parent lineage, agent definition, model-runtime profile, policy/grant, separation policy, and freshness identities.

A grant to assign or activate a role authorizes only that role transition. It does not grant any model, tool, filesystem, process, network, Steel, workspace, mutation, output, review, verification, or judgment capability listed by the role. Output/report/evidence/selection admission evaluates its own role-bound capability request.

**Rationale:** Role changes can alter visible prompts and tools, so they need admission, but assignment authority must not be confused with action authority.

### 4. Effective capabilities are an intersection, never a union

**Choice:** Add semantic role as an independent ceiling source in the existing pure capability resolver. Effective capabilities are the intersection of:

- verified UCAN grants and Nickel capability contracts;
- host/session policy and local caveats;
- parent delegation and task/recipe scope;
- admitted agent-definition ceilings;
- semantic role ceilings and required capabilities;
- admitted per-model runtime profile ceilings;
- workspace/context scope;
- tool catalog, executor, approval, sandbox, cancellation, timeout, and budget constraints.

Required role capabilities are eligibility checks, not grants. Validation rejects a required capability outside the role ceiling, and activation denies when any required capability is absent from the final intersection. Activating or stacking multiple roles does not union their capabilities; concurrent roles receive separate assignments and separate plans.

**Rationale:** “Reviewer plus implementer” must not accidentally become “all reviewer and implementer permissions.”

### 5. Baseline classes have enforceable invariants

**Choice:** Enforce class behavior in pure validation and admission:

- **Implementation** may request scoped mutation intents, but every mutation still requires normal authority, approval, sandbox, workspace, and executor admission. Its output is an artifact/change candidate and validation request, never independent review, verification, judgment, or completion evidence.
- **Review** is subject-read-only by default. It emits structured findings, uncertainty, evidence references, and recommendations. It may tighten, warn, block, or abstain according to policy, but cannot grant authority, suppress deterministic findings, or mutate the reviewed subject.
- **Verification** may request only named host verification services/check tools admitted for a pinned subject. It cannot mutate the subject, verifier/check definition, oracle/expected result, evidence policy, or gate policy. Its output records supplied check evidence and non-claims; it does not prove verifier soundness.
- **Judgment** consumes bounded candidate/evidence summaries under exactly one typed rubric identity or one prevalidated deterministic composite-rubric identity and may rank, select, veto, tie, or abstain according to explicit policy. It cannot mix undeclared rubrics, grant capabilities, override deterministic denials, fabricate verification, mutate candidates, or turn rationale into authority.
- **Coordination** may plan, delegate, cancel, aggregate, and hand off only through explicit service/capability contracts; it does not inherit child capabilities or apply child mutations.
- **Observation** is read/report-only and cannot request mutation or effectful execution.

**Rationale:** Role names become testable policy rather than prompt conventions.

### 6. Role resources compose with per-model runtime profiles

**Choice:** A role may request prompt sources, skills, toolsets, model/runtime profiles, output contracts, and named budgets. Those requests become another ceiling in the `add-per-model-runtime-profiles` composition plan. The effective model context and tools are the intersection of semantic role, model profile, agent definition, task/recipe, and host policy.

A fallback model retains the semantic assignment only when its model profile satisfies the role's required capabilities and output contract. It receives a freshly composed prompt/skill/tool plan. Missing compatibility blocks dispatch rather than downgrading the role or borrowing the prior model's resources.

**Rationale:** Role-specific behavior should reuse one context/tool composition path and remain correct under routing.

### 7. Separation of duty is explicit policy, not a label claim

**Choice:** Define `RoleSeparationPolicy` and pure checks over host-supplied actor, run/thread, session, model target/profile, workspace, mutation lineage, subject, verifier/check, rubric, and policy identities. Policies can require distinct assignment, clean context, no subject mutation lineage, distinct actor, distinct run/thread, distinct model/profile, or other reviewed combinations.

Baseline reviewer, verifier, and judge profiles forbid subject mutation within their assignment. When a workflow claims independent review, verification, or judgment, the named separation policy must pass. Reassigning the same model with a different prompt does not count as independent unless policy explicitly defines that dimension as sufficient. Receipts state observed dimensions and policy result without claiming human or statistical independence.

**Rationale:** Self-review and correlated judges are workflow facts that must be visible and fail closed when independence is required.

### 8. Role outputs have typed claim boundaries

**Choice:** Define a common role-output envelope bound to assignment, subject, role profile, model plan, policy, and evidence identities, with role-specific payload contracts:

- implementation candidate;
- review report;
- verification report; and
- judgment report.

Every contract supports denied, unavailable, timed-out, cancelled, malformed, conflict, and abstained outcomes where applicable. Output admission requires the matching role-bound candidate/report/evidence/judgment capability in addition to a valid assignment. Reviewer and judge recommendations are advisory/tighten-only unless a host policy and verified grant authorize a narrower selection or veto function. Verifier evidence may satisfy only the named check/gate contract for the pinned subject. No role output directly establishes authoritative agent completion; `receipt-authoritative-agent-completion` remains the separate final gate.

**Rationale:** A well-formed answer is not automatically an authoritative claim.

### 9. Role transitions and handoffs create new immutable plans

**Choice:** Assignment, activation, transition, handoff, resume, replay, and cancellation produce immutable role-plan identities. Changing role, subject, actor lineage, model plan, policy, grants, separation policy, prompt/skill/tool catalog, or output contract makes the old plan stale.

A role transition drops role-specific capabilities, prompt sources, skills, tool declarations, pending calls, approvals, and budgets before composing the new role. The new role context is rebuilt from admitted shared subject/evidence records; prior role-only prompt fragments, skill bodies, tool declarations, hidden reasoning, and raw transcript history are not retained merely because they were committed in the same session. Handoffs pass bounded evidence references and unresolved intents, not raw transcripts or inherited grants. Replay validates the historical role plan and never upgrades it against current broader policy.

**Rationale:** Implementer authority must not survive a switch to reviewer or verifier merely because the session continues.

### 10. Durable receipts remain metadata-only

**Choice:** Role receipts contain safe role/class labels, assignment and subject hashes, actor/lineage classes or hashes, profile/model/policy/grant/separation identities, effective and denied capability labels or hashes, mutation/evidence/output postures, bounded counts, status classes, redaction markers, and BLAKE3 receipt identity.

Receipts reject raw prompts, role instructions, hidden reasoning, provider payloads, credentials, tokens, private/full paths, verifier bodies, hidden tests/oracles, raw diffs/files, tool arguments/output, judge rationales, and unbounded transcripts.

**Rationale:** Operators need auditable role and capability decisions without turning receipts into sensitive context stores.

## Rejected Alternatives

- **Reuse router roles as semantic roles:** router roles order model execution and do not encode authority, subject scope, mutation posture, or evidence claims.
- **Put role rules only in prompts or skills:** prompt text cannot enforce capability, mutation, separation, or receipt policy.
- **Treat role activation as the role's authority grant:** this would let an assignment mint tool and host permissions.
- **Union capabilities for multi-role agents:** this defeats least privilege and makes separation-of-duty claims meaningless.
- **Hard-code only four role labels:** supported semantic classes and typed profiles allow planner, researcher, triager, coordinator, observer, or domain-specific variants without arbitrary semantics.
- **Let judge or reviewer approval override deterministic policy:** model recommendations remain below hard authority, monitor, sandbox, confirmation, and evidence gates.

## Risks / Trade-offs

- Role and model profiles add two composition axes. One pure resolver and explicit identities prevent hidden precedence.
- Separation policies depend on truthful host-supplied lineage identities. Animus can validate consistency, not prove real-world independence.
- Verification roles may need process or tool capabilities while remaining subject-immutable. Check-service descriptors and mutation-target policy must distinguish execution from subject mutation.
- Strict non-union and role-transition invalidation may require more explicit handoffs, but avoids capability leakage across long sessions.
- Baseline role outputs remain bounded evidence. They do not certify model competence, reviewer accuracy, verifier soundness, or judge correctness.
