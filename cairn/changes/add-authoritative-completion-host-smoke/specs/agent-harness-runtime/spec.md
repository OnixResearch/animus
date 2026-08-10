# agent-harness-runtime Change Spec: Packaged authoritative-completion host smoke

## ADDED Requirements

### Requirement: Packaged completion smoke composes accepted contracts

r[agent_harness_runtime.authoritative_completion_host_smoke.contract] Animus MUST provide a non-default packaged smoke that composes accepted semantic-role, model-profile, check-selection, execution-evidence, Cairn-summary, and authoritative-completion contracts.

#### Scenario: Explicit fixture inputs build the bounded host

r[agent_harness_runtime.authoritative_completion_host_smoke.contract.explicit]
- GIVEN an operator supplies explicit role-policy, model-policy, check-registry, subject, and head-state values
- WHEN the packaged smoke validates its inputs
- THEN it MUST run without provider, credential, network, clock, process-execution, ambient Git, or external storage authority
- AND it MUST use deterministic injected services for all host observations.

### Requirement: Role output remains advisory

r[agent_harness_runtime.authoritative_completion_host_smoke.role_boundary] The packaged completion smoke MUST reuse the four-role workflow while keeping every semantic-role output separate from completion authority.

#### Scenario: Judge selection permits separate completion evaluation

r[agent_harness_runtime.authoritative_completion_host_smoke.role_boundary.select]
- GIVEN implementer, reviewer, verifier, and judge assignments pass exact role and model-profile composition
- AND the judge emits an admitted rubric-bound selection
- WHEN the completion host receives the advisory role summary
- THEN it MAY continue to separate completion-candidate and selected-check evaluation
- AND the advisory summary MUST carry no passing completion authority.

#### Scenario: Judge veto blocks candidate admission

r[agent_harness_runtime.authoritative_completion_host_smoke.role_boundary.veto]
- GIVEN the same four-role workflow emits an admitted rubric-bound veto
- WHEN the completion host evaluates the role summary
- THEN it MUST stop before completion-candidate admission
- AND later supplied check or Cairn facts MUST NOT override the veto.

#### Scenario: Role output cannot self-attest completion

r[agent_harness_runtime.authoritative_completion_host_smoke.role_boundary.self_attestation]
- GIVEN a role output or advisory receipt hash is used as the completion request
- WHEN candidate admission runs
- THEN it MUST reject the request
- AND completion-looking role text or hashes MUST NOT substitute for the exact typed request.

### Requirement: Packaged completion smoke loads exact generated policy

r[agent_harness_runtime.authoritative_completion_host_smoke.loading] The packaged smoke MUST load role, model-profile, and check-registry projections only from explicit paths and MUST revalidate each projection before composition.

#### Scenario: Exact generated projections load

r[agent_harness_runtime.authoritative_completion_host_smoke.loading.pass]
- GIVEN the reviewed generated projections exist at the supplied paths
- WHEN host loading runs
- THEN the smoke MUST bind their validated identities into later role, selection, and completion receipts.

#### Scenario: Missing or malformed projection fails closed

r[agent_harness_runtime.authoritative_completion_host_smoke.loading.failure]
- GIVEN any required projection is missing, malformed, stale, or unsupported
- WHEN host loading runs
- THEN the smoke MUST fail before completion evaluation
- AND it MUST NOT print policy content or private paths.

### Requirement: Packaged completion smoke uses explicit host services

r[agent_harness_runtime.authoritative_completion_host_smoke.host_services] The packaged smoke MUST exercise injected changed-artifact observation, final-subject hashing, selected-check execution, metadata persistence, and Cairn-validation services through bounded deterministic fixtures.

#### Scenario: Exact quick checks are selected and executed

r[agent_harness_runtime.authoritative_completion_host_smoke.host_services.pass]
- GIVEN one safe documentation artifact, exact Nix and Cairn capabilities, and a quick tier ceiling
- WHEN check selection and execution run
- THEN every selected check and prerequisite MUST receive one exact identity-bound result
- AND the execution evidence MUST contain exact pass-only coverage before Cairn validation.

#### Scenario: Dirty state blocks before selected checks

r[agent_harness_runtime.authoritative_completion_host_smoke.host_services.dirty]
- GIVEN the explicit host head observation is dirty
- WHEN completion orchestration starts
- THEN it MUST stop before candidate admission and selected-check execution
- AND it MUST NOT describe the result as current-head completion evidence.

### Requirement: Matching Cairn evidence is the only passing path

r[agent_harness_runtime.authoritative_completion_host_smoke.completion] The packaged smoke MUST produce `authoritative_completion=true` only when the exact typed candidate, final subject, complete pass-only selected results, and matching valid Cairn summary all agree.

#### Scenario: Exact supplied facts produce a verified receipt

r[agent_harness_runtime.authoritative_completion_host_smoke.completion.pass]
- GIVEN clean explicit host state, judge selection, exact pass-only selected results, and a matching authoritative Cairn summary
- WHEN final reduction runs
- THEN it MUST return a verified authoritative-completion receipt
- AND the receipt MUST bind candidate, registry, selection, policy, final-subject, evidence, and Cairn identities.

#### Scenario: Completion remains bounded to supplied facts

r[agent_harness_runtime.authoritative_completion_host_smoke.completion.non_claims]
- GIVEN the deterministic fixture verifies
- WHEN documentation states the evidence rung
- THEN it MAY claim packaged authoritative-completion contract composition for supplied identities
- AND it MUST NOT claim check-oracle soundness, honest external execution, release readiness, or production readiness.

### Requirement: Completion fixtures fail closed

r[agent_harness_runtime.authoritative_completion_host_smoke.failure] The packaged smoke MUST reject incomplete, stale, contradicted, or missing completion evidence with stable bounded issues.

#### Scenario: Missing selected result blocks

r[agent_harness_runtime.authoritative_completion_host_smoke.failure.missing_result]
- GIVEN one required check has no result
- WHEN execution evidence and completion reduction run
- THEN authoritative completion MUST remain false.

#### Scenario: Post-check subject drift blocks

r[agent_harness_runtime.authoritative_completion_host_smoke.failure.stale_subject]
- GIVEN the candidate or execution rows bind a prior subject identity
- WHEN final reduction uses a changed final subject
- THEN authoritative completion MUST remain false.

#### Scenario: Missing Cairn summary blocks

r[agent_harness_runtime.authoritative_completion_host_smoke.failure.missing_cairn]
- GIVEN the candidate and selected results otherwise pass
- WHEN no matching Cairn summary is supplied
- THEN authoritative completion MUST remain false.

### Requirement: Completion smoke evidence is metadata-only and head-bound

r[agent_harness_runtime.authoritative_completion_host_smoke.receipt] The packaged smoke MUST emit metadata-only BLAKE3 identities and the focused rail MUST emit current-head source evidence.

#### Scenario: Successful output omits sensitive and advisory content

r[agent_harness_runtime.authoritative_completion_host_smoke.receipt.safe]
- GIVEN the deterministic completion fixture passes
- WHEN output is rendered
- THEN it MUST include bounded role-advisory, registry, selection, final-subject, execution, Cairn, candidate, and completion identities
- AND it MUST omit subject text, role output, prompts, judge rationale, check output, commands, credentials, paths, provider payloads, and transcripts.

#### Scenario: Focused receipt binds clean current head

r[agent_harness_runtime.authoritative_completion_host_smoke.receipt.current_head]
- GIVEN the focused rail passes in a Git worktree
- WHEN it writes its receipt
- THEN it MUST record the tested source BLAKE3 identity, current commit, and dirty status
- AND a dirty worktree receipt MUST NOT be described as clean current-head evidence.

### Requirement: Completion smoke has focused verification

r[agent_harness_runtime.authoritative_completion_host_smoke.verification] The change MUST include positive and negative Rust fixtures, a focused script, a Nix check, bounded documentation, and a dogfood evidence rung.

#### Scenario: Focused rail covers admitted and rejected paths

r[agent_harness_runtime.authoritative_completion_host_smoke.verification.rail]
- GIVEN the repository contains the packaged smoke
- WHEN the focused script and Nix check run
- THEN they MUST exercise successful completion plus veto, self-attestation, dirty state, missing result, stale subject, missing Cairn, and malformed-policy paths
- AND they MUST run without credentials, network access, or external command execution by the smoke.

### Requirement: Completion smoke closeout is evidence-bound

r[agent_harness_runtime.authoritative_completion_host_smoke.closeout] Maintainers MUST run focused role, completion, packaged-smoke, clippy, Nix, dogfood, workspace, and Cairn checks before they archive the change.

#### Scenario: Closeout records bounded evidence

r[agent_harness_runtime.authoritative_completion_host_smoke.closeout.pass]
- GIVEN implementation, tests, documentation, and lifecycle artifacts are complete
- WHEN closeout validation runs
- THEN selected checks and Cairn gates MUST pass
- AND evidence MUST retain fake-service, oracle-soundness, external-execution, release, and production non-claims.
