# agent-harness-runtime Change Spec: Zerostack fresh-context task adaptation

## Requirements

### Requirement: Fresh-context task descriptors

r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor] Animus MUST represent fresh-context task requests as typed descriptors with goal, admitted context references, allowed tool profile, model/provider policy, token budget, tool-call budget, timeout policy, cancellation policy, and expected result schema.

#### Scenario: Missing scheduler fails closed

r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor.missing_scheduler]
- GIVEN a runtime request includes a fresh-context task descriptor
- AND no task scheduler service is injected
- WHEN runtime prepares the task
- THEN it returns a missing-scheduler failure
- AND it does not spawn processes, call providers, or run tools as a fallback.

### Requirement: Read-only side-question mode

r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question] Side-question requests SHOULD be read-only by construction and MUST NOT write files, edit state, execute shell commands, mutate ledgers, or dispatch non-read tools unless a separate explicit policy admits that behavior.

#### Scenario: Side question mutation attempt is denied

r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question.readonly]
- GIVEN a side-question child run requests a write, edit, shell, network mutation, ledger mutation, or other non-read action
- WHEN permission resolution evaluates the child request
- THEN the action is denied with a read-only side-question reason
- AND the child result records the denied attempt as safe metadata.

### Requirement: Context and authority isolation

r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation] Fresh-context task runs MUST receive only explicitly admitted context references and capability grants, and MUST NOT inherit hidden parent prompts, credentials, session allowlists, or mutable process-global configuration.

#### Scenario: Parent allowlist is not inherited

r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation.allowlist]
- GIVEN the parent run has a session allowlist entry for a guarded action
- WHEN a fresh-context task run requests the same action
- THEN the child run must satisfy its own authority and permission policy
- AND the parent allowlist does not authorize the child action.

### Requirement: Attributed task results

r[agent_harness_runtime.zerostack_fresh_context_tasks.results] Task results MUST include child-run identity, task descriptor identity, result status, budget usage, cited context references, uncertainty markers, and redacted evidence summaries before they can influence parent prompt assembly.

#### Scenario: Malformed child result is withheld

r[agent_harness_runtime.zerostack_fresh_context_tasks.results.malformed]
- GIVEN a child run returns a result missing status, child identity, context references, or result schema fields
- WHEN parent prompt assembly evaluates the result
- THEN the result is withheld with a malformed-result diagnostic
- AND its raw body is not injected into the parent model-visible context.

### Requirement: Task receipts and cancellation

r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts] Fresh-context task receipts MUST record safe task identity, scheduler identity, status, cancellation reason, budget usage, redacted context/resource identifiers, and output hashes without embedding full child transcripts by default.

#### Scenario: Cancelled task records bounded receipt

r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts.cancelled]
- GIVEN a fresh-context task is cancelled by user, policy, timeout, or scheduler failure
- WHEN the runtime records the outcome
- THEN the receipt names the cancellation class and consumed budget metadata
- AND no unbounded child transcript or raw sensitive output is embedded.
