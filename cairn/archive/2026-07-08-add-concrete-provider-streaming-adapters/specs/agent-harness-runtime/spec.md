# agent-harness-runtime Change Spec: Concrete provider streaming adapters

## Requirements

### Requirement: Concrete provider streaming remains host-owned

r[agent_harness_runtime.provider_streaming.scope] Animus MUST treat concrete provider streaming as an optional host-owned adapter surface, and MUST NOT move provider clients, credential lookup, network transport, endpoint discovery, retry policy, or provider-native raw streams into `animus-core`, `animus-adapters`, `animus-router`, or reusable runtime core defaults.

#### Scenario: Reusable crates stay provider-client-free

r[agent_harness_runtime.provider_streaming.host_boundary]
- GIVEN a host enables concrete provider streaming for a supported provider family
- WHEN reusable crate boundaries are inspected
- THEN core/router/runtime defaults expose only typed requests, capabilities, stream frames, parser contracts, and injected-service seams
- AND concrete provider clients, credentials, endpoint discovery, network transport, and retry policy remain in the host-owned shell or optional adapter implementation.

### Requirement: Provider stream parsers use production entrypoints

r[agent_harness_runtime.provider_streaming.parser_entrypoint] Each concrete provider streaming adapter MUST expose a deterministic parser entrypoint that consumes admitted raw provider chunks and emits canonical `ModelStreamFrame` values for visible content, thought/reasoning, tool-call deltas, usage updates, finish, cancellation, and failure records.

#### Scenario: Raw chunks become canonical frames

r[agent_harness_runtime.provider_streaming.parser_entrypoint.fixture]
- GIVEN a checked-in raw provider stream fixture contains fragmented content deltas, provider-exposed thought/reasoning deltas, tool-call deltas, usage records, and a successful finish record
- WHEN the production parser entrypoint consumes the fixture chunks
- THEN it emits the expected canonical stream frames in strict per-call order with stable correlation metadata
- AND helper-only fixture paths are insufficient to satisfy the provider streaming claim.

#### Scenario: Malformed provider stream fails closed

r[agent_harness_runtime.provider_streaming.parser_entrypoint.malformed]
- GIVEN raw provider stream input contains malformed records, missing finish records, out-of-order terminal frames, truncated chunks, provider error records, cancellation records, or unknown stream channels
- WHEN the production parser entrypoint consumes the input
- THEN it returns a typed stream failure, cancellation, or unsupported-channel diagnostic
- AND runtime does not assemble or report a successful final response from that malformed stream.

### Requirement: Streaming dispatch is capability-gated

r[agent_harness_runtime.provider_streaming.capability_gate] Provider streaming dispatch MUST resolve requested content, thought/reasoning, tool-call delta, and usage streaming modes against selected-model capability metadata before contacting a provider, and unsupported requested modes MUST fail closed under reject policy or downgrade only through an explicit content-only policy.

#### Scenario: Unsupported thought streaming is not fabricated

- GIVEN a selected provider model advertises content streaming but not thought/reasoning streaming
- WHEN a host requests thought/reasoning streaming under reject policy
- THEN dispatch returns a typed unsupported-capability outcome before provider contact
- AND no adapter fabricates `ThoughtDelta` frames, silent downgrade, or fallback model selection.

### Requirement: Provider thought and reasoning remains policy-gated

r[agent_harness_runtime.provider_streaming.thought_policy] Provider-exposed reasoning, thinking, or summary chunks MUST map to a separate thought/reasoning stream channel with policy-gated or redacted default visibility, and MUST NOT become assistant-visible final answer text, receipt bodies, dataset rows, review report bodies, or logs unless an explicit host policy creates a safe projection.

#### Scenario: Reasoning stream does not leak into evidence

- GIVEN a provider stream includes visible answer chunks and provider-exposed reasoning chunks
- WHEN runtime projects events, assembles final output, or writes safe evidence under default policy
- THEN visible answer chunks may contribute to assistant-visible output according to normal content policy
- AND reasoning chunks are omitted, redacted, or safely summarized without exposing raw private reasoning.

### Requirement: Provider streaming verification distinguishes fixture and live evidence

r[agent_harness_runtime.provider_streaming.verification] Concrete provider streaming work MUST include positive and negative parser fixtures, capability-gate tests, thought-policy leak tests, runtime event projection tests, docs, and Cairn validation before archive, and MUST label optional live-provider checks separately from fixture-backed support.

#### Scenario: Closeout evidence avoids live-provider overclaims

- GIVEN concrete provider streaming support is ready to archive
- WHEN closeout evidence is prepared
- THEN evidence names fixture-backed parser tests, capability negotiation tests, malformed-stream tests, thought-policy tests, runtime projection tests, docs, Cairn validation, and Cairn gates
- AND any live provider checks are marked optional with credential requirements while absent live checks are recorded as non-claims rather than failures.
