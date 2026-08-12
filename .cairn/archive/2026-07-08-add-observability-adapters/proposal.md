# Proposal: Add metrics and logging adapters

## Why

Animus already emits ordered `RuntimeEvent` values and stores them in `RunReport`, but hosts do not yet have a first-class adapter contract for routing those safe lifecycle facts into metrics and logs. Without an explicit observability seam, embedders are likely to add ad hoc logging in reusable runtime code or serialize full events directly, which risks leaking prompts, model output, tool arguments, provider payloads, paths, credentials, or unbounded logs.

A metrics and logging adapter family gives hosts operator-visible runtime health while preserving Animus crate boundaries: reusable crates define implementation-free contracts and pure safe projections, and `animus-std` owns concrete sinks such as no-op, in-memory, JSONL, tracing, or external telemetry exporters.

## What Changes

- Define implementation-free metrics and logging adapter DTOs/contracts for safe metric samples, log records, labels, units, levels, event correlation, and sink outcomes.
- Add pure deterministic projections from `RuntimeEvent` and `RunReport` into bounded metric samples and redacted log records.
- Wire the runtime shell to flush newly emitted events through injected observe-only middleware or observability adapters without reordering the report event stream.
- Add standard-library no-op and in-memory observability adapters, with JSONL/tracing/exporter implementations kept as optional host-owned shell work.
- Specify best-effort default sink failures and an explicit fail-closed mode only for audit-grade policies that opt into it.
- Add positive and negative tests for safe projection, event ordering, best-effort sink failure, fail-closed audit policy, bounded labels, and leak rejection.

## Impact

- **Files**: future `animus-adapters` observability DTOs/traits, `animus-runtime` telemetry projection and observer wiring, `animus-std` sink implementations, docs, tests, and Cairn artifacts.
- **Testing**: focused positive tests for metrics/log projection and in-memory sinks; negative tests for unsafe payloads, overlong labels/messages, sink unavailable, observer failures, and fail-closed policy behavior.
- **Non-claims**: this does not add a default external telemetry backend, global tracing subscriber, network exporter, durable log store, UI dashboard, raw transcript logging, or provider/tool payload persistence.
