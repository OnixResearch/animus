## Why

Animus persists the admitted session ledger with a monotonic journal sequence and already exposes transcript and dataset exports, but it has no dense semantic agent-trace export that a consumer can replay or audit. The trajectory adapter is transcript-only. The evaluation core holds ledger trace references. Hosts cannot export a bounded causal timeline with authoritative per-agent sequence, lite and full content modes, or a content-free performance accounting report.

A bounded trace-export adapter should own the projection DTOs, content-mode bounds, sequence authority, content-free accounting, and fail-closed validation over the admitted session ledger. Live provider clients, daemon discovery, and billing reconciliation remain out of scope.

## What Changes

- Add a versioned trace-export DTO family over the admitted session ledger with canonical native JSONL and TOON text projections. r[agent_harness_runtime.trace_export.wire]
- Add lite and full content modes; the lite mode bounds each semantic text or output item to a fixed non-zero ceiling while retaining exact complete-content metrics. r[agent_harness_runtime.trace_export.bounds]
- Treat the per-agent journal sequence as the only causality authority and cross-agent wall-clock time as readable overview only. r[agent_harness_runtime.trace_export.sequence]
- Add a content-free performance accounting projection with provider-prompt accounting, fixed-point cache ratios, and explicit missing-evidence coverage markers. r[agent_harness_runtime.trace_export.accounting]
- Reject unknown item kinds, malformed records, non-monotonic sequences, unbounded items, duplicate summaries, and unsupported export shapes with bounded safe error classes. r[agent_harness_runtime.trace_export.fail_closed]
- Emit metadata-only receipts with schema, source ledger identity, counts, content-mode, and BLAKE3 identity, rejecting raw prompts, provider payloads, credentials, and private paths. r[agent_harness_runtime.trace_export.receipts]
- Add positive and negative fixtures for projection shape, bounds, sequence, accounting, receipts, and feature isolation. r[agent_harness_runtime.trace_export.verification]

## Impact

- **Rust**: a new non-default no-std adapter family owns trace-export DTOs and validation; the admitted session ledger remains the authoritative source.
- **Exports**: consumers get replayable JSONL and TOON timelines plus a content-free performance report without provider or daemon dependencies.
- **Claims**: this is not provider billing reconciliation, a full OTLP exporter, or proof of behavioral correctness.
