# agent-harness-runtime Change Spec: Pi compaction and branch-summary adaptation

## Requirements

### Requirement: Configurable compaction policy

r[agent_harness_runtime.pi_compaction.policy] Animus compaction MUST be governed by typed policy inputs for context budget, response reserve budget, kept-recent budget, summary format, summarizer selection, and redaction behavior.

#### Scenario: Policy drives compaction decision

r[agent_harness_runtime.pi_compaction.policy.trigger]
- GIVEN a replay context estimate exceeds the configured usable budget
- WHEN runtime evaluates compaction readiness
- THEN it prepares a compaction plan according to policy
- AND it does not compact when policy disables automatic compaction.

### Requirement: Safe compaction cut points

r[agent_harness_runtime.pi_compaction.cut_points] Compaction preparation MUST choose cut points that preserve valid model-visible message ordering and MUST NOT orphan tool results from their corresponding tool calls.

#### Scenario: Tool result boundary is rejected

r[agent_harness_runtime.pi_compaction.cut_points.reject_tool_result]
- GIVEN a candidate cut point would keep a tool result while removing the tool call that produced it
- WHEN compaction preparation evaluates the candidate
- THEN the candidate is rejected or moved to a valid boundary
- AND the resulting context remains structurally valid.

### Requirement: Split-turn compaction handling

r[agent_harness_runtime.pi_compaction.split_turn] Animus MUST represent split-turn compaction explicitly when a single turn exceeds the kept-recent budget, summarizing the early turn prefix without corrupting the kept suffix.

#### Scenario: Oversized turn gets prefix summary

r[agent_harness_runtime.pi_compaction.split_turn.prefix]
- GIVEN one user turn contains more model-visible content than the kept-recent budget
- WHEN compaction preparation runs
- THEN it identifies the kept suffix and early turn prefix
- AND the generated summary entry records that a split-turn prefix was summarized.

### Requirement: Branch handoff summaries

r[agent_harness_runtime.pi_compaction.branch_summary] When navigating away from a ledger branch, Animus SHOULD support host-requested branch handoff summaries that preserve bounded context from the abandoned branch at the new navigation point.

#### Scenario: Abandoned branch is summarized at navigation

r[agent_harness_runtime.pi_compaction.branch_summary.navigation]
- GIVEN a ledger has an old active leaf and a target leaf with a common ancestor
- WHEN the host requests branch handoff summarization during navigation
- THEN runtime summarizes the abandoned branch span from old leaf back to the common ancestor
- AND records a bounded summary entry associated with the navigation.

### Requirement: Injected summarizer service

r[agent_harness_runtime.pi_compaction.injected_summarizer] Runtime compaction and branch summarization MUST call only injected summarizer services and MUST fail closed when summarization is required but no summarizer is available, denied, cancelled, or returns malformed output.

#### Scenario: Missing summarizer halts compaction

r[agent_harness_runtime.pi_compaction.injected_summarizer.missing]
- GIVEN policy requires summarization before continuing a turn
- WHEN no summarizer service is injected
- THEN runtime emits a typed missing-summarizer failure
- AND it does not construct a provider client or call model/tool adapters as a fallback.

### Requirement: Summary metadata is safe and cumulative

r[agent_harness_runtime.pi_compaction.safe_metadata] Compaction and branch-summary entries MUST store bounded summary text and safe cumulative metadata, including counts, redacted resource identifiers, and BLAKE3 hashes when sensitive content identity is needed.

#### Scenario: Summary metadata rejects raw tool bodies

r[agent_harness_runtime.pi_compaction.safe_metadata.reject_raw]
- GIVEN a summary result attempts to include raw tool output or unredacted host paths in metadata
- WHEN runtime validates the summary entry
- THEN validation fails with a typed redaction or metadata error
- AND the unsafe summary is not appended to the ledger.
