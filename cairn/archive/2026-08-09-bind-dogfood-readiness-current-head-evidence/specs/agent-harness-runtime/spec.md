# agent-harness-runtime Change Spec: Dogfood current-head evidence binding

## ADDED Requirements

### Requirement: Packaged dogfood claims bind to current-head receipts

r[agent_harness_runtime.dogfood_readiness.current_head_binding] A release evidence index that claims `packaged-headless-dogfood` MUST record a clean head and a passing `packaged-headless-dogfood-smoke` check whose receipt matches that head. The claim MUST NOT require unrelated pure modes or imply live-provider, TUI, daemon, plugin, production, or Clankers-parity readiness.

#### Scenario: Current packaged smoke supports the bounded claim

r[agent_harness_runtime.dogfood_readiness.current_head_binding.current]
- GIVEN a release index records a clean head and a passing packaged-headless smoke receipt for that head
- WHEN release evidence validation checks the `packaged-headless-dogfood` claim
- THEN validation accepts the bounded packaged dogfood claim
- AND omitted optional surfaces remain explicit non-claims.

#### Scenario: Missing, stale, or dirty evidence blocks the claim

r[agent_harness_runtime.dogfood_readiness.current_head_binding.rejects_invalid]
- GIVEN a release index claims packaged-headless dogfood with no passing receipt, a stale receipt, or a dirty head
- WHEN release evidence validation runs
- THEN validation rejects the claim with a deterministic issue code
- AND the index remains draft or historical evidence only.

### Requirement: One dogfood rail validates claim hygiene and receipt binding

r[agent_harness_runtime.dogfood_readiness.current_head_binding.composed_rail] The dogfood readiness rail MUST validate roadmap claim hygiene and current-head release-index binding through one explicit operator command. The Rust validation cores MUST remain deterministic, and the shell MUST retain filesystem and process authority.

#### Scenario: One focused command tests both boundaries

r[agent_harness_runtime.dogfood_readiness.current_head_binding.composed_rail.operator]
- GIVEN the dogfood claim fixtures and release-index binding fixtures exist
- WHEN a maintainer runs `scripts/cairn/check-dogfood-readiness --root .`
- THEN the rail validates both fixture families
- AND any negative fixture that unexpectedly passes causes the rail to fail.

### Requirement: Dogfood receipt binding has positive and negative evidence

r[agent_harness_runtime.dogfood_readiness.current_head_binding.fixtures] The packaged dogfood receipt binding MUST include a positive clean-head fixture and negative fixtures for missing evidence, stale evidence, duplicate evidence, conflicting claim status, and dirty-head evidence.

#### Scenario: Fixture matrix detects regression

r[agent_harness_runtime.dogfood_readiness.current_head_binding.fixtures.matrix]
- GIVEN the release evidence validator changes
- WHEN the dogfood readiness rail runs the binding fixtures
- THEN the positive fixture passes
- AND each negative fixture reports its expected deterministic issue code.

### Requirement: Dogfood receipt binding closeout is evidenced

r[agent_harness_runtime.dogfood_readiness.current_head_binding.verification] The change MUST pass the dogfood readiness rail, release evidence rail, current-head generator rail, roadmap validation, Cairn validation, Cairn gates, and the focused Nix dogfood check before archive.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.dogfood_readiness.current_head_binding.docs]
- GIVEN maintainers review the completed change
- WHEN they inspect its tasks and evidence
- THEN the records name the focused positive and negative checks
- AND the records preserve live-provider, TUI, daemon, plugin, production, and Clankers-parity non-claims.
