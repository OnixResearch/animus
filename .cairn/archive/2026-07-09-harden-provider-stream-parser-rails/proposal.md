# Proposal: Harden provider stream parser rails

## Why

Animus has stream parser tests and provider event parsing seams, but some parser paths still rely on raw boundary assertions or concatenate input before enforcing all limits. Provider streaming is an adapter boundary where malformed data, oversized frames, unexpected event order, and hidden raw payloads can produce panics, memory growth, or unsafe receipts.

Stream parsers should be total over malformed input: they return typed errors, enforce byte and record budgets before concatenation, and keep receipt evidence safe.

## What Changes

- Replace parser-facing raw boundary `assert!`/panic behavior with typed parse, budget, ordering, and unsupported-event errors.
- Enforce per-frame, per-record, aggregate-byte, aggregate-record, and completion budgets before buffering or concatenation.
- Add parser-entrypoint tests that use provider-like raw chunks rather than only already-normalized events.
- Add receipt projections for stream failures that store safe counts, hashes, classes, and redaction markers only.
- Document parser guarantees and unsupported/non-claim behavior for live provider parity.

## Impact

- **Files**: streaming parsers, budget DTOs, error enums, fixture generators, parser-entrypoint tests, docs, and Cairn evidence.
- **Testing**: positive ordered stream fixtures plus negative invalid UTF-8, malformed JSON/SSE, oversized frame, record budget, aggregate byte budget, duplicate terminal, missing terminal, out-of-order delta, unsupported event, hidden reasoning leak, and receipt leak cases.
- **Non-claims**: this does not add live provider clients, full provider parity, network transport, retry behavior, or SDK-specific streaming guarantees.
