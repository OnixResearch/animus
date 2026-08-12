# agent-harness-runtime Change Spec: Goose message queue and interrupt controls

## Requirements

### Requirement: Queue and interrupt DTOs

r[agent_harness_runtime.goose_message_queue.dtos] Animus MUST define typed DTOs for queued messages, queue operations, queue state, dispatch acknowledgements, interrupt signals, redirection payloads, cancellation requests, and user-control events.

#### Scenario: Follow-up message is queued while active work runs

r[agent_harness_runtime.goose_message_queue.dtos.follow_up]
- GIVEN runtime is processing an active turn and a user submits a follow-up message with queue semantics
- WHEN the queue operation is validated
- THEN the queued message records source identity, sequence identity, active-work reference, safety class, budget class, and policy identity
- AND it is not appended to model-visible context until a later dispatch decision admits it.

### Requirement: Pure queue reducer

r[agent_harness_runtime.goose_message_queue.reducer] Queue state changes MUST be pure deterministic reducer operations over in-memory queue state, operation DTOs, active-work references, acknowledgements, policy limits, and safe message metadata, and MUST NOT call models, dispatch tools, mutate ledgers, persist storage, read files, open network connections, spawn processes, or use clocks.

#### Scenario: Reordering preserves stable queue identities

r[agent_harness_runtime.goose_message_queue.reducer.reorder]
- GIVEN a queue contains three admitted follow-up messages with stable item IDs
- WHEN a reorder operation moves the third item before the second item
- THEN the reducer returns a queue with the requested order and unchanged item identities
- AND no model, tool, ledger, or host cancellation effect is dispatched.

### Requirement: Interrupt signals are explicit control events

r[agent_harness_runtime.goose_message_queue.interrupts] Interrupt handling MUST classify explicit send modes and policy-owned keyword matches into pause, cancel, redirect, queue, or immediate-dispatch control events before deciding whether any text becomes model-visible prompt content.

#### Scenario: Stop keyword requests cancellation

r[agent_harness_runtime.goose_message_queue.interrupts.stop]
- GIVEN active work is running and the user sends a high-confidence stop keyword under an interrupt send mode
- WHEN interrupt classification runs
- THEN the result is a cancellation control event linked to the active-work identity
- AND the stop text is not added to model-visible prompt history unless a later admitted redirection explicitly includes it.

### Requirement: Queue and interrupt failures are fail-closed

r[agent_harness_runtime.goose_message_queue.fail_closed] Queue and interrupt handling MUST fail closed for stale active-work IDs, duplicate queue item IDs, unauthorized queue mutation sources, max-queue overflow, unsafe queued bodies, denied policy, missing cancellation handles, unsupported active-work states, and malformed acknowledgements.

#### Scenario: Missing cancellation handle prevents abort claim

r[agent_harness_runtime.goose_message_queue.fail_closed.no_cancel]
- GIVEN an interrupt requests cancellation for active work but the host supplied no cancellation handle or cancellation capability
- WHEN control planning evaluates the interrupt
- THEN the result is a missing-cancellation-handle failure
- AND no receipt, event, or status claims that provider, tool, or process work was aborted.

### Requirement: Safe queue and interrupt receipts

r[agent_harness_runtime.goose_message_queue.receipts] Queue and interrupt operations MUST produce safe receipts with operation kind, actor or source label, active-work hash, queue item hash, decision class, cancellation handle class, budget class, policy hash, denial class, redaction state, and BLAKE3 receipt hash.

#### Scenario: Queue receipt rejects raw queued prompt text

r[agent_harness_runtime.goose_message_queue.receipts.rejects_leaks]
- GIVEN a queue or interrupt receipt candidate contains raw prompt text, provider payloads, credentials, full paths, raw tool output, raw command output, or unbounded queue bodies
- WHEN receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND durable queue evidence does not echo the raw material.

### Requirement: Message queue and interrupt verification

r[agent_harness_runtime.goose_message_queue.verification] Message queue and interrupt adoption MUST include positive append, edit, reorder, delete, clear, dispatch, interrupt, pause, and redirection tests plus negative stale-work, unauthorized mutation, unsafe-body, missing-cancellation-handle, max-queue, malformed acknowledgement, no ambient effect, and receipt leak fixtures.

#### Scenario: Closeout evidence covers queue controls

r[agent_harness_runtime.goose_message_queue.verification.closeout]
- GIVEN maintainers intend to archive the Goose message queue and interrupt adaptation
- WHEN closeout validation runs
- THEN evidence names queue DTO, reducer, interrupt, fail-closed, receipt, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
