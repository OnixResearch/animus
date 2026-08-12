# Design: Ensure model token and thought streaming for Animus

## Context

The runtime event lifecycle change names model dispatch and final response events, while provider registry work records reasoning/thinking support and provider parity work requires future streaming parser-entrypoint tests. This change owns the missing cross-cutting contract: how streaming model output enters the adapter boundary, how runtime projects it to subscribers, and how thought/reasoning channels stay safe.

## Decisions

### 1. Streaming frames are adapter-boundary DTOs

**Choice:** Add typed model stream frames for visible content token/delta updates, provider-exposed thought/reasoning updates, tool-call deltas, usage updates, finish, cancellation, and failure. Each frame carries model-call correlation and an order field scoped to that call.

**Rationale:** Runtime, headless, TUI, tests, and receipts need one stable stream shape instead of provider-specific fragments.

### 2. Streaming capabilities are explicit

**Choice:** Effective model settings declare whether content streaming and thought/reasoning streaming are supported. A request for an unsupported stream mode fails with a typed capability error or downgrades according to an explicit host policy.

**Rationale:** Silent fallback would make tests and UI behavior lie about what the selected model can do.

### 3. Provider claims use real parser entrypoints

**Choice:** Any concrete provider adapter that claims streaming support must test raw provider stream bytes or chunks through the parser entrypoint used by production code.

**Rationale:** Helper-only tests can pass while real SSE or chunk parsing drops tokens, loses reasoning deltas, or mishandles finish/error records.

### 4. Runtime assembles final response from visible content

**Choice:** Runtime emits partial stream events as they arrive, then derives final assistant-visible response content from the visible content stream or final provider response according to deterministic assembly rules.

**Rationale:** Streaming observers and final-result consumers should agree without scraping logs or merging thought channels into the answer.

### 5. Thought/reasoning is a policy-gated channel

**Choice:** Thought/reasoning frames are separate content channels with visibility and sensitivity metadata. Default sinks omit or redact them unless policy explicitly allows a safe projection.

**Rationale:** Some providers expose reasoning summaries or thought-like deltas, but raw hidden reasoning must not leak into user-visible answers, receipts, dataset exports, or logs by accident.

### 6. Protocol and UI surfaces consume the same events

**Choice:** Headless protocol, future TUI streaming, and SDK subscribers consume the same runtime stream events and correlation IDs, with host-specific rendering outside reusable core DTOs.

**Rationale:** End-to-end streaming should be testable once and reusable across product shells.

## Risks / Trade-offs

- Providers disagree on whether streamed text is tokens, chunks, deltas, reasoning summaries, or hidden reasoning. The adapter contract should model channels and policy rather than provider names.
- Thought streaming can create privacy and safety failures. Defaults must redact or omit sensitive thought channels and tests must exercise leak attempts.
- Stream parsers are prone to fragmentation bugs. Fixtures must include split records, malformed data, finish records, and partial failures.
- Event volume can grow quickly. DTOs and receipts should keep bounded content policies and safe summaries.
