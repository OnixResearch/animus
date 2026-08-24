# Concrete provider streaming adapters

Concrete provider streaming is optional and host-owned. Animus reusable crates keep the stable stream-frame DTOs, validation, capability metadata, and runtime event projection contracts; `animus-std` owns the fixture-backed concrete parser seams for provider families that a host chooses to enable.

The parser entrypoints consume already-admitted raw provider chunks and emit canonical `ModelStreamFrame` values. They do not create clients, read credentials, discover endpoints, open networks, retry calls, or expose provider-native raw streams as runtime API. The separate [provider retry contract](provider-retry-execution.md) consumes only canonical stream-commit state and a safe versioned failure class; raw chunks and provider diagnostics never enter retry planning.

## Supported fixture-backed families

The current checked-in parser support covers raw event-stream fixtures for:

- OpenAI-compatible Responses stream events;
- Anthropic-compatible Messages stream events.

Each parser maps provider content deltas, reasoning/thought deltas, tool-call deltas, usage records, finish events, cancellation events, and provider error events into canonical frame kinds. The same parser entrypoint handles fragmented chunks, so tests exercise the production parser core rather than helper-only fixture paths. Retry tracking treats accepted content, thought, tool-call, and usage semantics as committed and denies ordinary automatic replay after any such frame; fragmented transport before semantic acceptance can remain eligible.

Local HTTP and mock providers intentionally have no concrete provider-streaming claim in this module. Hosts may add those later as separate adapter implementations with their own fixtures and evidence.

## Dispatch gate

Before provider contact, host-owned dispatch planning must resolve requested content, thought/reasoning, tool-call delta, and usage streaming modes against selected-model capability metadata. Unsupported modes fail closed under reject policy. Content-only downgrade is allowed only through the explicit downgrade policy and must not fabricate thought/reasoning frames.

Required credentials and injected streaming service readiness are checked before dispatch. Missing credentials, missing services, or unavailable services return typed fail-closed outcomes without contacting a provider.

## Thought and evidence policy

Provider-exposed reasoning, thinking, or summary chunks map to `ThoughtDelta` frames with redacted default visibility. Default final-response assembly includes only assistant-visible content deltas. Reasoning chunks must not become assistant-visible final text, receipt bodies, dataset rows, review report bodies, or logs unless a host policy creates a safe projection.

## Generation-scoped response ownership

`animus_runtime::response_ownership` gives each reusable response connection one active generation owner. A concurrent claim fails closed. Frames and terminal observations must name the active generation.

Interruption, malformed framing, or return before the admitted terminal frame poisons the connection. The host must close or discard a poisoned transport. Only an exact terminal observation releases ownership for reuse.

This state machine does not prove frame authenticity, exactly-once delivery, transport isolation, provider correctness, or safe replay.

## Verification and non-claims

Fixture-backed evidence covers positive OpenAI/Anthropic chunk parsing, fragmented content deltas, thought/reasoning deltas, tool-call deltas, usage records, successful finish records, malformed records, missing finish, terminal-not-last records, truncated chunks, provider error records, cancellation records, unknown channels, capability-gate rejection/downgrade, missing credentials, missing services, and runtime stream projection through existing runtime tests.

Optional live-provider checks are non-claims unless a host supplies credentials, runs them explicitly, and indexes the results separately. This work does not claim provider reliability, network transport, retry policy, credential storage, provider discovery, live OpenAI/Anthropic access, or default production streaming support.
