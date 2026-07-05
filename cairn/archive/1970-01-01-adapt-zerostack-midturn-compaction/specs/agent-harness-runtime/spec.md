# agent-harness-runtime Change Spec: Zerostack mid-turn compaction adaptation

## Requirements

### Requirement: Provider prompt pressure calibration

r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure] Animus MUST represent provider-reported prompt pressure separately from estimated replay tokens and MUST account for provider route metadata before using cached-token fields.

#### Scenario: Cached tokens are not double-counted

r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure.cache]
- GIVEN provider usage reports input tokens and cached-token fields
- AND route metadata says cached tokens are already included in input tokens
- WHEN prompt pressure is calculated
- THEN cached-token fields are not added a second time
- AND the calculation records the route rule used.

### Requirement: Mid-turn compaction planning

r[agent_harness_runtime.zerostack_midturn_compaction.plan] Runtime MUST produce an explicit mid-turn stop, compact, and resume plan when policy enables mid-turn compaction and prompt pressure crosses the configured threshold.

#### Scenario: Disabled mid-turn compaction does not trigger

r[agent_harness_runtime.zerostack_midturn_compaction.plan.disabled]
- GIVEN prompt pressure crosses the threshold
- AND mid-turn compaction is disabled by policy
- WHEN runtime evaluates continuation
- THEN it does not prepare a mid-turn compaction plan
- AND it returns the policy-selected continue, warn, or fail outcome.

### Requirement: Explicit ledger summary entries

r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries] Mid-turn compaction MUST append explicit bounded summary entries to the session ledger and MUST NOT hide compaction as an unrecorded in-memory transcript rewrite.

#### Scenario: Missing ledger service halts required compaction

r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries.missing]
- GIVEN policy requires mid-turn compaction before continuing
- AND no ledger service is injected
- WHEN runtime attempts to prepare the compaction
- THEN it returns a missing-ledger-service failure
- AND it does not call model or tool adapters as a fallback.

### Requirement: Injected summarizer for mid-turn compaction

r[agent_harness_runtime.zerostack_midturn_compaction.summarizer] Mid-turn compaction MUST use only injected summarizer services and MUST validate summary output for bounded text, safe metadata, and supported summary kind.

#### Scenario: Malformed summary fails closed

r[agent_harness_runtime.zerostack_midturn_compaction.summarizer.malformed]
- GIVEN the summarizer returns empty text, overlong text, unsupported metadata, or raw sensitive output in metadata
- WHEN runtime validates the summary
- THEN validation fails with a typed summary diagnostic
- AND the unsafe summary is not appended to the ledger.

### Requirement: Long tool-output artifactization

r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts] Tool outputs that exceed model-visible policy limits SHOULD be represented by bounded head/tail or windowed text plus artifact descriptors, total size metadata, truncation status, and BLAKE3 artifact hashes.

#### Scenario: Unbounded tool output is not embedded

r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts.bounded]
- GIVEN a tool output exceeds the configured model-visible limit
- WHEN it is recorded in ledger or receipt metadata
- THEN only the configured bounded window and artifact descriptor are model-visible by default
- AND the full raw output is not embedded in durable evidence.
