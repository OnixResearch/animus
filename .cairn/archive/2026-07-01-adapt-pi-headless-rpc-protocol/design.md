# Design: Adapt Pi headless RPC protocol for Animus

## Context

Pi's RPC mode is useful because it cleanly separates command acknowledgements from asynchronous runtime events. It also has an important framing rule: split JSONL records only on LF. Animus can adapt those lessons while preferring Animus-owned DTOs and Preserves-compatible boundaries.

## Decisions

### 1. Protocol DTOs mirror runtime concepts

**Choice:** Define Animus command, response, and event DTOs around runtime input, queueing, state inspection, model/provider selection, session ledger operations, compaction, and shutdown.

**Rationale:** The protocol should be a stable shell around the runtime, not a copy of Pi commands or UI internals.

### 2. Framing is strict and testable

**Choice:** The initial stream transport uses LF-delimited records for text compatibility, with strict handling of CRLF and no splitting on Unicode line separators. Preserves text/binary encoding remains the preferred long-term typed boundary when enabled.

**Rationale:** Cross-language clients need a small framing rule that can be tested. Pi's warning about generic line readers is worth carrying forward.

### 3. Acknowledgements and events are separate

**Choice:** Every command can carry a request ID. The matching response acknowledges acceptance or rejection, while runtime events continue asynchronously.

**Rationale:** Long-running turns cannot block protocol-level state updates or streaming output.

### 4. Queueing is explicit during active turns

**Choice:** Prompt commands sent while the runtime is busy must specify steering or follow-up semantics; otherwise they fail preflight.

**Rationale:** Implicit mid-stream prompt behavior creates surprising context ordering.

### 5. Protocol errors are safe and bounded

**Choice:** Parse errors, unsupported commands, invalid state transitions, and unavailable injected services return bounded error DTOs without leaking provider payloads, credentials, raw ledgers, or unbounded tool output.

**Rationale:** Headless protocol logs are often stored by host applications and need the same redaction posture as receipts.

## Risks / Trade-offs

- Supporting both Preserves and JSON-compatible records can create drift. Keep one canonical DTO model and test both encodings only where both are exposed.
- Too many commands can turn the protocol into a full application shell. Start with runtime/session operations needed by embedders.
- Event streams can leak sensitive content if they mirror raw messages. Use the runtime event/redaction rules and explicit content-bearing DTOs.
