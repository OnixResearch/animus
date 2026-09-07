# Accept Responses SSE keep-alive comments

## Problem

OpenRouter emits SSE comment keep-alive records (`": "` followed by a blank line) before and between real events when response generation is slow.
The production Responses SSE parser rejects every comment-only record as malformed, so slow live responses fail while fast ones succeed.
A retained live capture from 2026-09-07 shows the stream opening with repeated comment records; the parser fails on the first record.
The framing compatibility change already published handles data-only records but not comment-only records.

## Proposed solution

Skip comment-only SSE records during event processing.
A record whose every line starts with `:` carries no event name and no data, and per the SSE contract it is a comment for the consumer to ignore.
Records that mix comments with data or event fields keep the existing parsing, duplicate-field rejection, and lifecycle guards.
Bounds accounting still charges every record, so keep-alives cannot extend stream limits.

## Scope

- `animus-std-openai` Responses SSE parser: comment-only record skip.
- Parser tests: keep-alives before events, between events, inside a data record, CRLF variants, and existing rejection guards.
- Documentation for the provider host behavior.

## Non-goals

- No change to completion authority, tool handling, bounds, or sequence checks.
- No transport retry or reconnection behavior.
- No acceptance of malformed or unknown non-comment lines.
