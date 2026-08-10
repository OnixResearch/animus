# Agent Harness Runtime

## ADDED Requirements

### Requirement: Request-context contributions use one typed authority-free contract

r[agent_harness_runtime.request_context_contributions.contract] Animus MUST define typed request-context contributions with nominal identity, source and trust classes, mandatory posture, priority, finite budget cost, payload identity, deduplication key, supersession set, optional conflict group, lifetime, sensitivity, and safe label without granting source or persistence authority.

#### Scenario: Valid contribution is admitted

r[agent_harness_runtime.request_context_contributions.contract.valid]
- GIVEN an existing prompt, skill, memory, project-context, MCP, or runtime owner supplies a complete bounded contribution
- WHEN pure contribution validation runs
- THEN Animus MUST return one canonical admitted contribution
- AND it MUST NOT load source data, persist payloads, call providers, or change the owner's authority decision.

#### Scenario: Contribution metadata is malformed

r[agent_harness_runtime.request_context_contributions.contract.invalid]
- GIVEN a contribution has missing identity, unknown class, unbounded cost, unsafe label, invalid lifetime, malformed conflict metadata, or mismatched payload identity
- WHEN validation runs
- THEN Animus MUST reject it before prompt assembly
- AND it MUST NOT infer defaults from source order or another contribution.

### Requirement: Contribution reduction is pure, deterministic, and order-independent

r[agent_harness_runtime.request_context_contributions.reducer] Contribution selection MUST validate all candidates before applying explicit supersession, deduplication, conflict, priority, and budget rules. Equal candidate sets MUST produce equal ordered plans regardless of enumeration order.

#### Scenario: Equivalent candidate orders are reduced

r[agent_harness_runtime.request_context_contributions.reducer.order_independent]
- GIVEN two inputs contain the same admitted contributions in different enumeration orders
- WHEN the reducer runs under the same policy and budget
- THEN both outputs MUST contain the same selected identities, ordering, omissions, diagnostics, and plan identity
- AND filesystem order, source order, or hash-map iteration MUST NOT affect selection.

#### Scenario: Mandatory contribution cannot fit

r[agent_harness_runtime.request_context_contributions.reducer.mandatory]
- GIVEN a mandatory contribution is missing, denied, conflicting, superseded without an admitted replacement, malformed, or over the reserved budget
- WHEN reduction runs
- THEN provider composition MUST fail closed with bounded diagnostics
- AND advisory priority MUST NOT displace or weaken the mandatory contribution.

#### Scenario: Advisory contributions exceed the remaining budget

r[agent_harness_runtime.request_context_contributions.reducer.advisory]
- GIVEN all mandatory contributions fit and advisory contributions exceed the remaining finite budget
- WHEN reduction runs
- THEN Animus MUST select advisory contributions by declared priority and canonical identity
- AND it MUST return bounded omission classes without exposing payload bodies.

### Requirement: Contribution plans integrate with existing model invocation composition

r[agent_harness_runtime.request_context_contributions.composition] Existing context owners MUST produce already-admitted contributions, and one reduced contribution plan MUST bind the exact model target, profile, prompt plan, policy, authority ceilings, and provider request before dispatch.

#### Scenario: Exact model invocation is composed

r[agent_harness_runtime.request_context_contributions.composition.bound]
- GIVEN current owners supply admitted values and the reducer returns one valid contribution plan
- WHEN model invocation composition runs
- THEN the invocation MUST bind the selected contribution-plan identity and exact payload identities
- AND the reducer MUST NOT replace prompt, skill, memory, MCP, tool, or authority validation.

#### Scenario: Target or profile changes

r[agent_harness_runtime.request_context_contributions.composition.drift]
- GIVEN model fallback, profile change, policy change, payload change, or authority change occurs after contribution reduction
- WHEN provider or tool continuation admission runs
- THEN Animus MUST require complete contribution and invocation recomposition
- AND no selected contribution may transfer to the new target by inheritance.

### Requirement: Selected skills bind one later provider response

r[agent_harness_runtime.request_context_contributions.skill_turn] Explicit skill selection MUST bind the current session, logical turn, provider target, model profile, manifest identity, body identity, selection identity, and one `UntilNextProviderResponse` contribution. Logical turn completion MUST wait for the correlated response or terminal failure.

#### Scenario: Selected skill is consumed by the next request

r[agent_harness_runtime.request_context_contributions.skill_turn.consumed]
- GIVEN a trusted skill is explicitly selected and the host loads the exact current body
- WHEN the next admitted provider request binds the selection and contribution identities
- THEN that request MAY include the body under current prompt and budget policy
- AND completion MUST remain blocked until the correlated provider response reaches a terminal observation.

#### Scenario: Skill body or target is stale

r[agent_harness_runtime.request_context_contributions.skill_turn.stale]
- GIVEN the body, manifest, target, profile, turn, policy, or selection identity differs before provider dispatch
- WHEN skill-turn admission runs
- THEN Animus MUST reject the contribution and require a new selection or explicit failure path
- AND it MUST NOT load another body, reuse an earlier body, or cross a model fallback.

#### Scenario: Skill selection result is returned

r[agent_harness_runtime.request_context_contributions.skill_turn.result]
- GIVEN a skill selection action succeeds
- WHEN its tool or host result is projected
- THEN the result MUST contain only safe skill metadata, identities, lifetime, and next-response requirement
- AND it MUST NOT contain the skill body, private references, scripts, or assets.

### Requirement: Contribution lifetimes and retention are explicit

r[agent_harness_runtime.request_context_contributions.retention] Animus MUST support explicit `CurrentRequest`, `UntilNextProviderResponse`, and `CurrentLogicalTurn` lifetimes. Request-local and sensitive payloads MUST NOT enter conversation, compaction, memory, quality, or receipt storage by default.

#### Scenario: Lifetime reaches its terminal observation

r[agent_harness_runtime.request_context_contributions.retention.expires]
- GIVEN a contribution is bound to one supported lifetime and its exact terminal request, response, or turn observation arrives
- WHEN lifetime reduction runs
- THEN Animus MUST expire the contribution deterministically
- AND later turns MUST NOT inherit it without a new admitted contribution.

#### Scenario: Durable retention is requested

r[agent_harness_runtime.request_context_contributions.retention.denied]
- GIVEN a request-local or sensitive payload is proposed for session ledger, compaction input, memory generation, quality records, or receipt storage
- WHEN retention admission runs
- THEN Animus MUST reject the proposal before persistence
- AND a payload hash MUST NOT be treated as permission to retain the payload.

### Requirement: Request-context evidence is metadata-only

r[agent_harness_runtime.request_context_contributions.evidence] Contribution receipts MUST contain bounded contribution, source, trust, posture, priority, budget, deduplication, supersession, conflict, lifetime, omission, payload-hash, plan, redaction, non-claim, and BLAKE3 identity metadata only.

#### Scenario: Safe selection receipt is projected

r[agent_harness_runtime.request_context_contributions.evidence.safe]
- GIVEN contribution reduction and model binding pass exact validation
- WHEN receipt projection runs
- THEN the receipt MUST describe selected and omitted metadata with exact plan identity
- AND it MUST not claim prompt correctness, provider delivery, model compliance, or task completion.

#### Scenario: Receipt contains context payload

r[agent_harness_runtime.request_context_contributions.evidence.rejects]
- GIVEN a receipt candidate contains prompt bodies, skill bodies, MCP instructions, memory records, project files, hidden directives, credentials, paths, provider data, or unrestricted diagnostics
- WHEN receipt validation runs
- THEN Animus MUST reject it without echoing the value
- AND the candidate MUST NOT enter session, completion, lifecycle, or release evidence.

### Requirement: Request-context conformance covers successful and failed flows

r[agent_harness_runtime.request_context_contributions.verification] The change MUST include positive and negative production-path tests for contract validation, order independence, mandatory and advisory budgets, supersession, deduplication, conflicts, finite lifetimes, model drift, skill response barriers, retention, and safe evidence.

#### Scenario: Complete conformance corpus passes

r[agent_harness_runtime.request_context_contributions.verification.complete]
- GIVEN paired fixtures cover every producer class, reducer rule, lifetime, skill-turn transition, model transition, and receipt boundary
- WHEN focused core, host, model-profile, skill, and composition checks run
- THEN allowed cases MUST produce deterministic plans and denied cases MUST fail before provider dispatch or persistence
- AND repeated runs over equal inputs MUST produce equal BLAKE3 identities.

#### Scenario: Closeout evidence is incomplete

r[agent_harness_runtime.request_context_contributions.verification.incomplete]
- GIVEN maintainers intend to archive without provider-barrier, stale-body, mandatory-conflict, retention-leak, no-std, Wasm, Cairn, or relevant Nix evidence
- WHEN closeout validation runs
- THEN archive readiness MUST remain blocked with the missing evidence or exact unavailable-rail blocker
- AND prompt correctness, skill quality, model compliance, and production readiness MUST remain explicit non-claims.
