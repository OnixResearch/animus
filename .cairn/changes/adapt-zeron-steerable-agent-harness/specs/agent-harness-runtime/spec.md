# agent-harness-runtime Change Spec: Zeron steerable external-agent harness adaptation

## Requirements

### Requirement: Bounded external-agent harness reference

r[agent_harness_runtime.zeron_agent_harness.scope] Animus MUST treat the Zeron (zeronsh/comet) steerable external-agent subprocess harness and durable owner-gated command plane as a bounded reference for external agent control, and MUST NOT adopt the gpui UI, Loro CRDT sync, TypeScript edge worker, Cloudflare Durable Objects, token-usage display, or multi-device transport as part of this adaptation.

#### Scenario: Bounded scope review rejects wholesale adoption

r[agent_harness_runtime.zeron_agent_harness.scope.review]
- GIVEN a proposed change imports gpui, Loro, a TypeScript edge worker, a Durable Objects client, or a token-usage display into reusable Animus harness or command-plane crates
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until only the approved harness and command-plane boundaries remain
- AND the rejection names the Animus-owned alternative boundary.

### Requirement: Typed external-agent harness contract

r[agent_harness_runtime.zeron_agent_harness.contract] Animus MUST represent external-agent control through typed harness send, steer, interrupt, and respond-input operations with explicit command DTOs, model, reasoning, and option metadata, and explicit outcome classes for created, completed, steered, interrupted, input-required, cancelled, timed-out, unavailable, and malformed outcomes, and MUST NOT name a concrete agent CLI as the shared contract.

#### Scenario: Malformed harness contract fails before dispatch

r[agent_harness_runtime.zeron_agent_harness.contract.malformed]
- GIVEN a harness command or outcome DTO is missing required fields or carries malformed metadata
- WHEN harness contract validation runs
- THEN validation returns typed diagnostics
- AND no subprocess, deadline, cancellation, or harness dispatch occurs with the malformed value.

### Requirement: Injected bounded external-process delegation

r[agent_harness_runtime.zeron_agent_harness.bounds] Animus MUST delegate external-agent harness execution to an injected bounded external-process service with explicit program, arguments, working directory, environment mode, standard-input bound, output bounds, deadline, cancellation, and owned teardown, and reusable harness core MUST NOT spawn processes, discover agent CLIs, read credentials, or attach network transports, and MUST fail closed when the bounded-process service is unavailable or returns an unsuccessful observation.

#### Scenario: Missing bounded-process service fails before harness effects

r[agent_harness_runtime.zeron_agent_harness.bounds.missing_service]
- GIVEN an external-agent harness run requires a subprocess
- AND no bounded-process service is injected or the service reports unavailable
- WHEN harness dispatch runs
- THEN dispatch returns a typed unavailable-harness-service outcome
- AND no fallback CLI, shell, thread, or network call is attempted.

#### Scenario: Absent explicit limits reject the request

r[agent_harness_runtime.zeron_agent_harness.bounds.invalid_limits]
- GIVEN a harness subprocess request omits an explicit input bound, output bound, deadline, or teardown bound
- WHEN execution-bound validation runs
- THEN validation rejects the request with a missing-limit diagnostic
- AND no implicit default limits or ambient environment discovery are used as a fallback.

### Requirement: Durable command-plane entries

r[agent_harness_runtime.zeron_command_plane.entries] Send, steer, interrupt, and respond-input MUST be represented as typed durable command entries carrying stable identity, kind, sequence or generation, target session, ownership metadata, expected outcome class, payload hash, and safe envelope metadata, and identity material MUST NOT embed raw prompt or payload bodies.

#### Scenario: Command-entry identity rejects raw material

r[agent_harness_runtime.zeron_command_plane.entries.identity_leak]
- GIVEN a command-entry candidate embeds a raw prompt body, provider payload, or credential in its identity material
- WHEN command-entry validation runs
- THEN validation rejects the entry with a leak diagnostic
- AND the command is not admitted with that leaking identity.

### Requirement: Owner-gated idempotent command execution

r[agent_harness_runtime.zeron_command_plane.ownership] Command execution MUST be gated on explicit host ownership of the target session, MUST mark a command processed BEFORE host execution, and MUST consult the processed-command ledger to reject re-execution of an already-processed command.

#### Scenario: Non-owner host enqueues without executing

r[agent_harness_runtime.zeron_command_plane.ownership.not_owner]
- GIVEN a session command targets a session owned by another host
- WHEN a non-owner host evaluates the command
- THEN the non-owner host enqueues the command without executing it
- AND no subprocess or harness effect is invoked.

#### Scenario: Processed command is not executed again

r[agent_harness_runtime.zeron_command_plane.ownership.processed_idempotence]
- GIVEN the processed-command ledger records a command entry as processed
- WHEN the owner host or a replay reader presents the same command entry again
- THEN the entry is not executed again
- AND the receipt records the skipped duplicate without claiming a new harness effect.

### Requirement: Steering and offline command semantics

r[agent_harness_runtime.zeron_command_plane.steer] A steer or respond-input command with no live active run MUST dispatch as the next turn, and send, steer, interrupt, or respond-input commands for an unavailable harness MUST remain queued without being dropped or converted into transcript content.

#### Scenario: Steer with no live run dispatches as the next turn

r[agent_harness_runtime.zeron_command_plane.steer.no_active_run]
- GIVEN a steer or respond-input command is submitted
- AND no live active run exists for the target session
- WHEN the command plane evaluates the submission
- THEN the command dispatches as the next turn
- AND it is not executed against a stale or phantom run.

#### Scenario: Unavailable harness keeps commands queued

r[agent_harness_runtime.zeron_command_plane.steer.offline_queue]
- GIVEN the target harness is unavailable or not started
- WHEN a send, steer, interrupt, or respond-input command is submitted
- THEN the command remains queued
- AND it does not enter transcript content and is not dropped.

### Requirement: Safe command-plane and harness receipts

r[agent_harness_runtime.zeron_command_plane.receipts] Command-plane and harness receipts MUST record safe metadata only, including command kind, sequence, command hash, outcome class, status class, payload hash, harness family label, and bounded counts, and MUST NOT include raw prompts, harness JSONL lines, provider payloads, tool bodies, model outputs, credentials, full paths, or secret summaries.

#### Scenario: Receipt rejects raw command and harness material

r[agent_harness_runtime.zeron_command_plane.receipts.rejects_leaks]
- GIVEN a command-plane or harness receipt candidate contains a raw prompt, harness JSONL line, provider payload, tool body, model output, credential, full host path, or secret summary
- WHEN receipt validation runs
- THEN validation rejects the candidate with a typed leak diagnostic
- AND durable evidence records only safe labels, hashes, counts, and redaction markers.

### Requirement: Zeron harness adaptation verification

r[agent_harness_runtime.zeron_agent_harness.verification] The adaptation MUST include focused positive and negative tests for typed command admission, harness send, steer, interrupt, and respond-input, owner-gated execution, mark-processed-before-execute idempotence, steering with no active run, offline queuing, missing or timed-out bounded-process service, invalid execution bounds, malformed harness outcomes, and receipt redaction.

#### Scenario: Closeout evidence covers harness fixtures

r[agent_harness_runtime.zeron_agent_harness.verification.closeout]
- GIVEN maintainers intend to archive the Zeron harness adaptation
- WHEN closeout validation runs
- THEN evidence names focused harness, command-plane, bounded-delegation, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
