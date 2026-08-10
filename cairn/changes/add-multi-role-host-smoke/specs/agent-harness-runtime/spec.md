# agent-harness-runtime Change Spec: Capability-bound multi-role host smoke

## ADDED Requirements

### Requirement: Packaged multi-role smoke composes accepted contracts

r[agent_harness_runtime.multi_role_host_smoke.contract] Animus MUST provide a non-default packaged smoke that uses accepted role, model-profile, capability, separation, output, receipt, and completion-boundary contracts.

#### Scenario: Explicit policy paths build the bounded workflow

r[agent_harness_runtime.multi_role_host_smoke.contract.explicit]
- GIVEN an operator supplies explicit generated role and model-profile paths plus one bounded subject label
- WHEN the packaged smoke validates its inputs
- THEN it MUST load both projections without ambient discovery
- AND it MUST run without provider, tool, credential, network, clock, process, or storage effects.

#### Scenario: Invalid configuration fails before activation

r[agent_harness_runtime.multi_role_host_smoke.contract.failure]
- GIVEN a required argument or generated projection is missing, malformed, stale, unsupported, or unsafe
- WHEN smoke admission runs
- THEN it MUST return a stable bounded failure before role activation
- AND it MUST NOT print raw policy content, private paths, subject text, role output, or diagnostics.

### Requirement: Packaged smoke activates four exact semantic roles

r[agent_harness_runtime.multi_role_host_smoke.activation] The packaged smoke MUST compose and activate implementer, reviewer, verifier, and judge assignments over one immutable subject through exact scoped authority.

#### Scenario: Each role receives an exact model-bound assignment

r[agent_harness_runtime.multi_role_host_smoke.activation.pass]
- GIVEN the loaded catalogs contain the reviewed four-role cohort and eligible model profiles
- WHEN the coordinator builds each assignment
- THEN every role MUST bind its exact role profile, model profile, subject, actor, lineage, policy, separation, output contract, and assignment identity
- AND assignment and activation MUST use distinct exact scoped grants.

#### Scenario: Missing role or stale model binding fails closed

r[agent_harness_runtime.multi_role_host_smoke.activation.failure]
- GIVEN a required role is absent or its selected model profile is missing, stale, ineligible, or outside its resource ceiling
- WHEN role composition runs
- THEN the workflow MUST stop before output admission
- AND it MUST NOT infer a fallback role or model profile.

### Requirement: Multi-role smoke enforces capability and separation boundaries

r[agent_harness_runtime.multi_role_host_smoke.separation] The packaged smoke MUST intersect every role capability source and evaluate explicit separation for reviewer, verifier, and judge assignments.

#### Scenario: Independent role contexts pass named dimensions

r[agent_harness_runtime.multi_role_host_smoke.separation.pass]
- GIVEN reviewer, verifier, and judge observations use distinct assignment, actor, run, session, model-profile, workspace, and mutation-lineage identities with clean immutable context
- WHEN separation evaluation runs
- THEN each required decision MUST pass for the exact checked dimensions
- AND no broader human, organizational, statistical, or global independence claim is created.

#### Scenario: Collapsed reviewer identity blocks activation

r[agent_harness_runtime.multi_role_host_smoke.separation.failure]
- GIVEN the reviewer actor identity equals the subject-origin actor identity
- WHEN reviewer separation evaluation runs
- THEN the reviewer assignment MUST remain blocked
- AND later verifier and judge outputs MUST not run.

### Requirement: Multi-role outputs require separate authority

r[agent_harness_runtime.multi_role_host_smoke.outputs] The packaged smoke MUST admit implementation candidate, review finding, verification evidence, and judge veto outputs only through separate exact output grants.

#### Scenario: Four typed outputs are admitted

r[agent_harness_runtime.multi_role_host_smoke.outputs.pass]
- GIVEN every role assignment, capability decision, separation decision, evidence identity, verifier identity, and rubric identity matches
- WHEN separate output authorization runs
- THEN the smoke MUST admit candidate, finding, check-evidence, and veto effects
- AND each output MUST remain bound to its exact assignment and immutable subject.

#### Scenario: Denied veto authority blocks judge output

r[agent_harness_runtime.multi_role_host_smoke.outputs.failure]
- GIVEN the judge has valid activation but no exact veto grant
- WHEN veto output admission runs
- THEN the veto MUST be denied
- AND activation authority MUST not substitute for output authority.

### Requirement: Role output is not authoritative completion

r[agent_harness_runtime.multi_role_host_smoke.completion_boundary] The packaged smoke MUST prove that reviewer mutation and authoritative-completion claims remain outside semantic role authority.

#### Scenario: Reviewer mutation remains denied

r[agent_harness_runtime.multi_role_host_smoke.completion_boundary.mutation]
- GIVEN the reviewer role ceiling omits mutation authority
- WHEN the coordinator attempts reviewer mutation capability use
- THEN the attempt MUST fail
- AND no independent ceiling may be unioned to mint mutation authority.

#### Scenario: Judge output cannot become completion

r[agent_harness_runtime.multi_role_host_smoke.completion_boundary.completion]
- GIVEN the judge emits an admitted rubric-bound veto
- WHEN the role layer attempts an authoritative-completion claim and completion-candidate admission
- THEN both attempts MUST fail without a matching separate completion request and Cairn-gated evidence
- AND the smoke MUST record `authoritative_completion=false`.

### Requirement: Multi-role smoke evidence is metadata-only and head-bound

r[agent_harness_runtime.multi_role_host_smoke.receipt] The packaged smoke MUST emit bounded metadata-only role records with BLAKE3 identities, explicit non-claims, and current-head metadata when Git is available.

#### Scenario: Successful output omits role content

r[agent_harness_runtime.multi_role_host_smoke.receipt.safe]
- GIVEN the four-role workflow succeeds
- WHEN the smoke renders its result
- THEN it MUST include role, profile, model-plan, assignment, separation, capability-count, output, and receipt identities
- AND it MUST omit subject text, prompts, role output, verifier bodies, judge rationale, credentials, paths, provider payloads, and transcripts.

#### Scenario: Focused receipt binds source and current head

r[agent_harness_runtime.multi_role_host_smoke.receipt.current_head]
- GIVEN the focused rail passes in a Git worktree
- WHEN the rail writes its receipt
- THEN it MUST record the tested source BLAKE3 identity, current commit, and dirty status
- AND a receipt from a dirty worktree MUST NOT be described as clean current-head evidence.

#### Scenario: Stronger behavior remains a non-claim

r[agent_harness_runtime.multi_role_host_smoke.receipt.non_claims]
- GIVEN the deterministic packaged smoke passes
- WHEN dogfood readiness documentation records the evidence
- THEN it MAY claim packaged capability-bound role activation and output separation
- AND it MUST retain provider quality, verifier soundness, human independence, mutation execution, authoritative completion, TUI, daemon, and production behavior as non-claims.

### Requirement: Multi-role smoke has focused verification

r[agent_harness_runtime.multi_role_host_smoke.verification] The change MUST include positive and negative tests for explicit loading, role composition, capability intersection, separation, output authorization, reviewer mutation, judge veto, completion overclaim, completion-candidate admission, receipt projection, and content omission.

#### Scenario: Focused rail covers admitted and rejected paths

r[agent_harness_runtime.multi_role_host_smoke.verification.rail]
- GIVEN the repository contains the packaged smoke and its fixtures
- WHEN the focused script and Nix check run
- THEN they MUST exercise the successful four-role workflow and required negative paths
- AND they MUST run without provider credentials or network access.

### Requirement: Multi-role smoke closeout is evidence-bound

r[agent_harness_runtime.multi_role_host_smoke.closeout] Maintainers MUST run focused role, completion, packaged-smoke, Nix, dogfood, and Cairn checks before they archive the change.

#### Scenario: Closeout records bounded evidence

r[agent_harness_runtime.multi_role_host_smoke.closeout.pass]
- GIVEN implementation, tests, docs, and lifecycle artifacts are complete
- WHEN closeout validation runs
- THEN the selected focused checks and Cairn gates MUST pass
- AND the evidence MUST state that no provider execution or authoritative completion occurred.
