# agent-harness-runtime Change Spec: Claude Code agent-team adaptation

## Requirements

### Requirement: Bounded agent-team reference

r[agent_harness_runtime.claude_code_agents.scope] Animus MUST treat Claude Code subagents and background agents as a bounded reference for scoped delegation, and MUST NOT require cloud sessions, inherit full parent context by default, spawn OS processes from reusable core crates, or bypass permission and tool policy.

#### Scenario: Worker cannot bypass parent policy

r[agent_harness_runtime.claude_code_agents.scope.no_bypass]
- GIVEN a worker task requests a tool or mutation outside its allowed capabilities
- WHEN runtime evaluates the worker task or report
- THEN the request is denied or returned as an unresolved mutation intent
- AND no dispatch occurs without normal permission, capability, and mutation-coordination checks.

### Requirement: Scoped worker task contracts

r[agent_harness_runtime.claude_code_agents.task_contract] Worker tasks MUST declare objective, scope, input context packet, allowed tools or capabilities, budgets, expected output schema, parent trace metadata, and authority/grant metadata before worker execution.

#### Scenario: Overbroad worker task is rejected

r[agent_harness_runtime.claude_code_agents.task_contract.overbroad]
- GIVEN a worker task omits scope, allowed capabilities, budget, output schema, or parent trace metadata
- WHEN worker task validation runs
- THEN validation returns typed diagnostics
- AND the worker task is not enqueued.

### Requirement: Minimized worker context packets

r[agent_harness_runtime.claude_code_agents.context_packet] Worker context packets MUST be explicitly assembled from selected parent messages, summaries, file observations, memory descriptors, and tool results, and MUST NOT include full parent transcripts, raw hidden memory, credentials, provider payloads, or unbounded tool outputs by default.

#### Scenario: Parent context leak is rejected

r[agent_harness_runtime.claude_code_agents.context_packet.leak]
- GIVEN a worker context packet attempts to include a full parent transcript or raw credential-looking memory body without policy admission
- WHEN context packet validation runs
- THEN validation rejects the packet with a leak diagnostic
- AND the worker task cannot start with that packet.

### Requirement: Host-injected worker service

r[agent_harness_runtime.claude_code_agents.worker_service] Worker execution MUST use an injected host service for enqueue, status, cancel, and collect operations with typed unavailable-service, cancellation, timeout, and malformed-report failures.

#### Scenario: Missing worker service fails before delegation

r[agent_harness_runtime.claude_code_agents.worker_service.missing]
- GIVEN a coordinator requests worker delegation
- WHEN no worker service is injected
- THEN runtime returns a worker-service-unavailable failure
- AND no fallback thread, process, shell command, model call, or network request is attempted.

### Requirement: Evidence-bearing worker report merge

r[agent_harness_runtime.claude_code_agents.report_merge] Coordinator merge logic MUST validate worker reports against their output schemas, preserve evidence references, surface conflicts, and require policy or operator resolution before treating conflicting or malformed reports as final.

#### Scenario: Conflicting worker reports require arbitration

r[agent_harness_runtime.claude_code_agents.report_merge.conflict]
- GIVEN two worker reports make incompatible claims or propose conflicting mutations
- WHEN coordinator merge evaluates the reports
- THEN the merge result records a structured conflict with evidence references
- AND no conflicting mutation is applied until policy or operator arbitration resolves it.
