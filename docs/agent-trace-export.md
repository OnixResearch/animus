# Agent trace export

The `animus-adapter-trace-export` crate owns bounded projection of semantic agent timelines and content-free performance accounting. It is a pure no-std decision core.

## Boundary

- `validate_trace` checks one agent timeline: known kinds, bounded content, and a strictly monotonic per-agent journal sequence.
- `bound_item` applies lite or full content mode. Lite mode bounds each item to 4 KiB and retains the exact complete-content length for metrics.
- The per-agent journal sequence is the only causality authority. Cross-agent wall time is an overview, never causality.
- `project_performance_accounting` emits one content-free summary per agent with provider prompt counts, fixed-point cache ratios, and explicit missing-evidence markers.
- `build_trace_receipt` emits a metadata-only receipt.

## Use

```rust
use animus_adapter_trace_export::{AgentTrace, TraceContentMode, validate_trace};
```

Validate a timeline before any export is accepted. Watch the positive and negative tests in `src/tests.rs`.

## Non-claims

- This is not provider billing reconciliation.
- This is not a full OTLP exporter.
- This is not proof of behavioral correctness or release eligibility.
