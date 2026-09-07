# Design: comment-only SSE records

## Context

The parser splits the byte stream into records on blank-line separators, parses each record into an optional event name and required data, and rejects records that yield no data.
A comment keep-alive record contains one or more lines of the form `": text"` and no fields, so it currently fails with `Malformed`.
Live captures show OpenRouter sending sequences of such records before the first real event and between events.
Failure is intermittent because keep-alives appear only when the provider is slow.

## Decision

In `process_event`, after the UTF-8 check, return early when every line of the record starts with `:` after trailing-CR trimming.
The skipped record still consumed one event-count unit and its bytes still count against the event bound.
All other records proceed unchanged:

- Comments mixed with `data:` or `event:` lines do not trigger the skip because at least one line lacks the `:` prefix.
- Duplicate field rejection, envelope type validation, explicit-name agreement, sequence and identity checks, the guarded `[DONE]` marker, and final completeness checks are untouched.
- An incomplete stream that ends during keep-alives still fails `finish()` because no terminal was observed.

## Alternatives considered

- Relaxing `parse_sse_fields` to return an optional record — wider surface, same effect, more churn in a hot function.
- Stripping comments before field parsing — hides malformed-line rejection behind preprocessing and complicates the duplicate checks.

The early skip is the smallest change that restores live compatibility without weakening any guard.

## Test plan

Unit tests in the framing module cover:

- Keep-alive comments before the first event, including the exact captured pattern of repeated `": "` records.
- Keep-alives between events and immediately before the guarded marker.
- A comment line inside a data record.
- CRLF line endings in comment records.
- Rejection still holds for unknown non-comment lines, duplicate fields, premature markers, and incomplete streams.

A retained live capture that previously failed at its first record must parse offline to a completed response through the patched parser.
