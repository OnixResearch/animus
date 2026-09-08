# Accept Responses reasoning text events

## Problem

When GLM responses include reasoning, OpenRouter emits the OpenAI Responses `reasoning` output item with `reasoning_text` content parts and `response.reasoning_text.delta` / `.done` events.
The production parser only models reasoning summary events: it hardwires `response.content_part.added` / `.done` to message items and rejects unknown reasoning events as unsafe.
A retained live capture shows the reasoning item's first content part failing function-call correlation, so every reasoning-bearing response fails while reasoning-free responses succeed.

## Proposed solution

Route `response.reasoning_text.delta` and `.done` through the existing reasoning paths:

- Deltas become redacted thought frames exactly like reasoning summary deltas; they require a `content_index` (or a `summary_index`) so uncorrelated reasoning stays rejected.
- Done events and `response.content_part.added` / `.done` records select their expected item kind from the part type: `reasoning_text` parts validate against reasoning items, and text parts keep validating against message items.

Bounds, sequence, identity, marker, and terminal guards are untouched. Thought frames stay redacted and never become assistant-visible output.

## Scope

- `animus-std-openai` Responses SSE parser: reasoning text event family.
- Parser tests mirroring the captured reasoning-bearing stream, plus rejection guards.
- Provider host documentation.

## Non-goals

- No change to what becomes assistant-visible output or evidence.
- No acceptance of encrypted or uncorrelated reasoning.
- No tool-call, bound, or lifecycle changes.
