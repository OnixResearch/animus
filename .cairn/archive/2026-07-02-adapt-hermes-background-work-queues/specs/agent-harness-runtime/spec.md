# agent-harness-runtime Change Spec: Hermes background work queue adaptation

## Requirements

### Requirement: Host-injected background queue service

r[agent_harness_runtime.hermes_background_queue.service] Animus MUST define host-injected background queue service contracts and DTOs for enqueue, status, wait, log, send-input, pause, resume, restart, cancel, clean, group listing, and group parallelism operations without requiring reusable runtime crates to spawn processes directly.

#### Scenario: Queue service handles long-running task

r[agent_harness_runtime.hermes_background_queue.service.enqueue]
- GIVEN a host injects a background queue service and a tool requests a long-running non-interactive command through an approved queue tool
- WHEN runtime dispatches the request
- THEN the queue service receives a typed enqueue request with command identity, group, environment policy, timeout policy, and output bounds
- AND reusable runtime code does not call shell, process, or pueue APIs directly.

### Requirement: Destructive queue controls require explicit selectors

r[agent_harness_runtime.hermes_background_queue.explicit_targets] Queue operations that pause, kill, restart, clean, resume, or alter group-wide parallelism MUST require explicit task, group, default-group, or all-groups selectors and MUST reject ambiguous default behavior.

#### Scenario: Ambiguous kill request is rejected

r[agent_harness_runtime.hermes_background_queue.explicit_targets.reject_ambiguous]
- GIVEN a queue-control request asks to kill work but omits task IDs, group name, explicit default-group selector, and all-groups selector
- WHEN request validation runs
- THEN validation returns a typed ambiguous-target diagnostic
- AND no queue backend operation is invoked.

### Requirement: Queue groups and parallelism are typed

r[agent_harness_runtime.hermes_background_queue.groups] Background queue DTOs MUST represent group identity, configured parallelism, pending/running/paused/stashed/completed counts, and backend-specific status labels as safe metadata.

#### Scenario: Group status is inspectable

r[agent_harness_runtime.hermes_background_queue.groups.status]
- GIVEN a queue backend has multiple groups with different parallelism limits
- WHEN a host reports queue status to Animus
- THEN the normalized DTO includes each group, parallelism limit, counts by stable status class, and safe backend labels
- AND prompt/report rendering can distinguish CPU-heavy, IO-heavy, or default groups when hosts provide labels.

### Requirement: Logs and command output are bounded

r[agent_harness_runtime.hermes_background_queue.logs] Background task logs MUST be exposed through bounded viewports with visible text, total line/byte counts, truncation state, safe references, and BLAKE3 content hashes when full-output identity is needed.

#### Scenario: Large log tail is truncated safely

r[agent_harness_runtime.hermes_background_queue.logs.truncated]
- GIVEN a completed background task produced a large stdout/stderr log
- WHEN the model-visible log view is requested
- THEN the response includes only the configured visible window or tail, line/byte counts, truncation state, and optional BLAKE3 hash
- AND it does not embed the full raw log body in model-visible content or receipts.

### Requirement: Wait and send semantics are explicit capabilities

r[agent_harness_runtime.hermes_background_queue.wait_send] Background queue tooling MUST model wait, stream, poll, and send-input behavior as explicit capabilities with timeout, paused/stashed/queued handling, and unsupported-operation failures.

#### Scenario: Send input unsupported fails closed

r[agent_harness_runtime.hermes_background_queue.wait_send.unsupported_send]
- GIVEN a backend task does not support stdin/send-input operations
- WHEN a tool request attempts to send input to that task
- THEN the queue service returns an unsupported-operation failure
- AND runtime does not fall back to shelling out to a backend-specific CLI.

### Requirement: Background queue adaptation verification

r[agent_harness_runtime.hermes_background_queue.verification] The background queue adaptation MUST include focused tests for enqueue/status/log flows, explicit target validation, group/parallelism DTOs, bounded logs, wait behavior, unsupported send failures, missing queue service failures, and backend isolation.

#### Scenario: Closeout evidence covers queue fixtures

r[agent_harness_runtime.hermes_background_queue.verification.closeout]
- GIVEN maintainers intend to archive the Hermes background queue adaptation
- WHEN closeout validation runs
- THEN evidence names focused queue DTO tests, destructive-target tests, log-bound tests, wait/send tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
