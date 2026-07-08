# agent-harness-runtime Change Spec: Memory control surface

## Requirements

### Requirement: Adapter-neutral memory control commands

r[agent_harness_runtime.memory_controls.commands] Animus memory control surfaces MUST represent list, show, search, why, approve, reject, forget, update or supersede, and safe export-summary operations as adapter-neutral DTOs over normalized memory identities, scopes, policies, and safe output budgets.

#### Scenario: Memory listing exposes only safe descriptors

r[agent_harness_runtime.memory_controls.commands.safe_listing]
- GIVEN a host lists or searches available memories through the control surface
- WHEN reusable command projection builds the response
- THEN the response contains safe descriptors, scopes, statuses, summaries, evidence handles, and hashes within budget
- AND raw hidden memory bodies, credentials, provider payloads, full paths, and unbounded text are absent.

### Requirement: Memory control validation precedes mutation

r[agent_harness_runtime.memory_controls.validation] Memory control requests MUST be validated for scope, capability, admission policy, freshness, required evidence, backend support, and safe output posture before invoking storage, update/delete, extraction, remote, or prompt-influence adapters.

#### Scenario: Denied control request performs no mutation

r[agent_harness_runtime.memory_controls.validation.denied]
- GIVEN a caller requests memory update, forget, export, approval, or prompt-visible use without required scope, capability, admission, or freshness
- WHEN control validation runs
- THEN validation returns a typed denial or stale-command diagnostic
- AND no adapter mutation, remote call, prompt injection, or receipt claiming completion occurs.

### Requirement: Extracted candidate review states

r[agent_harness_runtime.memory_controls.review] Extracted memory candidates MUST move through explicit review states, including pending, approved, rejected, expired, superseded, unavailable, and persisted, before they can be stored or influence prompts.

#### Scenario: Approval requires scope and evidence

r[agent_harness_runtime.memory_controls.review.approval]
- GIVEN an extracted memory candidate is pending review
- WHEN a host approves or rejects it
- THEN approval requires a valid scope, source label, evidence handle, confidence, redaction status, and admission decision
- AND rejection records safe reason metadata without persisting or injecting the candidate.

### Requirement: Forget, update, and supersede are adapter-dispatched

r[agent_harness_runtime.memory_controls.forget_update] Forget, update, tombstone, and supersession operations MUST dispatch through injected memory adapters and return typed complete, partial, denied, stale, unavailable, or unsupported outcomes with safe receipts.

#### Scenario: Unavailable backend does not claim deletion

r[agent_harness_runtime.memory_controls.forget_update.unavailable]
- GIVEN a caller requests forgetting a memory record
- AND the active store adapter is unavailable or does not support deletion
- WHEN the control operation runs
- THEN the result records unavailable or unsupported status
- AND no receipt claims the record was deleted, tombstoned, or superseded.

### Requirement: Memory influence explanation is safe

r[agent_harness_runtime.memory_controls.influence_explain] Memory `why` or influence explanations MUST be derived from prompt-injection ledgers and MUST include only safe record IDs, source labels, ranks, match reasons, scope, confidence, freshness, policy decision, consumed budget, skip reason, redaction class, and BLAKE3 body hashes.

#### Scenario: Why output omits private bodies

r[agent_harness_runtime.memory_controls.influence_explain.safe]
- GIVEN a user asks why a memory was included or skipped for a prompt
- WHEN the control surface renders the explanation
- THEN the response names the safe inclusion or skip reasons and policy inputs
- AND raw memory bodies, credentials, provider payloads, full paths, and hidden memory text are absent.

### Requirement: Remote memory controls require admission

r[agent_harness_runtime.memory_controls.remote_admission] Remote, headless, MCP, or automation-triggered memory control requests MUST pass admission and authority checks before prompt-visible use, approval, update, deletion, export, or store mutation is dispatched to any memory adapter.

#### Scenario: Remote denial blocks memory mutation

r[agent_harness_runtime.memory_controls.remote_admission.denied]
- GIVEN a remote caller requests approving, updating, deleting, exporting, or using memory
- WHEN remote admission is missing, denied, expired, or malformed
- THEN the memory control request fails before adapter dispatch
- AND the denial receipt records safe caller, scope, action, and reason metadata without leaking raw memory content.

### Requirement: Memory control verification

r[agent_harness_runtime.memory_controls.verification] The memory control surface MUST include focused positive and negative tests for safe listing, command validation, unauthorized requests, candidate approval and rejection, forget/update outcomes, unavailable backends, influence explanation, remote admission denial, and receipt leak rejection.

#### Scenario: Closeout evidence covers memory control fixtures

r[agent_harness_runtime.memory_controls.verification.closeout]
- GIVEN maintainers intend to archive the memory control surface
- WHEN closeout validation runs
- THEN evidence names focused command, validation, review, forget/update, influence-explain, remote-admission, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
