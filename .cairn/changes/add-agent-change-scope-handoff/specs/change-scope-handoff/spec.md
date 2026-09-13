# Agent Change-Scope Handoff Specification

## ADDED Requirements

### Requirement: Change scopes are typed and bound

r[animus.change_scope_handoff.scope] Animus MUST record every agent change as a typed change scope that binds files, hunks, the driving task, and validation obligations, and MUST reject a scope that omits any of these facts.

#### Scenario: Complete scope facts are admitted

- GIVEN a caller supplies complete bounded in-memory scope facts
- WHEN scope validation runs
- THEN the core MUST return deterministic admission facts or typed blockers
- AND it MUST perform no file, process, or product effect.

#### Scenario: Scope omits the driving task

- GIVEN a change scope lists files and hunks without a driving task or obligations
- WHEN scope validation runs
- THEN the core MUST reject the scope.

#### Scenario: A label crosses a typed field boundary

- GIVEN two admitted scopes contain the same flattened labels but different file and obligation partitions
- WHEN the core derives their scope identities
- THEN the identities MUST differ
- AND a decision for either scope MUST be rejected for the other scope without changing its pending state.

#### Scenario: A scope fact changes

- GIVEN an admitted scope changes its task, ordered collection contents, collection counts, or any hunk field
- WHEN the core records a decision for the original scope against the changed scope
- THEN the core MUST reject the decision at the scope-identity boundary.

#### Scenario: A legacy scope identity answers a new scope

- GIVEN a scope identity uses the uncounted `animus.change-scope-handoff.scope.v1` encoding
- WHEN the decision attempts to answer a newly validated scope
- THEN the core MUST reject the decision
- AND fresh scope validation and review MUST precede completion.

### Requirement: Handoff is explicit and recorded

r[animus.change_scope_handoff.handoff] Every change scope MUST be presented as an explicit ownership handoff, and nothing MUST be marked complete until the recorded review decision accepts it; missing decisions MUST block completion.

#### Scenario: Developer records acceptance

- GIVEN a presented scope receives a recorded acceptance decision
- WHEN completion evaluation runs
- THEN the scope MAY be marked complete
- AND the completion MUST cite the recorded decision.

#### Scenario: No decision is recorded

- GIVEN a scope is presented but has no recorded review decision
- WHEN completion evaluation runs
- THEN completion MUST be blocked.

### Requirement: Review artifacts are bounded and hashed

r[animus.change_scope_handoff.artifacts] Review artifacts MUST be bounded diff and decision identities with hashes, and MUST exclude raw transcripts, tool outputs, prompts, and responses beyond declared bounds.

#### Scenario: Artifact candidate is within bounds

- GIVEN an artifact candidate carries only bounded diff and decision identities
- WHEN artifact validation runs
- THEN the artifact MAY persist.

#### Scenario: Artifact candidate packs raw output

- GIVEN an artifact candidate packs raw transcripts or tool outputs beyond bounds
- WHEN artifact validation runs
- THEN validation MUST reject or redact the payload before persistence.

### Requirement: Continuation is subset-bounded

r[animus.change_scope_handoff.continuation] After a recorded decision, agent continuation MUST operate only within the approved subset, and rejected items MUST open a fresh scope with their own handoff.

#### Scenario: Approved subset continues

- GIVEN a recorded decision approves a declared subset
- WHEN continuation admission runs
- THEN the agent MAY continue within that subset only.

#### Scenario: Rejected items resume

- GIVEN the decision rejects items outside the approved subset
- WHEN continuation admission runs
- THEN the rejected items MUST form a fresh scope
- AND the agent MUST NOT silently reapply them inside the approved scope.

### Requirement: Non-claims are preserved

r[animus.change_scope_handoff.nonclaims] Passing handoff evidence MUST NOT claim change correctness, VCS completion, policy enforcement, tool correctness, production readiness, or release eligibility.

#### Scenario: Handoff receipt is promoted to change correctness

- GIVEN a passing handoff receipt is labeled as change correctness or policy enforcement
- WHEN non-claim validation runs
- THEN the evidence MUST fail.

### Requirement: The rail has positive and negative fixtures

r[animus.change_scope_handoff.fixtures] The handoff rail MUST include positive scope-shape, recorded-handoff, bounded-continuation, and artifact-identity fixtures plus negative malformed-scope, missing-decision, unbounded-continuation, artifact-leak, and overclaim fixtures.

#### Scenario: Rail is proposed for product use

- GIVEN scopes, decisions, artifacts, docs, and fixtures are complete
- WHEN focused Cargo, no-std, octet, Cairn, and Nix validation runs
- THEN every positive fixture MUST pass
- AND every negative fixture MUST fail at its declared boundary.
