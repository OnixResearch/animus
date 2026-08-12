# agent-harness-runtime Change Spec: Split std host adapters into authority-scoped crates

## Requirements

### Requirement: Std host adapter facade remains compatible

r[agent_harness_runtime.std_host_adapter_crates.facade] Animus MUST preserve the `animus-std` facade while splitting host adapter families into authority-scoped crates or feature packs, and moved public exports MUST remain available through documented compatibility paths during migration.

#### Scenario: Existing std-shell import remains valid

r[agent_harness_runtime.std_host_adapter_crates.facade.compat]
- GIVEN downstream code imports a moved host adapter from `animus_std`
- WHEN that adapter family is moved into an authority-scoped crate or feature pack
- THEN the old `animus_std` import path continues to compile through a documented facade re-export
- AND docs identify the smaller direct host-adapter dependency for new embedders.

### Requirement: Host adapter authority is explicit

r[agent_harness_runtime.std_host_adapter_crates.authority] Each std host adapter block MUST declare its required authority class, including environment reads, filesystem reads, process execution, network transport, credential access, persistence, terminal access, or no host effect, and missing or disabled authority MUST fail closed without ambient fallback.

#### Scenario: Missing host block does not discover ambient services

r[agent_harness_runtime.std_host_adapter_crates.authority.no_fallback]
- GIVEN a runtime path requests local context, provider registry selection, document ingest, model-evidence ingest, observability delivery, transport, or persistence through a host adapter block
- WHEN the required block or feature is absent
- THEN Animus returns a typed unavailable or denied outcome
- AND it does not scan files, read environment variables, probe providers, open networks, create storage, execute processes, or access terminal state as a fallback.

### Requirement: Host adapter migration preserves pure planning

r[agent_harness_runtime.std_host_adapter_crates.migration] Moved host adapter families MUST preserve pure planning and validation helpers over explicit inputs, with host I/O isolated to thin shells after admission decisions have succeeded.

#### Scenario: Planning tests run without host effects

r[agent_harness_runtime.std_host_adapter_crates.migration.pure_plans]
- GIVEN a moved host adapter validates config, context, provider metadata, document ingest, model evidence, or observability delivery
- WHEN focused tests exercise branch decisions
- THEN tests run over in-memory values without filesystem, process, network, credential, terminal, clock, or persistence effects
- AND shell-owned effects are covered by separate adapter tests or explicit non-claims.

### Requirement: Std host adapter split verification

r[agent_harness_runtime.std_host_adapter_crates.verification] Std host adapter splits MUST include focused positive and negative tests for explicit authority admission, missing services, disabled features, ambient fallback rejection, facade compatibility, catalog metadata, and request-lifecycle documentation.

#### Scenario: Closeout evidence covers host authority boundaries

r[agent_harness_runtime.std_host_adapter_crates.verification.closeout]
- GIVEN maintainers intend to archive a std host adapter split
- WHEN closeout validation runs
- THEN evidence names host-adapter tests, ambient-fallback negative fixtures, facade compatibility, component catalog checks, request-lifecycle checks, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
