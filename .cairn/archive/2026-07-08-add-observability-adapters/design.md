# Design: Add metrics and logging adapters

## Context

`animus-runtime` already owns ordered lifecycle DTOs through `RuntimeEvent` and returns them in `RunReport`. `RuntimeMiddleware` also exposes an observe-only hook, but observer delivery is not yet the durable observability adapter family for metrics/logging sinks. The missing piece is a safe projection layer and host-owned sink boundary that prevents reusable runtime code from depending on logging frameworks, clocks, storage, network exporters, or global subscribers.

## Decisions

### 1. Observability contracts live in the adapter boundary

**Choice:** Add implementation-free metrics and logging contracts in `animus-adapters`: safe metric samples, safe log records, labels, units, levels, sink requests/responses, and `MetricsAdapter`/`LoggingAdapter` or an equivalent combined `ObservabilityAdapter` seam.

**Rationale:** Metrics and logs are host integrations. The reusable runtime should emit typed lifecycle facts and call injected contracts only; hosts choose concrete sinks.

### 2. Projection is a pure runtime core

**Choice:** `animus-runtime` projects `RuntimeEvent` and `RunReport` into safe metric/log DTOs through deterministic functions over in-memory values.

**Rationale:** Projection rules are business logic and should be testable without standing up logging frameworks, files, networks, clocks, or global state.

### 3. Safe fields are smaller than raw runtime events

**Choice:** Projection emits event kind, sequence, terminal status, adapter component, failure class, boundary labels, bounded counts, safe labels, redaction markers, and BLAKE3 fingerprints where correlation is useful. It does not emit raw prompts, model responses, tool arguments/output, provider JSON, credentials, headers, environment values, full paths, or unbounded messages.

**Rationale:** `RuntimeEvent` can carry validated request/response DTOs for in-process consumers. Observability sinks need a lower-risk public projection.

### 4. Runtime observer delivery preserves event order

**Choice:** The runtime shell flushes newly appended events to observe-only hooks or observability adapters in report order and never lets sink delivery reorder or mutate the `RunReport` event stream.

**Rationale:** Consumers need consistent correlation between reports, logs, metrics, safe receipts, and future headless streams.

### 5. Sink failures are best-effort by default

**Choice:** Metrics/logging sink failures default to best-effort: record a bounded observability diagnostic and continue runtime execution. A separate audit-grade policy may opt into fail-closed behavior before guarded actions.

**Rationale:** A broken metrics sink should not normally break model/tool runtime behavior. Audit policies that require durable evidence need explicit stronger semantics.

### 6. Concrete sinks are std-shell work

**Choice:** `animus-std` provides no-op and in-memory adapters for tests and embedding. JSONL, `tracing`, OpenTelemetry, Prometheus, or other exporters remain optional host-owned features or downstream crates.

**Rationale:** Reusable crates stay no-ambient and low-dependency while still giving hosts a canonical seam.

## Risks / Trade-offs

- Redacted log records may be less convenient than full event dumps. Keep raw detail available only to trusted in-process report consumers and provide stable safe labels/fingerprints for correlation.
- Best-effort sink failure can hide observability outages. Emit bounded diagnostics and allow opt-in fail-closed policy for audit-required contexts.
- Metrics cardinality can grow if labels are unchecked. Validate label names, label values, metric names, and log targets with explicit bounds and typed errors.
- Observer delivery inside the runtime shell adds plumbing. Keep the core projection pure and the shell flush path thin.
