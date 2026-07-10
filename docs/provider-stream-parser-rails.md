# Provider stream parser rails

Animus keeps provider streaming parser logic in a pure, deterministic core. Host shells may feed admitted provider-like byte chunks into the parser, but live provider clients, credentials, network transport, retries, and SDK-specific stream behavior remain host-owned non-claims.

## Entrypoint inventory

| Entrypoint | Input framing | Output | Budget policy | Receipt projection | Raw-boundary posture |
|---|---|---|---|---|---|
| `parse_provider_stream_chunks` | UTF-8 byte chunks containing JSON lines or SSE `data:` JSON lines | `animus_adapters::ModelStreamFrame` values | frame bytes, record bytes, aggregate bytes, record count, event count, and completion count are checked before buffering past the admitted limit | `ProviderStreamParserReceipt` with parser class, provider family, status, counts, budget/failure class, BLAKE3 stream identity, and redaction markers | malformed bytes, malformed records, unsupported events, invalid order, duplicate terminals, missing terminals, and unknown terminal states return typed failures instead of panics |

## Receipt safety

Parser receipts are summaries only. They may contain safe provider-family labels, counts, status classes, budget kinds, failure classes, BLAKE3 identities, and redaction markers such as `thought:redacted`, `tool_call:redacted`, and `provider_payload:redacted`.

Receipts must not include raw provider chunks, hidden reasoning text, tool-call payloads, credentials, provider JSON bodies, full paths, or unbounded logs. `validate_provider_stream_parser_receipt` rejects forged or malformed redaction markers.

## Fixture evidence

Focused tests in `crates/animus-runtime/src/provider_hardening/tests.rs` cover:

- ordered SSE and JSON-line chunks through `parse_provider_stream_chunks`;
- split-boundary records;
- invalid UTF-8;
- malformed records;
- oversized frame, record, aggregate, record-count, event-count, and completion budgets;
- duplicate terminal records;
- missing terminal records;
- out-of-order deltas;
- unsupported events and unknown terminal states;
- hidden-reasoning redaction and forged receipt-leak rejection.

These tests prove parser hardening and receipt safety for the fixture-backed parser rail. They do not claim live provider parity or SDK behavior.