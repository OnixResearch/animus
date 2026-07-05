# Clankers observability audit receipts

Animus audit receipts are bounded projections over host-supplied runtime state. They provide operator evidence without storing transcripts, raw tool data, provider payloads, or private pending maps.

## Snapshots

`AuditSnapshot` records schema version, policy hash, event window identity, bounded work summaries, timing buckets, status classes, over-limit flags, and failure class counts. Hosts provide already-summarized counts for model calls, tool calls, confirmations, background work, delivery attempts, and retries.

Empty windows are valid and return zero counts with safe metadata.

## Pending clamping

`project_audit_snapshot` clamps each public pending count to `AuditProjectionPolicy::public_pending_limit`. The snapshot can report the total pending count and an over-limit flag, but it never accepts or serializes raw pending call IDs, arguments, outputs, prompts, or private metadata.

## Host-owned boundaries

Metrics sinks, telemetry exporters, clocks, persistent storage, and UI rendering remain outside reusable projection logic. Host adapters can record `AuditHostBoundaryOutcome` values, but sink availability does not mutate an `AuditSnapshot`.

## Composition

Audit summaries use stable safe categories rather than raw payloads:

- work kind: model call, tool call, confirmation, background work, delivery attempt, retry
- status class: idle, pending, completed, failed
- timing bucket: none, immediate, interactive, background, slow
- failure class: invalid input, denied capability, timeout, cancellation, malformed output, unavailable service

## Receipts

`AuditReceipt::validate` rejects forbidden sensitive fields and computes a deterministic BLAKE3 receipt hash over safe snapshot metadata.

Forbidden receipt material includes raw prompts, tool arguments, tool outputs, provider payloads, credentials, authorization headers, environment values, full paths, and unbounded logs.

## Non-claims

This contract does not implement telemetry export, metrics storage, clocks, UI rendering, transcript capture, raw logs, provider/tool payload persistence, or full session tracing.
