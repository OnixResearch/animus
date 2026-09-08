# Design: reasoning text event family

## Context

The parser tracks output items by identifier and kind (`message`, `reasoning`, `function_call`).
Content part events are hardwired to the message kind, and every unrecognized event containing `reasoning` is rejected with `UnsafeReasoning`.
The OpenAI Responses specification also defines exposed reasoning text: an item of type `reasoning` whose content parts have type `reasoning_text`, streamed through `response.reasoning_text.delta` and `response.reasoning_text.done` events that correlate with `output_index`, `item_id`, and `content_index` fields.
Live GLM responses use exactly this family, so reasoning-bearing responses fail today.

## Decision

1. Add the event constants `response.reasoning_text.delta` and `response.reasoning_text.done`.
2. Route the delta through the existing thought-delta path. That path already requires a started response, a registered reasoning item for `item_id`, bounded redacted frames, and sequence checks.
   The summary-index requirement is generalized: a thought delta must carry at least one bounded index field (`summary_index` or `content_index`), so summary deltas are unchanged and uncorrelated reasoning stays rejected.
3. Route the done event through the existing output lifecycle check with reasoning kind.
4. Make content part events select the expected item kind from the part type: `reasoning_text` parts validate against reasoning items; every other part validates against message items exactly as before.

Thought frames remain redacted: they never appear in assistant-visible output, never enter final-answer extraction, and stay omitted from evidence rendering.

## Rejection guards preserved

- A reasoning delta without `summary_index` and without `content_index` still fails with `UnsafeReasoning`.
- A content part for an unregistered item still fails with `Correlation`.
- Duplicate output items, unknown non-reasoning lines, premature or repeated markers, sequence drift, and terminal violations are unchanged.
- Reasoning events after a terminal still fail lifecycle checks.

## Test plan

- A stream mirroring the captured GLM shape (reasoning item, reasoning text parts and deltas, then the message) parses to `Completed` with the same assistant-visible text as the plain stream.
- Uncorrelated reasoning deltas and unknown-item content parts still fail.
- CRLF and byte-fragmented variants still parse.
- The retained failing live capture parses offline to a completed response after the change.
