# agent-harness-runtime Change Spec: Clankers controller continuation policy adaptation

## Requirements

### Requirement: Typed continuation state

r[agent_harness_runtime.clankers_continuation_policy.state] Animus MUST represent continuation state with typed queued prompt, active loop work, validation follow-up, busy prompt, cancellation, pending dispatch, and completion acknowledgement fields.

#### Scenario: Queued prompt state is explicit

r[agent_harness_runtime.clankers_continuation_policy.state.queued_prompt]
- GIVEN user input arrives while a prompt is busy
- WHEN continuation state is updated
- THEN the queued prompt is represented as typed pending work with an opaque ID
- AND no shell-local hidden flag is required to decide the next action.

### Requirement: Deterministic continuation priority

r[agent_harness_runtime.clankers_continuation_policy.priority] Continuation decisions MUST apply deterministic policy priority among queued user prompt replay, active loop continuation, validation or auto-test follow-up, and completion.

#### Scenario: Queued user prompt wins over automation

r[agent_harness_runtime.clankers_continuation_policy.priority.queued_wins]
- GIVEN a queued user prompt and an eligible validation follow-up both exist after a model turn
- WHEN continuation policy evaluates next action
- THEN it selects queued prompt replay first
- AND the validation follow-up remains pending.

### Requirement: Exact dispatch and completion acknowledgements

r[agent_harness_runtime.clankers_continuation_policy.acknowledgements] Shell adapters MUST acknowledge dispatch and completion using the exact pending work ID, and stale, duplicate, missing, or mismatched IDs MUST fail without clearing unrelated continuation state.

#### Scenario: Stale completion is rejected

r[agent_harness_runtime.clankers_continuation_policy.acknowledgements.stale]
- GIVEN a completion acknowledgement references a stale pending work ID
- WHEN continuation state validates the acknowledgement
- THEN it rejects the acknowledgement with a stale-work failure
- AND current pending work remains intact.

### Requirement: Continuation effects are shell-owned

r[agent_harness_runtime.clankers_continuation_policy.shell_owned] Provider calls, timers, queue waits, session persistence, UI events, and validation command execution MUST remain shell-owned behavior outside the pure continuation decision core.

#### Scenario: Decision core does not execute provider call

r[agent_harness_runtime.clankers_continuation_policy.shell_owned.no_provider]
- GIVEN continuation policy selects a loop continuation
- WHEN the pure decision core returns the action
- THEN it returns a typed decision only
- AND it does not call providers, spawn commands, read storage, or emit UI events directly.

### Requirement: Safe continuation receipts

r[agent_harness_runtime.clankers_continuation_policy.receipts] Continuation receipts MUST record safe metadata such as prior state class, selected action, pending work ID hash, reason, rejection class, policy hash, and BLAKE3 receipt hash, and MUST NOT include raw prompts, model outputs, tool outputs, provider payloads, full transcripts, or unbounded logs.

#### Scenario: Continuation receipt leak rejected

r[agent_harness_runtime.clankers_continuation_policy.receipts.rejects_leaks]
- GIVEN a continuation receipt fixture contains raw prompt text, model output, tool output, provider JSON, full transcript, or unbounded log text
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Continuation policy verification

r[agent_harness_runtime.clankers_continuation_policy.verification] The adaptation MUST include focused positive and negative tests for typed state, priority order, queued prompts, loop continuation, validation follow-ups, dispatch/completion acknowledgements, stale and duplicate IDs, shell-owned effects, cancellation, and receipt redaction.

#### Scenario: Closeout evidence covers continuation fixtures

r[agent_harness_runtime.clankers_continuation_policy.verification.closeout]
- GIVEN maintainers intend to archive the controller continuation policy adaptation
- WHEN closeout validation runs
- THEN evidence names focused state, priority, acknowledgement, shell-boundary, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
