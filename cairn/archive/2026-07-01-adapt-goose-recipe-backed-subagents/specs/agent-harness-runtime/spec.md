# agent-harness-runtime Change Spec: Goose recipe-backed subagent adaptation

## Requirements

### Requirement: Recipe-backed subagent delegation is typed

r[agent_harness_runtime.goose_recipe_backed_subagents.delegation] Animus MUST support explicit subagent delegation DTOs that may reference an admitted recipe or profile and include parameter bindings, tool ceilings, workspace scope, model profile, timeout and turn ceilings, return mode, and summary schema.

#### Scenario: Valid recipe-backed subagent request is accepted

r[agent_harness_runtime.goose_recipe_backed_subagents.delegation.valid]
- GIVEN a trusted delegation request references an admitted review recipe with valid parameter bindings and a summary schema
- WHEN subagent delegation validation runs
- THEN validation returns an accepted child-run request
- AND no subagent model or tool dispatch occurs during validation.

### Requirement: Subagent starts require explicit admission

r[agent_harness_runtime.goose_recipe_backed_subagents.explicit] Recipe-backed subagents MUST start only from explicit user delegation, trusted skill delegation, or admitted automation policy delegation, and MUST NOT auto-spawn or recursively spawn by default.

#### Scenario: Implicit or recursive subagent start is denied

r[agent_harness_runtime.goose_recipe_backed_subagents.explicit.denied]
- GIVEN no explicit admitted delegation exists, or a subagent attempts to create another subagent
- WHEN runtime evaluates the child-run request
- THEN the request is rejected
- AND no model, tool, external agent, or host-effect dispatch is started for that child run.

### Requirement: Subagent capability inheritance is intersected

r[agent_harness_runtime.goose_recipe_backed_subagents.capability_intersection] Effective subagent capabilities MUST be the intersection of parent policy, recipe or profile ceilings, request ceilings, workspace scope, and host admission.

#### Scenario: Inherited tool widening is denied

r[agent_harness_runtime.goose_recipe_backed_subagents.capability_intersection.denied]
- GIVEN a parent session has write-capable tools but a recipe-backed review subagent is read-only
- WHEN the subagent requests a write-capable tool or out-of-scope workspace access
- THEN runtime denies the request
- AND the denial is recorded in the subagent ledger and parent handoff status.

### Requirement: Parent handoff follows return mode bounds

r[agent_harness_runtime.goose_recipe_backed_subagents.return_modes] Recipe-backed subagents MUST hand results to the parent using explicit return modes for summary-only, status-plus-summary, or full-detail-with-bounds, while keeping subagent transcripts isolated by default.

#### Scenario: Raw logs cannot flood parent context

r[agent_harness_runtime.goose_recipe_backed_subagents.return_modes.bounded]
- GIVEN a subagent produces long exploratory logs and test output
- WHEN the subagent completes with summary-only return mode
- THEN the parent receives only the bounded summary and evidence references
- AND raw subagent logs remain outside parent context unless explicitly imported under configured bounds.

### Requirement: External subagents use injected adapters

r[agent_harness_runtime.goose_recipe_backed_subagents.external] External subagents backed by ACP or MCP agents MUST use explicit descriptors and injected host adapters rather than ambient CLI discovery, inherited credentials, or undeclared tools.

#### Scenario: Ambient external agent discovery is rejected

r[agent_harness_runtime.goose_recipe_backed_subagents.external.no_ambient]
- GIVEN a subagent recipe names an external agent without an admitted ACP or MCP descriptor and injected host service
- WHEN runtime prepares the child run
- THEN preparation returns a missing-external-agent-service failure
- AND runtime does not search PATH, read credentials, or start a fallback CLI.

### Requirement: Subagent aggregation records partial outcomes

r[agent_harness_runtime.goose_recipe_backed_subagents.aggregation] Subagent aggregation MUST distinguish success, failure, timeout, cancellation, malformed summary, denied tool, partial parallel success, and write conflict outcomes.

#### Scenario: Failed subagent is not reported as success

r[agent_harness_runtime.goose_recipe_backed_subagents.aggregation.failure]
- GIVEN parallel recipe-backed subagents include one successful summary and one timed-out child run
- WHEN parent aggregation completes
- THEN the aggregate report names the successful and failed outcomes separately
- AND the timed-out child run is not counted as a successful finding.

### Requirement: Recipe-backed subagent closeout is evidenced

r[agent_harness_runtime.goose_recipe_backed_subagents.verification] Recipe-backed subagent adoption MUST include positive recipe-backed delegation tests, negative implicit-spawn tests, capability-denial tests, return-mode bound tests, external-adapter tests, aggregation failure tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Subagent evidence is complete

- GIVEN the Goose recipe-backed subagent change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused subagent tests, interop descriptor tests, ledger isolation tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
