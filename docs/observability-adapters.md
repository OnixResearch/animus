# Observability adapters

Animus treats metrics and logs as host-owned observability sinks fed by safe projections, not as raw runtime event dumps.

## Boundary

`animus-adapters` defines implementation-free DTOs for:

- `MetricSample`, `MetricLabel`, `MetricKind`, and `MetricUnit`;
- `LogRecord` and `LogLevel`;
- `MetricsAdapter`, `LoggingAdapter`, `ObservabilitySinkOutcome`, and sink failure classes.

DTO validation rejects empty names, invalid label cardinality, overlong message labels, unsafe redaction state, raw private paths, credential-looking values, and other unsafe metadata before a sink request is produced.

## Runtime projection

`animus-runtime` provides pure projection from `RuntimeEvent` and `RunReport` through `project_runtime_event_observability` and `project_run_report_observability`. Projection emits safe event-kind, sequence, decision, component, failure-class, hook-boundary, and terminal-status labels. It does not emit raw prompts, model responses, tool arguments or output, provider payloads, credentials, headers, environment values, full paths, or unbounded log text.

Observe-only middleware is delivered newly emitted runtime events in the same order as `RunReport::events()`. Observer failures are best-effort by default: runtime appends bounded `observe` diagnostics and continues without extra model or tool adapter calls. Audit-grade fail-closed behavior remains an explicit host policy choice before guarded dispatch.

## Std sinks

`animus-std-observability` provides deterministic sinks for tests and embedding. The default `animus-std/observability-host` feature preserves the existing facade names:

- `NoopObservabilitySink` accepts safe records and drops them.
- `InMemoryObservabilitySink` stores validated metric/log DTOs, with typed unavailable and closed-collector failures.

New embedders that need only these sinks can depend directly on `animus-std-observability`; they do not pull context, provider registry, ingest, transport, credential, persistence, or terminal behavior. See [authority-scoped std host adapters](std-host-adapter-crates.md).

External telemetry backends, JSONL stores, global tracing subscribers, OpenTelemetry, Prometheus, dashboards, network exporters, and durable log stores remain optional host-owned shell work.

## Focused verification

Run:

```sh
nix develop -c cargo test -p animus-adapters observability --locked
nix develop -c cargo test -p animus-runtime observability --locked
nix develop -c cargo test -p animus-runtime observe_only --locked
nix develop -c cargo test -p animus-std observability --locked
```

These checks cover DTO construction, invalid names, unsafe labels, leak rejection, pure runtime projection, observer order, best-effort observer failure, no-op sinks, in-memory sinks, unavailable sinks, closed collectors, and unsafe log-record attempts.
