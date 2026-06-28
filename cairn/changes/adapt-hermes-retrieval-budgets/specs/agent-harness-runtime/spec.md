# agent-harness-runtime Change Spec: Hermes retrieval budget adaptation

## Requirements

### Requirement: Typed retrieval budget descriptors

r[agent_harness_runtime.hermes_retrieval_budgets.descriptor] Animus MUST define typed retrieval budget descriptors with retrieval purpose, first action guidance, repeat conditions, stop conditions, missing-evidence behavior, citation or source-naming expectations, progress guidance, provenance, and optional host-configured budget limits.

#### Scenario: Valid retrieval budget is advertised with resource

r[agent_harness_runtime.hermes_retrieval_budgets.descriptor.valid]
- GIVEN a retrieval-capable skill or tool has a validated retrieval budget descriptor
- WHEN prompt resource context is built
- THEN the model-visible guidance includes the bounded first/repeat/stop behavior
- AND the descriptor remains attributable to the resource that introduced it.

### Requirement: Retrieval-capable resources declare budget policy

r[agent_harness_runtime.hermes_retrieval_budgets.resource_policy] Resources that perform web search, browser automation, documentation lookup, repository browsing, multi-lens analysis, or property discovery SHOULD declare whether a retrieval budget is required before they are advertised.

#### Scenario: Required budget is missing

r[agent_harness_runtime.hermes_retrieval_budgets.resource_policy.missing]
- GIVEN host policy requires retrieval budgets for retrieval-capable resources
- WHEN a web-search tool or browsing skill lacks a valid budget descriptor
- THEN resource validation fails or marks the resource unavailable
- AND prompt construction reports a deterministic missing-budget diagnostic.

### Requirement: Repeat retrieval is justified by missing evidence

r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules] Retrieval budget descriptors MUST distinguish initial retrieval, permitted repeat retrieval, and stopping behavior, and repeat retrieval MUST be justified by missing required facts, missing artifacts, contradictions, or insufficient evidence for the requested output.

#### Scenario: Evidence is sufficient after first retrieval

r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules.stop_after_sufficient]
- GIVEN the first retrieval result supplies the required fact, owner, date, parameter, source, or code artifact
- WHEN retrieval planning evaluates the next step
- THEN the budget decision recommends answering from existing evidence
- AND it does not authorize additional retrieval for nonessential polish.

#### Scenario: Missing required fact authorizes targeted repeat

r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules.retrieve_missing]
- GIVEN the first retrieval result lacks a required fact needed for the user's requested output
- WHEN retrieval planning evaluates the next step
- THEN the budget decision authorizes one targeted retrieval or asks for the smallest missing input if retrieval cannot supply it
- AND the missing evidence is named in the report.

### Requirement: Retrieval evidence metadata is bounded

r[agent_harness_runtime.hermes_retrieval_budgets.evidence_metadata] Runtime reports and receipts MAY record retrieval budget decisions, source labels, safe references, counts, truncation status, and BLAKE3 content hashes, but MUST NOT embed raw fetched bodies, credentials, full host paths, or unbounded retrieved content by default.

#### Scenario: Receipt records retrieval decision safely

r[agent_harness_runtime.hermes_retrieval_budgets.evidence_metadata.safe]
- GIVEN a retrieval-capable tool returns a large document
- WHEN the runtime records retrieval evidence
- THEN the report contains source labels, bounded counts, truncation status, and optional BLAKE3 hashes
- AND it does not store the full raw document body unless an explicit host storage policy permits it outside reusable core receipts.

### Requirement: Retrieval budget adaptation verification

r[agent_harness_runtime.hermes_retrieval_budgets.verification] The retrieval budget adaptation MUST include focused tests for valid descriptors, missing budget diagnostics, repeat/stop decisions, missing-evidence naming, bounded evidence metadata, prompt attribution, and prompt-hygiene checker behavior.

#### Scenario: Closeout evidence covers retrieval fixtures

r[agent_harness_runtime.hermes_retrieval_budgets.verification.closeout]
- GIVEN maintainers intend to archive the Hermes retrieval budget adaptation
- WHEN closeout validation runs
- THEN evidence names focused budget descriptor tests, repeat/stop decision tests, evidence metadata tests, prompt-hygiene checks, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
