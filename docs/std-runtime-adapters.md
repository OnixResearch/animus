# Std runtime adapters

`animus-std` owns concrete adapter boundaries for host effects. Reusable crates (`animus-core`, `animus-adapters`, `animus-runtime`, and `animus-steel`) continue to consume typed DTOs and injected services only.

## Deterministic default path

Default checks use deterministic fixtures for model, filesystem, process, network, clock, and credential behavior. Fixture adapters return safe labels and safe receipts only: raw prompts, provider payloads, credentials, absolute paths, raw tool bodies, and raw model outputs are represented by BLAKE3 hashes, counts, status classes, and redaction markers.

Before a concrete adapter performs a host effect, the shell plans the effect with `EffectPrerequisites`. Missing provider metadata, missing tool contracts, policy denial, sandbox mismatch, missing receipts/events, absent host approval, missing credentials, malformed output, timeout, or cancellation all fail closed before dispatch.

## Context and observability shells

`animus-std` includes deterministic, opt-in context and observability shells:

- `NullContextResolver` denies every context role without fallback authority.
- `LocalFileContextResolver`, `LocalDirectoryContextResolver`, and `LocalDiffContextResolver` are policy-gated local shells for already-admitted roots and preloaded diff text. They reject path escapes, binary or oversized bodies, missing files, unavailable roots, and denied roles; URL fetching, image decoding, crawlers, credentialed remote handles, and global discovery remain absent by default.
- `NoopObservabilitySink` and `InMemoryObservabilitySink` accept only validated `MetricSample` and `LogRecord` DTOs. JSONL, `tracing`, OpenTelemetry, Prometheus, dashboards, global subscribers, durable log stores, and network exporters are host-owned optional adapters.

Runtime observability projection is pure over `RuntimeEvent` and `RunReport`. Observe-only middleware receives events in report order; failures are best-effort by default and add bounded `observe` diagnostics without extra model/tool dispatch.

## Opt-in live smoke tests

Live provider calls, real filesystem mutation, process execution, and HTTP/network access are not part of deterministic default checks. A host may add explicit smoke tests only when all of the following are true:

- the test is outside reusable crates and owned by `animus-std` or a host crate;
- credentials and network access are supplied through host configuration, not process-global assumptions;
- policy, sandbox, receipt, runtime-event, and host-approval prerequisites are checked before dispatch;
- evidence stores only safe labels, bounded counts, BLAKE3 hashes, status classes, issue codes, and redaction markers;
- failure cases for missing credentials, denied capabilities, sandbox mismatch, malformed output, timeout, cancellation, and leak attempts remain covered by deterministic tests.

If live configuration is unavailable, closeout evidence should say that deterministic fixture tests passed and live smoke tests were intentionally not run.
