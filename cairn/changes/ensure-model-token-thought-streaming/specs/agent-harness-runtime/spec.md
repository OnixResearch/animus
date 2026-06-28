# agent-harness-runtime Change Spec: Model token and thought streaming

## Requirements

### Requirement: Streaming model adapter contract

r[agent_harness_runtime.model_streaming.adapter_contract] Animus MUST define a streaming model-adapter contract that reports ordered frames for visible content token or delta updates, provider-exposed thought/reasoning updates, tool-call deltas, usage, finish, cancellation, and failure with model-call correlation IDs and per-call ordering.

#### Scenario: Stream frames preserve adapter order

r[agent_harness_runtime.model_streaming.adapter_contract.order]
- GIVEN an injected model adapter emits visible content frames, thought/reasoning frames, usage metadata, and a finish frame for one model call
- WHEN runtime consumes the model stream
- THEN emitted runtime facts preserve the adapter frame order and model-call correlation ID
- AND final assistant-visible content is assembled from visible content frames unless an explicit policy selects another visible channel.

### Requirement: Streaming capabilities are explicit

r[agent_harness_runtime.model_streaming.capabilities] Effective model settings MUST identify whether the selected model supports visible content streaming, thought/reasoning streaming, tool-call delta streaming, and usage streaming before runtime dispatches a streaming model request.

#### Scenario: Unsupported thought streaming fails explicitly

r[agent_harness_runtime.model_streaming.capabilities.unsupported_thought]
- GIVEN the selected model does not support thought/reasoning streaming
- WHEN a host requests thought/reasoning streaming for that model
- THEN settings resolution returns a typed unsupported-capability failure or an explicit policy-approved content-only downgrade
- AND runtime does not fabricate thought/reasoning frames.

### Requirement: Provider streaming uses parser-entrypoint fixtures

r[agent_harness_runtime.model_streaming.parser_entrypoint] Any provider or shell adapter that claims streaming support MUST include fixtures that feed raw provider stream bytes or chunks through the same parser entrypoint used by production code.

#### Scenario: Raw stream fixture becomes canonical frames

r[agent_harness_runtime.model_streaming.parser_entrypoint.fixture]
- GIVEN a raw provider stream fixture contains fragmented content deltas, thought/reasoning deltas, tool-call deltas, usage records, and a finish record
- WHEN the production parser entrypoint consumes the fixture chunks
- THEN it emits the expected canonical stream frames in order
- AND no helper-only parser path is sufficient to satisfy the streaming claim.

#### Scenario: Malformed stream fails closed

r[agent_harness_runtime.model_streaming.parser_entrypoint.malformed]
- GIVEN raw provider stream input contains invalid records, a missing finish record, out-of-order terminal events, or truncated chunks
- WHEN the production parser entrypoint consumes the input
- THEN parsing returns a typed stream failure
- AND runtime does not report a successful final response from the malformed stream.

### Requirement: Runtime projects streaming events

r[agent_harness_runtime.model_streaming.runtime_events] Runtime MUST project model stream frames into typed runtime events and protocol/UI-neutral DTOs before final response completion, preserving correlation IDs, channel kind, ordering, bounded body policy, and terminal status.

#### Scenario: Subscribers observe incremental output before completion

r[agent_harness_runtime.model_streaming.runtime_events.end_to_end]
- GIVEN a model call streams visible content frames and authorized thought/reasoning frames before producing a final response
- WHEN a headless or SDK-style subscriber observes runtime events
- THEN the subscriber receives ordered stream-start, stream-frame, and stream-end or stream-failure events before final response completion
- AND each event carries stable correlation metadata without requiring terminal UI state.

### Requirement: Thought and reasoning visibility is policy-gated

r[agent_harness_runtime.model_streaming.thought_visibility] Thought/reasoning stream frames MUST carry visibility and sensitivity metadata, and default runtime sinks MUST NOT treat them as assistant-visible final answer text, receipt bodies, dataset rows, review report bodies, or logs unless policy explicitly authorizes a safe projection.

#### Scenario: Default policy prevents thought leakage

r[agent_harness_runtime.model_streaming.thought_visibility.default_redaction]
- GIVEN a model stream contains provider-exposed thought/reasoning frames and visible answer frames
- WHEN runtime builds final output, receipts, dataset export rows, review reports, or logs under the default policy
- THEN visible answer frames may appear in assistant-visible output according to normal content policy
- AND thought/reasoning frames are omitted, redacted, or safely summarized without exposing raw private reasoning.

### Requirement: Streaming closeout is evidenced

r[agent_harness_runtime.model_streaming.verification] Model streaming work MUST include positive and negative tests for stream frame ordering, content finalization, thought/reasoning policy, parser-entrypoint fixtures, unsupported capabilities, malformed streams, cancellation or failure, and protocol/UI-neutral event projection before the change is archived.

#### Scenario: Closeout evidence covers streaming behavior

r[agent_harness_runtime.model_streaming.verification.closeout]
- GIVEN token and thought streaming work is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused streaming tests, parser fixture tests, leak-rejection tests, capability-negotiation tests, runtime event projection tests, workspace checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
