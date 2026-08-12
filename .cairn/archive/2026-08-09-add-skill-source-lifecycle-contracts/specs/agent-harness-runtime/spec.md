## ADDED Requirements

### Requirement: Skill sources have an authority-free lifecycle contract

r[agent_harness_runtime.skill_source_lifecycle.contract] Animus MUST define bounded `Frozen` and `TrackedCandidate` source descriptors that bind a skill asset, safe source kind and origin labels, exact revision, BLAKE3 tree identity, manifest identity, bounds profile, policy, provenance, trust, observation, and lifecycle state without performing source effects or granting runtime authority.

#### Scenario: Valid descriptor is admitted

r[agent_harness_runtime.skill_source_lifecycle.contract.valid]
- GIVEN a descriptor contains the required mode-specific fields, nominal references, supported source kind, positive bounds, safe metadata, and valid BLAKE3 identities
- WHEN pure descriptor validation runs
- THEN Animus MUST return the canonical admitted descriptor without source discovery or mutation.

#### Scenario: Source descriptor is malformed or unsafe

r[agent_harness_runtime.skill_source_lifecycle.contract.invalid]
- GIVEN a descriptor has an unsupported mode, missing revision, invalid identity, missing bound, duplicate asset, source collision, raw path, private URL, credential, token, or unrestricted diagnostic
- WHEN pure descriptor validation runs
- THEN Animus MUST return a stable typed rejection before reconciliation or host mutation.

### Requirement: Active skill sources are frozen

r[agent_harness_runtime.skill_source_lifecycle.frozen] Every active source-lifecycle-aware skill manifest MUST bind an exact source revision, lowercase BLAKE3 tree identity, exact manifest content identity, canonicalization profile, bounds profile, policy, provenance, trust, and review references. A branch, tag, tracking label, or successful fetch MUST NOT replace these immutable bindings.

#### Scenario: Exact frozen source matches

r[agent_harness_runtime.skill_source_lifecycle.frozen.match]
- GIVEN the observed exact revision, canonical tree identity, manifest identity, bounds, policy, provenance, trust, and review facts match the frozen descriptor
- WHEN frozen-source reconciliation runs
- THEN Animus MAY classify the source as matching the active manifest.

#### Scenario: Named reference moves

r[agent_harness_runtime.skill_source_lifecycle.frozen.moved]
- GIVEN a branch or tag now resolves to a different revision or tree than the frozen descriptor
- WHEN frozen-source reconciliation runs
- THEN Animus MUST retain the active frozen identity
- AND it MUST NOT classify the new content as active or trusted.

### Requirement: Tracked sources produce inactive candidates only

r[agent_harness_runtime.skill_source_lifecycle.tracked_candidate] A tracked source observation MUST bind its tracking-label identity, exact observed revision, BLAKE3 tree identity, prior frozen identity when present, observation identity, freshness epoch, bounds, policy, provenance, and fixed outcome. It MUST produce only an inactive candidate or no-op and MUST NOT install, enable, select, trust, or replace a skill.

#### Scenario: Tracked source changes

r[agent_harness_runtime.skill_source_lifecycle.tracked_candidate.changed]
- GIVEN a fresh bounded observation has a new exact revision and tree identity under the same admitted tracking label
- WHEN tracked reconciliation runs
- THEN Animus MUST classify one inactive candidate bound to that exact observation
- AND the current frozen manifest MUST remain unchanged.

#### Scenario: Observation is stale or conflicting

r[agent_harness_runtime.skill_source_lifecycle.tracked_candidate.denied]
- GIVEN an observation is stale, incomplete, over-bound, conflicting, duplicated, or mismatched with its source and asset identities
- WHEN tracked reconciliation runs
- THEN Animus MUST block candidate admission without falling back to prior tool output or ambient source state.

### Requirement: Promotion is explicit and exact

r[agent_harness_runtime.skill_source_lifecycle.promotion] Candidate promotion MUST bind the current candidate, exact revision, BLAKE3 tree and manifest identities, review, policy, bounds, host observation, requested action, and capability label. A passing pure promotion decision MUST NOT mutate the host store and MUST remain subject to the existing fresh dry-run plan and injected mutation service.

#### Scenario: Current candidate is promoted

r[agent_harness_runtime.skill_source_lifecycle.promotion.allowed]
- GIVEN a current candidate matches exact reviewed content, policy, bounds, trust, host observation, action, and capability facts
- WHEN pure promotion admission runs
- THEN Animus MAY produce a frozen-manifest mutation request for the existing host service.

#### Scenario: Automatic activation is requested

r[agent_harness_runtime.skill_source_lifecycle.promotion.automatic-denied]
- GIVEN a tracked observation or candidate requests direct install, enablement, runtime selection, source replacement, or host mutation without exact current promotion facts
- WHEN admission runs
- THEN Animus MUST deny before any host effect.

#### Scenario: Candidate changes after review

r[agent_harness_runtime.skill_source_lifecycle.promotion.drift]
- GIVEN review or promotion evidence names an earlier candidate, revision, tree, manifest, policy, bounds, or host observation
- WHEN promotion admission runs against changed current facts
- THEN Animus MUST deny with a stable drift class.

### Requirement: Source lifecycle policy is typed Nickel

r[agent_harness_runtime.skill_source_lifecycle.policy] Allowed source kinds, safe label grammar, mode fields, canonical identity profiles, positive bounds, entry-type posture, source transitions, review requirements, promotion rules, diagnostics, and non-claims MUST be authored in a typed Nickel policy with a deterministic checked runtime projection.

#### Scenario: Valid policy projects deterministically

r[agent_harness_runtime.skill_source_lifecycle.policy.valid]
- GIVEN a policy declares closed source kinds, mode requirements, positive named bounds, canonicalization, entry-type posture, promotion requirements, diagnostics, and non-claims
- WHEN policy export and freshness checks run
- THEN the generated projection MUST match the reviewed Nickel source exactly
- AND Rust MUST consume it without runtime Nickel evaluation.

#### Scenario: Ambient or unsafe policy is rejected

r[agent_harness_runtime.skill_source_lifecycle.policy.invalid]
- GIVEN a policy allows ambient discovery, automatic activation, unrestricted paths, unrestricted source kinds, raw credentials, unbounded fetching, unknown entry types, or weakened non-claims
- WHEN Nickel and Rust policy validation runs
- THEN validation MUST fail before source reconciliation.

### Requirement: Source effects and activation authority remain host owned

r[agent_harness_runtime.skill_source_lifecycle.host_authority] Source configuration, raw locations, credential resolution, VCS and transport execution, archive handling, tree traversal, symlink and submodule policy, extraction, storage, clocks, cancellation, review, retention, rollback, and mutation MUST remain caller-owned host behavior outside reusable Animus cores.

#### Scenario: Host supplies bounded observation facts

r[agent_harness_runtime.skill_source_lifecycle.host_authority.injected]
- GIVEN a caller uses an authorized bounded source service and supplies typed revision, tree, path-admission, entry-type, bound, and observation facts
- WHEN Animus validates those facts
- THEN Animus MAY classify the source lifecycle without acquiring the service's effects or authority.

#### Scenario: Source service is absent

r[agent_harness_runtime.skill_source_lifecycle.host_authority.absent]
- GIVEN no source observation or mutation service is injected
- WHEN default, core, adapter, runtime, Steel, TUI, no-std, Wasm, and unrelated std profiles run
- THEN they MUST acquire no VCS, transport, archive, credential, filesystem, process, network, clock, source-store, or activation implementation
- AND source-dependent mutation MUST fail closed.

### Requirement: Skill-source evidence is metadata only

r[agent_harness_runtime.skill_source_lifecycle.evidence] Source observation, candidate, promotion, reconciliation, and mutation-admission receipts MUST contain only bounded mode, source-kind, safe-label, opaque-revision, BLAKE3 tree and manifest identity, asset, bounds, policy, provenance, trust, freshness, drift, action, capability, outcome, and fixed diagnostic metadata.

#### Scenario: Safe promotion receipt is produced

r[agent_harness_runtime.skill_source_lifecycle.evidence.safe]
- GIVEN a current promotion decision completes
- WHEN pure receipt construction runs
- THEN the receipt MUST bind the exact public decision facts and mandatory non-claims
- AND BLAKE3 MUST apply only to canonical public bytes.

#### Scenario: Receipt candidate contains source material

r[agent_harness_runtime.skill_source_lifecycle.evidence.leak]
- GIVEN a receipt candidate contains a raw URL, full path, credential, token, environment value, source byte, file body, prompt, script, VCS argument, stdout, stderr, tool output, or unrestricted error
- WHEN receipt validation runs
- THEN Animus MUST reject it before persistence or publication.

#### Scenario: Hash is presented as trust proof

r[agent_harness_runtime.skill_source_lifecycle.evidence.overclaim]
- GIVEN a revision and BLAKE3 tree identity match
- WHEN evidence claims source authenticity, authorship, review quality, script safety, authority, confinement, or release readiness
- THEN claim validation MUST reject the unsupported claim.

### Requirement: Skill-source lifecycle validation includes failures

r[agent_harness_runtime.skill_source_lifecycle.verification] Adoption MUST pair positive frozen, unchanged tracked, changed candidate, explicit promotion, legacy compatibility, policy, and safe-receipt fixtures with negative moving-reference, tree-mismatch, stale, duplicate, collision, bound, mode, path, entry-type, automatic-activation, promotion-drift, missing-service, partial-result, leak, and feature-isolation fixtures.

#### Scenario: Complete conformance corpus passes

r[agent_harness_runtime.skill_source_lifecycle.verification.complete]
- GIVEN the reviewed positive and negative adapter, policy, no-std, Wasm, component, contract, safe-receipt, and consumer corpus
- WHEN maintained source-lifecycle checks run
- THEN allowed cases MUST preserve frozen activation and explicit promotion
- AND denied cases MUST produce no source, store, runtime, or credential effect.

#### Scenario: Closeout evidence is incomplete

r[agent_harness_runtime.skill_source_lifecycle.verification.incomplete]
- GIVEN maintainers intend to archive the change without exact policy, fixture, no-std, Wasm, Cairn, traceability, consumer, or immutable-revision evidence
- WHEN closeout validation runs
- THEN readiness MUST remain blocked with the missing evidence or exact skipped-rail blocker.
