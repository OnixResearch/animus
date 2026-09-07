# Responses SSE keep-alive comments

## Purpose

Accept SSE comment keep-alive records without changing event, completion, or bound validation.

## ADDED Requirements

### Requirement: Comment-only records are ignored

r[agent_harness_runtime.responses_sse_framing.keepalive] The parser MUST skip a record whose every line begins with a comment prefix and MUST charge the record against the configured event bound.

#### Scenario: Keep-alives before the first event

- GIVEN a stream that opens with repeated comment-only records before a valid ordered Responses envelope
- WHEN the parser processes the stream
- THEN admission of later events is unchanged.

#### Scenario: Comment inside a data record

- GIVEN a record mixing a comment line with a data line
- WHEN the parser processes the record
- THEN the data is parsed under the existing envelope and lifecycle checks.

### Requirement: Preserved rejection guards

r[agent_harness_runtime.responses_sse_framing.keepalive.guards] The parser MUST keep rejecting unknown non-comment lines, duplicate fields, premature or repeated markers, and incomplete streams when comment records are present.

#### Scenario: Keep-alives do not mask violations

- GIVEN a stream with comment records around a sequence, identity, bound, or marker violation
- WHEN the parser processes or finishes the stream
- THEN it rejects the violation exactly as it would without comments.
