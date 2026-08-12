# agent-harness-runtime Change Spec: Add metrics and logging adapters

## Requirements

### Requirement: Implementation-free observability adapter contracts

r[agent_harness_runtime.observability_adapters.contracts] Animus MUST define implementation-free observability contracts for metrics and logging that represent safe metric samples, log records, labels, units, levels, sink requests, sink responses, and sink failure classes without depending on concrete logging frameworks, global subscribers, files, network exporters, clocks, storage, or UI state from reusable crates.

#### Scenario: Host injects a metrics and logging sink

r[agent_harness_runtime.observability_adapters.contracts.positive]
- GIVEN a host wants runtime metrics and logs
- WHEN it composes Animus runtime with observability support
- THEN it can inject a metrics adapter, logging adapter, or equivalent combined observability adapter through typed contracts
- AND reusable runtime code does not construct a concrete sink.

#### Scenario: Unsafe observability DTOs are rejected

r[agent_harness_runtime.observability_adapters.contracts.negative]
- GIVEN an observability DTO has an empty metric name, invalid label key, overlong label value, unbounded message, or unsafe redaction state
- WHEN the DTO is constructed or validated
- THEN validation fails with a typed contract error
- AND no sink request is produced.

### Requirement: Pure safe projection from runtime events

r[agent_harness_runtime.observability_adapters.projection] Animus MUST provide pure deterministic projection from `RuntimeEvent` and `RunReport` into safe metric samples and log records using event kind, sequence, terminal status, adapter component, failure class, middleware boundary, bounded counts, safe labels, redaction markers, and BLAKE3 fingerprints when correlation is needed.

#### Scenario: Runtime lifecycle projects to safe metrics and logs

r[agent_harness_runtime.observability_adapters.projection.events]
- GIVEN a run emits start, turn, model/tool, failure, final-response, stop, max-turn, cancellation, compaction, and shutdown events
- WHEN observability projection runs
- THEN it emits bounded metrics and log records classified by stable event and status labels
- AND it performs no file I/O, clock reads, global logging, network export, storage write, or adapter dispatch.

#### Scenario: Projection rejects raw private material

r[agent_harness_runtime.observability_adapters.projection.redaction]
- GIVEN runtime events contain model requests, model responses, tool requests, tool responses, provider payloads, credentials, headers, environment values, full paths, or unbounded text
- WHEN projection builds metrics or log records for sinks
- THEN raw private material is omitted, redacted, bounded, or represented only by approved BLAKE3 fingerprints and safe labels
- AND unsafe sink records fail validation before delivery.

### Requirement: Ordered runtime observer delivery

r[agent_harness_runtime.observability_adapters.runtime_wiring] The runtime shell MUST deliver newly emitted runtime events to injected observe-only middleware or observability adapters in the same order as `RunReport::events()` and MUST NOT allow observer delivery to mutate, reorder, or drop events from the returned report.

#### Scenario: Observer order matches report order

r[agent_harness_runtime.observability_adapters.runtime_wiring.ordering]
- GIVEN a run emits model, tool, final-response, stop, or max-turn lifecycle events
- WHEN observer delivery is enabled
- THEN observer records preserve the same event-kind sequence as the returned `RunReport`
- AND the report remains available even when no observer is configured.

#### Scenario: Best-effort sink failure does not break runtime by default

r[agent_harness_runtime.observability_adapters.runtime_wiring.best_effort]
- GIVEN a metrics or logging sink is unavailable under the default best-effort policy
- WHEN the runtime emits an observability record
- THEN runtime records a bounded observability diagnostic or sink failure class
- AND it continues without making extra model or tool adapter calls.

#### Scenario: Audit-grade mode can fail closed

r[agent_harness_runtime.observability_adapters.runtime_wiring.fail_closed]
- GIVEN a host enables an audit-grade observability policy that requires sink delivery before guarded adapter dispatch
- WHEN required metrics or log delivery fails
- THEN runtime fails closed with a typed observability failure before the guarded model or tool adapter is called
- AND the failure evidence excludes raw prompt, tool, provider, credential, path, and unbounded log material.

### Requirement: Standard-library observability sinks stay host-owned

r[agent_harness_runtime.observability_adapters.std_sinks] `animus-std` MUST provide no-op and in-memory observability sink implementations for deterministic tests and embedding, while JSONL, `tracing`, OpenTelemetry, Prometheus, or other exporters MUST remain optional host-owned shell features or downstream adapters rather than reusable runtime dependencies.

#### Scenario: Deterministic std sinks support tests

r[agent_harness_runtime.observability_adapters.std_sinks.tests]
- GIVEN tests run without external telemetry configuration
- WHEN no-op or in-memory observability sinks are selected
- THEN safe metric samples and log records can be collected or ignored deterministically
- AND unavailable, closed, or unsafe sink behavior returns typed failures without external side effects.

### Requirement: Observability adapter verification

r[agent_harness_runtime.observability_adapters.verification] Observability adapter work MUST include focused positive and negative tests for DTO validation, projection, event ordering, default best-effort failures, opt-in fail-closed audit behavior, std no-op and in-memory sinks, leak rejection, workspace checks, adapter/facade safety rails, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers observability adapters

r[agent_harness_runtime.observability_adapters.verification.closeout]
- GIVEN maintainers intend to archive the observability adapter change
- WHEN closeout validation runs
- THEN evidence names focused DTO, projection, runtime wiring, std sink, leak rejection, facade inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped external telemetry backend is recorded as a non-claim with a blocker and next-best validation.
