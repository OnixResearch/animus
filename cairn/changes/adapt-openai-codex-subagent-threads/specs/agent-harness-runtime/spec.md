# agent-harness-runtime Change Spec: OpenAI Codex subagent thread adaptation

## Requirements

### Requirement: Explicit subagent delegation

r[agent_harness_runtime.openai_codex_subagent_threads.delegation_request] Animus MUST start subagent threads only from explicit delegation requests that define role, task, allowed tools, workspace scope, model profile, and expected summary shape.

#### Scenario: Implicit subagent spawn is rejected

r[agent_harness_runtime.openai_codex_subagent_threads.delegation_request.explicit]
- GIVEN a runtime turn has no explicit user, trusted skill, or approved automation delegation request
- WHEN the loop fabric attempts to start a subagent
- THEN runtime rejects the subagent start
- AND no model or tool calls are dispatched for that subagent.

### Requirement: Parent thread is protected from raw subagent logs

r[agent_harness_runtime.openai_codex_subagent_threads.thread_isolation] Subagent intermediate output MUST remain in an isolated subagent thread ledger by default, with only bounded status and summary references imported into the parent thread.

#### Scenario: Raw subagent logs stay out of parent context

r[agent_harness_runtime.openai_codex_subagent_threads.thread_isolation.no_raw_logs]
- GIVEN a subagent produces long test logs and exploratory notes
- WHEN the subagent completes successfully
- THEN the parent thread receives a bounded summary reference
- AND the raw logs are not inserted into parent model context unless explicitly imported under output bounds.

### Requirement: Typed subagent summary contract

r[agent_harness_runtime.openai_codex_subagent_threads.summary_contract] Subagent completion MUST produce a typed summary containing findings, evidence references, confidence, unresolved questions, and recommended next actions.

#### Scenario: Malformed subagent summary fails aggregation

r[agent_harness_runtime.openai_codex_subagent_threads.summary_contract.validation]
- GIVEN a subagent returns a summary missing required evidence references for findings
- WHEN the parent aggregates subagent results
- THEN aggregation records a malformed-summary failure
- AND the parent does not treat the incomplete finding as validated evidence.

### Requirement: Subagent role profiles constrain capabilities

r[agent_harness_runtime.openai_codex_subagent_threads.role_profiles] Subagent role profiles MUST constrain model preference, reasoning effort, tool permissions, workspace scope, and output schema before execution.

#### Scenario: Unauthorized subagent tool use is denied

r[agent_harness_runtime.openai_codex_subagent_threads.role_profiles.permissions]
- GIVEN a review-only subagent profile excludes write-capable tools
- WHEN the subagent requests a write-capable tool
- THEN runtime denies the request
- AND the denial is recorded in the subagent ledger and parent summary status.

### Requirement: Parallel write conflict policy

r[agent_harness_runtime.openai_codex_subagent_threads.write_conflict_policy] Write-capable subagents MUST use isolated workspaces, mutation serialization, or parent-approved handoff before changes to shared resources are merged.

#### Scenario: Parallel writes do not merge silently

r[agent_harness_runtime.openai_codex_subagent_threads.write_conflict_policy.conflict]
- GIVEN two subagents modify the same shared resource in parallel
- WHEN their results are aggregated
- THEN runtime reports a write conflict unless a mutation coordinator or explicit handoff resolved the conflict
- AND neither change is silently preferred over the other.

### Requirement: Subagent runtime event reporting

r[agent_harness_runtime.openai_codex_subagent_threads.event_reporting] Runtime reports MUST include subagent start, stop, continuation, failure, and aggregation events with parent/subagent correlation identifiers.

#### Scenario: Failed subagent is visible to parent

r[agent_harness_runtime.openai_codex_subagent_threads.event_reporting.failure]
- GIVEN a subagent fails before producing a valid summary
- WHEN the parent turn aggregates subagent outcomes
- THEN the parent report includes the subagent identifier, failure class, and bounded failure evidence
- AND the parent does not report the subagent task as successful.
