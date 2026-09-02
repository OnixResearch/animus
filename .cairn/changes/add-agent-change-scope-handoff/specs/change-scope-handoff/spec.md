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
