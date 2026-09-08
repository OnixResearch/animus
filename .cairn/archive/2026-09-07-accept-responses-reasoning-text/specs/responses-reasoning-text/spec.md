# Responses reasoning text events

## Purpose

Accept exposed reasoning text events without making thought content assistant-visible or weakening any guard.

## ADDED Requirements

### Requirement: Reasoning text events are redacted thought frames

r[agent_harness_runtime.responses_sse_framing.reasoning_text] The parser MUST accept `response.reasoning_text.delta` and `.done` events for registered reasoning items as redacted thought frames, and MUST require a `summary_index` or `content_index` correlation field on every reasoning delta.

#### Scenario: Reasoning item precedes the message

- GIVEN a stream with a registered reasoning item, its `reasoning_text` content parts and deltas, and then a completed message
- WHEN the parser processes the stream
- THEN the assistant-visible output equals the message text and thought frames stay redacted.

#### Scenario: Uncorrelated reasoning delta

- GIVEN a reasoning delta without `summary_index` and without `content_index`
- WHEN the parser processes the event
- THEN it rejects the stream.

### Requirement: Content parts select their item kind

r[agent_harness_runtime.responses_sse_framing.reasoning_text.parts] The parser MUST validate a content part against the item kind that its part type implies: `reasoning_text` parts against reasoning items and all other parts against message items.

#### Scenario: Reasoning content part

- GIVEN a `response.content_part.added` record whose part type is `reasoning_text` and whose item is a registered reasoning item
- WHEN the parser processes the record
- THEN admission succeeds under the existing sequence and lifecycle checks.

#### Scenario: Unknown item correlation

- GIVEN a content part record whose item identifier is not registered
- WHEN the parser processes the record
- THEN it rejects the stream.
