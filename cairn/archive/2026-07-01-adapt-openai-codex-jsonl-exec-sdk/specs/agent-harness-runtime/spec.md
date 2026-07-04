# agent-harness-runtime Change Spec: OpenAI Codex JSONL exec and SDK adaptation

## Requirements

### Requirement: Non-interactive run request contract

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request] Animus MUST represent non-interactive execution as a typed run request containing prompt input, optional external context handles, sandbox profile, output preferences, and optional resume identity.

#### Scenario: Invalid run request fails before runtime start

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request.validation]
- GIVEN a non-interactive run request with an invalid sandbox profile
- WHEN Animus validates the request
- THEN validation fails before any model, tool, or hook adapter is invoked
- AND the failure identifies the invalid run-request field.

### Requirement: Versioned machine-readable event stream

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.event_stream] Runtime automation MUST expose a versioned append-only event stream for thread, turn, item, model, tool, policy, hook, usage, completion, and failure events.

#### Scenario: Events preserve runtime ordering

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.event_stream.ordering]
- GIVEN a runtime turn dispatches a model request followed by a tool request and final response
- WHEN the automation event stream is serialized
- THEN event order reflects the runtime order
- AND each event body remains within the configured event-size budget.

### Requirement: Final output derives from terminal response item

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.final_output] Non-interactive shells MUST derive final output from the terminal runtime response item instead of scraping logs or progress messages.

#### Scenario: Human-readable progress is not final output

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.final_output.selection]
- GIVEN a run emits progress messages and a terminal final-response item
- WHEN the shell writes the final output sink
- THEN the sink contains the final-response item content
- AND progress messages are not included unless they are part of that item.

### Requirement: Structured final-output validation

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.structured_output] Structured final outputs MUST be validated against a schema-identified contract before the run is reported successful.

#### Scenario: Malformed structured output fails the run

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.structured_output.validation]
- GIVEN a run requests structured final output with required fields
- WHEN the model produces a final response missing a required field
- THEN validation fails before success is reported
- AND the event stream records a structured-output validation failure.

### Requirement: Thread controller API

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.thread_controller] Animus MUST provide controller-facing operations to start, resume, run, inspect, and cancel runtime threads through typed thread identities.

#### Scenario: Resume checks thread identity before continuing

r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.thread_controller.resume]
- GIVEN a resume request references a thread identity with a mismatched ledger hash
- WHEN the controller validates the resume request
- THEN the thread is not resumed
- AND no new model or tool calls are dispatched for that request.
