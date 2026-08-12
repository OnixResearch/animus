# Agent Harness Runtime Change Specification: Aeneas Proof Campaign Recipe

## ADDED Requirements

### Requirement: Aeneas proof campaigns use a typed composed profile

r[agent_harness_runtime.aeneas_proof_campaign.profile] Animus MUST define a Nickel-authored `aeneas-proof-campaign` profile that composes existing recipe workflow, semantic role, per-model runtime, skill manifest, tool catalog, verification-service, named budget, output, and safe-receipt identities without creating a second agent runtime or granting authority through the profile itself.

#### Scenario: Complete campaign profile passes

- GIVEN a campaign profile references admitted exact component identities, finite budgets, output contracts, and non-claims
- WHEN profile validation and deterministic export run
- THEN Animus MUST produce a validated no-authority campaign profile.

#### Scenario: Campaign profile widens authority

- GIVEN a profile names unknown components, omits required budgets/outputs, broadens capabilities, embeds ambient shell authority, or weakens non-claims
- WHEN validation runs
- THEN validation MUST fail before campaign activation.

### Requirement: One canonical campaign skill source stays fresh

r[agent_harness_runtime.aeneas_proof_campaign.skill_source] Animus MUST maintain one compact outcome-first canonical Aeneas campaign skill source whose BLAKE3 identity is referenced by the Nickel profile and whose generated safe manifest/catalog projections are freshness-checked.

#### Scenario: Canonical skill and projections match

- GIVEN the canonical skill, profile identity, generated manifest, and catalog projection agree
- WHEN freshness validation runs
- THEN the skill MAY be progressively disclosed through existing trusted skill loading.

#### Scenario: Skill body or generated manifest drifts

- GIVEN a duplicate canonical body, stale projection, hidden extra directive, project-local replacement, or mismatched content identity exists
- WHEN freshness validation runs
- THEN activation MUST fail and runtime policy MUST NOT be replaced by prompt text.

### Requirement: Campaign roles are capability-bound and distinct

r[agent_harness_runtime.aeneas_proof_campaign.roles] The campaign MUST represent statement author, statement reviewer, prover, and verifier as narrow specializations of existing semantic role classes with exact assignment, capability, mutation, input, output, model, tool, and evidence ceilings.

#### Scenario: Valid role plan is composed

- GIVEN each assignment satisfies its semantic class, required resources, capability intersection, and output contract
- WHEN campaign planning runs
- THEN Animus MUST produce deterministic role-bound plans without minting missing authority.

#### Scenario: Role label or alias attempts bypass

- GIVEN an assignment relabels an implementer as reviewer/verifier, unions role capabilities, or requests mutation outside its class
- WHEN role admission runs
- THEN the assignment MUST be denied.

### Requirement: Review and verification separation is explicit

r[agent_harness_runtime.aeneas_proof_campaign.separation] Statement review and final verification MUST bind immutable subject identity and a reviewed separation policy over supplied assignment, actor, run/thread, context, workspace, model/profile, mutation-lineage, statement, model, and verifier/check identities, while making no unmeasured independence claim.

#### Scenario: Reviewed separation dimensions pass

- GIVEN the supplied observations satisfy every required distinct and immutable dimension
- WHEN separation evaluation runs
- THEN the review or verification assignment MAY activate with the exact separation decision bound.

#### Scenario: Self-review or mutable subject is detected

- GIVEN required dimensions overlap, the reviewer/verifier mutated the subject, inherited unreviewed mutable context, or a bound identity is stale
- WHEN separation evaluation runs
- THEN activation MUST fail with the exact failed dimensions.

### Requirement: Theorem statement approval precedes proof execution

r[agent_harness_runtime.aeneas_proof_campaign.statement_gate] The campaign MUST require a statement-review handoff binding requirement ID, approved theorem-statement BLAKE3, claim class, source/generated-model/type identities, adequacy/vacuity decision, and complete declared model/assumption/axiom/opaque-definition closure before a prover or proof service may execute.

#### Scenario: Approved non-vacuous statement proceeds

- GIVEN the reviewer accepts the exact statement and complete trust closure for the declared claim class
- WHEN the prover transition is evaluated
- THEN the campaign MAY request proof execution for those immutable identities.

#### Scenario: Statement or trust boundary is inadequate

- GIVEN the statement is vacuous, contains `sorry` or `admit`, omits a trust input, lacks requirement/claim identity, or changes after review
- WHEN proof execution or later verification is requested
- THEN the campaign MUST block and return to statement review or a typed invalid-statement outcome.

### Requirement: Generated and maintained artifacts stay separate

r[agent_harness_runtime.aeneas_proof_campaign.artifact_boundary] Campaign plans MUST classify generated translation artifacts and maintained theorem sources separately, require non-overlapping destinations, allow generation writes only to generated destinations, limit prover mutation to maintained destinations, and bind regeneration freshness before verification.

#### Scenario: Fresh generated model and maintained theorem proceed

- GIVEN generation identities and bytes match, destinations do not overlap, and prover edits are confined to maintained theorem source
- WHEN artifact-boundary validation runs
- THEN verification MAY consume the bound artifacts.

#### Scenario: Generated output is manually repaired or overlaps theorem source

- GIVEN an agent edits generated output, generation would overwrite maintained theorem source, or generated bytes are stale
- WHEN artifact validation runs
- THEN the campaign MUST block until reviewed regeneration produces a fresh binding.

### Requirement: Campaign budgets and terminal outcomes are typed

r[agent_harness_runtime.aeneas_proof_campaign.outcomes] The campaign MUST bind named finite proof-time, stage/tool-call, output, retrieval, log, and retained-artifact budgets and MUST distinguish proved, blocked, counterexample, invalid statement, invalid model, budget exhausted, tool failure, denied, cancelled, and abstained outcomes.

#### Scenario: Proved outcome has required evidence

- GIVEN every required stage passes within budget and exact verification evidence is present
- WHEN terminal-state evaluation runs
- THEN the campaign MAY emit `proved` as a candidate for a separate completion gate.

#### Scenario: Non-proved outcome is reported honestly

- GIVEN a blocker, counterexample, invalid input, exhausted budget, tool failure, denial, cancellation, or abstention occurs
- WHEN terminal-state evaluation runs
- THEN Animus MUST preserve the exact outcome and MUST NOT convert it into proved or authoritative completion.

### Requirement: Proof effects remain host-owned

r[agent_harness_runtime.aeneas_proof_campaign.host_boundary] Reusable campaign cores MUST operate only on already-loaded typed values; hosts MUST own provider dispatch, proof-tool execution, filesystem/process/network/credential/clock authority, workspace mutation, cancellation, measurement, and artifact storage through existing injected services and capability checks.

#### Scenario: Admitted service request is bound

- GIVEN an injected proof/evidence service and capability decision match the assignment, subject, statement, model, policy, and budget identities
- WHEN the host executes the request
- THEN it MAY return a typed observation for pure campaign validation.

#### Scenario: Reusable core attempts an effect or service drifts

- GIVEN reusable campaign code attempts ambient I/O or a service/result identity differs from the admitted request
- WHEN boundary validation runs
- THEN the operation MUST fail and no prior evidence status may be reused.

### Requirement: Campaign receipts are safe and non-authoritative

r[agent_harness_runtime.aeneas_proof_campaign.receipts] Animus MUST emit metadata-only campaign receipts bound to profile, skill, role, assignment, subject, statement, model, budget, stage, outcome, and evidence identities, bounded counts, issue classes, and redaction markers, and MUST keep authoritative completion as a separate host decision.

#### Scenario: Safe campaign receipt passes

- GIVEN a campaign reaches a typed terminal outcome
- WHEN receipt projection runs
- THEN the receipt MUST contain only bounded metadata and exact identities.

#### Scenario: Receipt leaks or promotes itself

- GIVEN a receipt includes raw prompts, hidden reasoning, credentials, private paths, source/proof bodies, tool arguments/output, provider payloads, unbounded logs, or claims authoritative completion
- WHEN receipt validation runs
- THEN validation MUST fail.

### Requirement: Proof campaign verification covers success and failure

r[agent_harness_runtime.aeneas_proof_campaign.verification] Before archive, the change MUST provide a focused rail and positive/negative fixtures for profile/skill freshness, role/separation policy, statement adequacy/trust closure, generated/maintained boundaries, budgets/outcomes, injected services, safe receipts, completion boundaries, documentation, and Cairn lifecycle gates.

#### Scenario: Focused campaign evidence is complete

- GIVEN the campaign change is ready for review
- WHEN focused and relevant existing rails run
- THEN every positive case MUST pass, every negative case MUST fail for its declared reason, and unavailable external proof services MUST be recorded as bounded blockers with next-best checks rather than simulated success.
