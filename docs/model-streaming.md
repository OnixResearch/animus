# Model token and thought streaming

Animus models streaming as adapter-boundary data before any host renders it. A model adapter emits canonical `ModelStreamFrame` values with a stable model-call ID, strictly increasing per-call order, a frame kind, optional text body, optional tool-call ID, optional usage metadata, and default sink visibility.

The stable frame kinds are visible content deltas, provider-exposed thought/reasoning deltas, tool-call deltas, usage updates, finish, cancellation, and failure. Runtime projection turns a validated frame list into protocol/UI-neutral stream-start, stream-frame, stream-end, or stream-failure events before final response completion.

Default final-response assembly includes only assistant-visible content deltas. Thought/reasoning frames are policy-gated or redacted by default and must not become final answer text, receipts, dataset rows, review report bodies, or logs unless a host policy explicitly creates a safe projection.

Provider or shell adapters that claim streaming support must test raw stream chunks through the production parser entrypoint and compare the canonical frames. Fixtures must cover fragmented content, thought/reasoning, tool-call deltas, usage, finish, cancellation or failure, malformed records, missing finish, out-of-order terminal events, and truncated chunks.

See [concrete provider streaming adapters](provider-streaming-adapters.md) for the optional `animus-std` parser seams, dispatch gates, fixture-backed evidence, and live-provider non-claims.

Non-claims: this contract does not add a network provider, expose private chain-of-thought by default, require every model to support thought streaming, or make provider-native raw streams part of the public runtime API.
