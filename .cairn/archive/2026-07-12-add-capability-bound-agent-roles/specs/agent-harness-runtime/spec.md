# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Typed capability-bound agent role catalog

r[agent_harness_runtime.agent_role_profiles.contract] Animus MUST define a versioned Nickel-authored agent-role catalog with semantic role ID and class, purpose, source/trust/admission metadata, required and ceiling capability references, eligible model-runtime profiles, prompt/skill/toolset requests, workspace/context/mutation posture, input/output/evidence contracts, separation-policy reference, named finite budgets, policy identity, and BLAKE3 profile identity.

#### Scenario: Baseline and extension roles validate deterministically

r[agent_harness_runtime.agent_role_profiles.contract.nickel]
- GIVEN the catalog declares reviewed implementer, reviewer, verifier, and judge profiles plus additional roles using supported implementation, review, verification, judgment, coordination, or observation classes
- WHEN Nickel validation, deterministic export, and Rust DTO validation run
- THEN every accepted profile MUST preserve the same typed fields, class invariants, references, and BLAKE3 identity, with every required capability also present in that profile's ceiling set
- AND duplicate IDs, unknown classes, contradictory postures, required capabilities outside the ceiling, missing references, unsafe labels, unadmitted project profiles, or export drift MUST fail before assignment.

### Requirement: Semantic roles remain distinct from routing and presentation roles

r[agent_harness_runtime.agent_role_profiles.semantic_boundary] Semantic agent roles MUST remain distinct typed data from router recipe roles, provider/model roles, agent-definition names, prompt modes, workflow step kinds, and UI labels, and any mapping between those axes MUST be explicit, validated, and non-authoritative.

#### Scenario: Router judge does not imply semantic judge authority

r[agent_harness_runtime.agent_role_profiles.semantic_boundary.router_judge]
- GIVEN a router execution plan contains a `Judge` step
- WHEN runtime prepares the selected model invocation
- THEN semantic judge behavior MUST require a separately admitted agent-role assignment and compatible role/model plan
- AND the router role alone MUST NOT grant judge capabilities, evidence status, tool access, veto, selection, or completion authority.

### Requirement: Role assignment uses scoped capability admission

r[agent_harness_runtime.agent_role_profiles.activation] Agent-role assignment, activation, handoff, candidate/report/evidence emission, and judgment selection/veto MUST use typed guarded capability requests bound to role resource, subject/task, actor/parent lineage, agent definition, model plan, policy/grant, separation, freshness, and output-contract identities before the role can influence prompt, model, tool, workspace, output, or evidence behavior.

#### Scenario: Activation grant does not grant role actions

r[agent_harness_runtime.agent_role_profiles.activation.no_minting]
- GIVEN a caller has a valid grant to activate the reviewer role for one subject
- AND it lacks a grant for a requested model, tool, filesystem, process, network, Steel, workspace, mutation, external-service, review-report, verification-evidence, or judgment-selection action
- WHEN role activation and later action or output admission run
- THEN activation MAY produce a reviewer assignment but the underlying action or output influence MUST remain denied
- AND no role label or activation receipt may be used as a substitute capability grant.

#### Scenario: Wrong subject or stale assignment fails closed

r[agent_harness_runtime.agent_role_profiles.activation.binding]
- GIVEN an activation or handoff request cites a different subject, actor lineage, role profile, model plan, policy, grant, separation policy, output contract, or freshness identity than the admitted request
- WHEN role admission runs
- THEN it MUST return a typed binding or stale-assignment denial
- AND it MUST NOT select a fallback role, broader grant, or ambient profile.

### Requirement: Effective role capabilities are an exact intersection

r[agent_harness_runtime.agent_role_profiles.capability_intersection] Effective capabilities for a role assignment MUST be the pure deterministic intersection of verified grants and Nickel contracts, host/session policy and caveats, parent delegation and task scope, admitted agent-definition ceilings, semantic role ceilings and requirements, admitted per-model runtime profile ceilings, workspace/context scope, tool/executor availability, approval, sandbox, cancellation, timeout, and named budgets.

#### Scenario: Missing required role capability denies assignment

r[agent_harness_runtime.agent_role_profiles.capability_intersection.required]
- GIVEN a verifier profile declares a required capability that is also in its role ceiling but is absent from any stronger grant, policy, parent, task, model, workspace, or host-admission input
- WHEN capability resolution runs
- THEN role activation MUST be denied with the missing ceiling sources identified safely
- AND “required” MUST remain an eligibility condition and MUST NOT synthesize the capability from the role profile.

#### Scenario: Multiple roles do not union authority

r[agent_harness_runtime.agent_role_profiles.capability_intersection.no_union]
- GIVEN one actor has separate implementer and reviewer assignments with different capability ceilings
- WHEN runtime resolves either assignment or aggregates their outputs
- THEN each plan MUST retain only its own intersection and immutable assignment identity
- AND capabilities, tools, grants, workspaces, approvals, or budgets from the two roles MUST NOT be unioned.

### Requirement: Semantic role classes enforce bounded behavior

r[agent_harness_runtime.agent_role_profiles.role_invariants] Animus MUST enforce semantic class invariants for implementation, review, verification, judgment, coordination, and observation profiles before assignment and before each role-bound action or output is admitted.

#### Scenario: Implementer cannot self-attest independent evidence

r[agent_harness_runtime.agent_role_profiles.role_invariants.implementer]
- GIVEN an implementation assignment may request scoped mutations for a subject
- WHEN it emits output or requests another capability
- THEN its output MUST remain an artifact/change candidate, unresolved intent, or validation request subject to normal mutation and completion gates
- AND it MUST NOT be admitted as independent review, verification, judgment, or authoritative completion evidence.

#### Scenario: Reviewer remains subject-read-only and tighten-only

r[agent_harness_runtime.agent_role_profiles.role_invariants.reviewer]
- GIVEN a review assignment receives a pinned subject and bounded evidence
- WHEN it requests actions or emits a review result
- THEN it MAY inspect admitted context and emit findings, warnings, blockers, recommendations, uncertainty, or abstention according to policy
- AND it MUST NOT mutate the subject, grant authority, suppress deterministic findings, lower hard risk, bypass confirmation/sandbox, or mark completion verified.

#### Scenario: Verifier cannot mutate subject or verifier

r[agent_harness_runtime.agent_role_profiles.role_invariants.verifier]
- GIVEN a verification assignment names a subject, check/verifier identity, oracle or expected-result identity, evidence policy, and gate contract
- WHEN it requests a check or emits verification evidence
- THEN only an admitted host verification service MAY execute the named check and return evidence for the pinned identities
- AND the assignment MUST NOT mutate the subject, verifier/check, oracle, expected result, evidence policy, gate policy, or receipt into a pass.

#### Scenario: Judge cannot grant authority or fabricate verification

r[agent_harness_runtime.agent_role_profiles.role_invariants.judge]
- GIVEN a judgment assignment receives bounded candidates, evidence references, and exactly one typed rubric identity or one prevalidated deterministic composite-rubric identity
- WHEN it ranks, selects, vetoes, ties, or abstains
- THEN its result MUST stay within the host-authorized judgment function and preserve deterministic denials
- AND it MUST NOT mix undeclared rubrics, grant capabilities, mutate candidates, suppress hard findings, fabricate check evidence, expose hidden rationale, or claim authoritative completion.

#### Scenario: Coordination and observation stay bounded

r[agent_harness_runtime.agent_role_profiles.role_invariants.other]
- GIVEN a coordination or observation profile is assigned
- WHEN it plans, delegates, aggregates, observes, or reports
- THEN coordination MUST use explicit service/capability contracts and observation MUST remain read/report-only
- AND neither class MUST inherit child capabilities, apply child mutations, or request undeclared effect authority.

### Requirement: Role resources compose with per-model runtime profiles

r[agent_harness_runtime.agent_role_profiles.resource_composition] Role-requested prompts, skills, toolsets, model/runtime profiles, context, output contracts, and budgets MUST compose as ceilings within the per-model invocation plan, and dispatch MUST require a compatible intersection with agent definition, task/recipe, selected model capabilities, host policy, tool catalog, and executor admission.

#### Scenario: Reviewer and implementer receive different effective plans

r[agent_harness_runtime.agent_role_profiles.resource_composition.distinct]
- GIVEN reviewer and implementer assignments target the same task but request different admitted role resources
- WHEN each selected model invocation plan is composed
- THEN the reviewer plan MUST exclude implementer-only mutation tools and prompts while the implementer plan retains only separately authorized implementation resources
- AND shared model selection MUST NOT make the plans or capabilities equivalent.

#### Scenario: Fallback model cannot downgrade role requirements

r[agent_harness_runtime.agent_role_profiles.resource_composition.fallback]
- GIVEN router fallback selects a model whose runtime profile cannot satisfy the active role's required capability, tool-call, context, or output contract
- WHEN fallback composition runs
- THEN dispatch MUST remain blocked or select another already-authorized compatible target according to router policy
- AND it MUST NOT weaken the semantic role, borrow prior-model resources, or silently change output claims.

### Requirement: Separation-of-duty claims use explicit identity policy

r[agent_harness_runtime.agent_role_profiles.separation] Any claim of independent review, verification, or judgment MUST pass a typed separation policy over host-supplied actor, assignment, run/thread, session, model/profile, workspace, mutation lineage, subject, verifier/check, rubric, and policy identities, and a role label by itself MUST NOT establish independence.

#### Scenario: Implementer lineage cannot satisfy independent verification

r[agent_harness_runtime.agent_role_profiles.separation.self_verification]
- GIVEN an actor or assignment lineage mutated the subject and the active separation policy requires distinct mutation lineage for verification
- WHEN that lineage attempts to activate verifier or submit independent verification evidence for the subject
- THEN separation validation MUST deny the independent claim
- AND changing only the prompt, display role, or router role MUST NOT satisfy the policy.

#### Scenario: Explicitly separated assignment passes bounded policy

r[agent_harness_runtime.agent_role_profiles.separation.distinct]
- GIVEN a reviewer, verifier, or judge assignment supplies identities satisfying every dimension required by the selected separation policy
- WHEN separation validation runs
- THEN it MAY return a bounded policy-pass result tied to those exact identities
- AND the result MUST NOT claim human independence, model diversity, lack of correlation, or correctness beyond the checked dimensions.

### Requirement: Role outputs have typed claim boundaries

r[agent_harness_runtime.agent_role_profiles.outputs] Every role output MUST use an assignment- and subject-bound envelope, pass its matching role-output capability admission, and conform to role-specific implementation, review, verification, judgment, coordination, or observation payload contracts with typed denied, unavailable, timed-out, cancelled, malformed, conflict, and abstained outcomes where applicable.

#### Scenario: Output subject mismatch is rejected

r[agent_harness_runtime.agent_role_profiles.outputs.subject_binding]
- GIVEN a role output cites a subject, assignment, role profile, model plan, capability policy, separation policy, verifier, rubric, or evidence identity different from its admitted plan
- WHEN output validation runs
- THEN the output MUST be rejected as stale or mismatched
- AND it MUST NOT influence mutation, review state, gate satisfaction, judgment, continuation, or completion.

#### Scenario: Missing output capability prevents influence

r[agent_harness_runtime.agent_role_profiles.outputs.capability]
- GIVEN a valid assignment lacks the scoped candidate, report, verification-evidence, judgment-report, selection, or veto capability required by its attempted output
- WHEN output admission runs
- THEN the output MUST be denied or retained as non-influential untrusted data according to policy
- AND role activation alone MUST NOT authorize review state, gate evidence, candidate selection, veto, continuation, or completion changes.

#### Scenario: Role output cannot bypass completion authority

r[agent_harness_runtime.agent_role_profiles.outputs.completion_boundary]
- GIVEN implementer, reviewer, verifier, or judge output claims the task or repository is complete
- WHEN completion admission runs
- THEN the claim MUST remain bounded role output or supplied check evidence
- AND only the separately supported authoritative-completion policy and current matching receipts MAY establish verified completion.

### Requirement: Role transitions bind immutable plans

r[agent_harness_runtime.agent_role_profiles.transition_binding] Assignment, activation, role transition, handoff, fallback, resume, replay, cancellation, returned tool calls, and role outputs MUST bind an immutable role-plan identity, and any change to role, subject, actor lineage, model plan, policy, grants, separation, prompts, skills, tools, budgets, or output contract MUST require explicit replanning.

#### Scenario: Implementer authority does not survive reviewer transition

r[agent_harness_runtime.agent_role_profiles.transition_binding.drop_authority]
- GIVEN a session transitions from an implementer assignment to reviewer, verifier, or judge
- WHEN the new role plan is composed
- THEN prior role-specific capabilities, mutation intents, prompt sources, skill bodies, tool declarations, pending calls, approvals, budgets, grants, hidden reasoning, and raw role transcript material MUST be dropped or separately re-admitted, while explicitly shared subject/evidence records are revalidated for the new role
- AND stale implementer calls or outputs MUST be rejected under the new role identity.

#### Scenario: Handoff carries evidence, not ambient authority

r[agent_harness_runtime.agent_role_profiles.transition_binding.handoff]
- GIVEN one role hands work to another role
- WHEN the handoff is validated
- THEN it MAY carry bounded subject identity, evidence references, findings, unresolved questions, and requested next actions
- AND it MUST NOT carry raw transcripts, hidden reasoning, credentials, unbounded logs, inherited grants, approvals, pending tool calls, or mutation authority.

### Requirement: Agent-role receipts are safe evidence

r[agent_harness_runtime.agent_role_profiles.receipts] Agent-role profile, assignment, capability-resolution, separation, transition, handoff, and output receipts MUST contain only bounded safe role/class labels or hashes, assignment/subject/actor/lineage identities, model/profile/policy/grant/separation identities, effective and denied capability summaries, mutation/evidence/output postures, status classes, counts, redaction markers, and BLAKE3 receipt identity.

#### Scenario: Receipt rejects raw role context and hidden evidence

r[agent_harness_runtime.agent_role_profiles.receipts.rejects_leaks]
- GIVEN a role receipt contains raw prompts or instructions, hidden reasoning, provider payloads, credentials, tokens, private/full paths, verifier bodies, hidden tests or oracles, raw diffs/files, tool arguments/output, judge rationales, or unbounded transcripts
- WHEN receipt validation runs
- THEN validation MUST reject the receipt with a deterministic leak diagnostic
- AND no assignment, capability, separation, evidence, judgment, or completion claim may rely on that unsafe receipt.

### Requirement: Capability-bound roles have positive and negative verification

r[agent_harness_runtime.agent_role_profiles.verification] Capability-bound agent role work MUST include focused positive and negative tests for Nickel/Rust catalog parity, semantic-role separation from router/model roles, assignment authorization, exact capability intersection, non-union, class invariants, role/model composition, separation of duty, output claims, transitions/handoffs, safe receipts, docs, component/conformance inventories, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers role and capability behavior

r[agent_harness_runtime.agent_role_profiles.verification.closeout]
- GIVEN maintainers intend to sync and archive capability-bound agent roles
- WHEN closeout validation runs
- THEN evidence MUST name role-catalog, capability, assignment, implementer, reviewer, verifier, judge, coordination/observation, model-composition, separation, transition, output, receipt, Nickel, workspace, conformance, facade, Cairn validation, and proposal/design/tasks gate checks
- AND every skipped or unavailable rail MUST record a concrete blocker and next-best validation without claiming role competence, independence, verifier soundness, judge correctness, completion, or release readiness.
