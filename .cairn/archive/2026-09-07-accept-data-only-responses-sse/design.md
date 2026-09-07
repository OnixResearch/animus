## Context

The public Responses parser is deterministic over bounded bytes and an admitted tool dispatch.
Its SSE field parser currently requires both `event:` and `data:` fields.
Its terminal guard rejects all subsequent records, including the OpenRouter transport marker.
The captured GLM response contains valid ordered JSON events and a completed `READY` answer.
Offline header insertion and guarded marker removal make that capture pass the unchanged parser.

## Decisions

### Decision: Parse optional event metadata without rewriting payloads

**Choice:** Return an optional event name from field parsing. Decode the required JSON envelope once. Compare names when an explicit name exists.

**Rationale:** The JSON type remains mandatory and passes the existing event allowlist, sequence, identity, and lifecycle validators.
Duplicate fields, empty explicit names, unknown fields, and malformed data remain errors.

### Decision: Treat the marker as transport termination, not completion evidence

**Choice:** Admit one unnamed `[DONE]` record only after `Completed`. Record its presence and reject duplicates or later records.

**Rationale:** The marker emits no canonical frame, changes no semantic terminal state, and cannot grant success to an incomplete or failed stream.
Existing finish validation still checks unfinished items and tool calls.
The marker consumes normal byte and event bounds.

### Decision: Keep publication and product adoption narrow

**Choice:** Implement in the owner worktree from current `origin/main`. Backport only this change onto the product's existing Animus revision before product adoption.

**Rationale:** Current upstream history contains unrelated changes that the user did not request for this compatibility correction.
The product consumes a published immutable revision. No ambient sibling dependency or live transport workaround becomes product code.

## Risks / Trade-offs

- Optional headers can hide a mismatch unless explicit names still require exact agreement.
- A transport marker can create false completion unless a validated completed event already exists.
- Chunk boundaries and final unterminated blocks can bypass bounds unless the existing accounting remains intact.
- Comments-only and multiline-data extensions remain outside this narrow change.
- Broad upstream verification can expose existing unrelated debt. Preserve those results and do not label a focused pass as a whole-workspace pass.

## Validation

Run the existing focused suite before edits.
Add controls for named, data-only, mixed, CRLF, fragmented, missing-header, mismatch, malformed-envelope, premature-marker, duplicate-marker, post-terminal, hidden-reasoning, and event-bound behavior.
Retain raw-capture replay evidence separately from minimized public fixtures.
No captured model content or credential enters a published fixture.
