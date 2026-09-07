# Responses SSE framing

## Purpose

Admit the selected data-only Responses framing without weakening event or completion validation.

## ADDED Requirements

### Requirement: Optional event metadata

r[agent_harness_runtime.responses_sse_framing.metadata] The parser MUST admit an absent SSE event name only when the required JSON envelope supplies a valid supported type and MUST retain exact agreement for an explicit event name.

#### Scenario: Data-only event

- GIVEN a data-only record with a valid ordered Responses envelope
- WHEN the parser processes the record
- THEN the existing typed event and lifecycle checks determine admission.

#### Scenario: Conflicting or malformed metadata

- GIVEN an explicit name that conflicts with the JSON type, an empty explicit name, duplicate fields, or a malformed envelope
- WHEN the parser processes the record
- THEN the parser rejects the record.

### Requirement: Guarded transport marker

r[agent_harness_runtime.responses_sse_framing.termination] The parser MUST accept at most one unnamed `[DONE]` marker after an already validated completed response and MUST NOT treat the marker as semantic completion or permit subsequent records.

#### Scenario: Completed response and marker

- GIVEN a valid completed response and an optional trailing unnamed marker
- WHEN parsing finishes
- THEN the semantic terminal and canonical frames match the same response without the marker.

#### Scenario: Premature or repeated marker

- GIVEN a marker before completion, after failure, with an explicit event name, or repeated after a prior marker
- WHEN the parser processes it
- THEN the parser rejects the stream.

### Requirement: Preserved validation boundaries

r[agent_harness_runtime.responses_sse_framing.guards] The parser MUST preserve configured bounds, sequence checks, explicit-name agreement, response and tool identity, hidden-reasoning rejection, and final completeness checks for both framing forms.

#### Scenario: Fragmented valid stream

- GIVEN valid data-only events and a marker across arbitrary byte boundaries
- WHEN all bytes arrive within the configured bounds
- THEN parsing produces the same completed outcome as the named form.

#### Scenario: Invalid or excessive stream

- GIVEN a sequence, identity, reasoning, lifecycle, or bound violation in either framing form
- WHEN the parser processes or finishes the stream
- THEN it rejects the violation without tool execution or fabricated completion.
