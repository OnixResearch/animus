# agent-harness-runtime Change Spec: Boundary DTO versioning and normalization

## Requirements

### Requirement: Stable boundary contract identities

r[agent_harness_runtime.boundary_versions.contract_identities] Public model, tool, model-stream, capability, runtime-event, and safe-receipt boundary DTOs MUST expose stable contract identity strings that can be used in docs, receipts, fixtures, and optional serialization bridges.

#### Scenario: Boundary identity is available without concrete adapters

r[agent_harness_runtime.boundary_versions.contract_identities.validation]
- GIVEN an embedder depends on reusable Animus boundary crates
- WHEN they construct or inspect public model, tool, stream, capability, runtime-event, or receipt DTOs
- THEN each maintained DTO exposes a non-empty stable contract identity
- AND obtaining that identity does not require concrete providers, filesystem access, network access, process execution, credentials, or plugin loading.

### Requirement: Explicit text normalization policy

r[agent_harness_runtime.boundary_versions.normalized_text] Adapter-boundary DTO constructors MUST store normalized text for safe labels, names, identifiers, schema identities, capability labels, and receipt metadata, while deliberately raw bodies MUST use explicit bounded raw/body policy.

#### Scenario: Safe labels normalize while raw bodies remain deliberate

r[agent_harness_runtime.boundary_versions.normalized_text.validation]
- GIVEN boundary constructor input contains leading or trailing whitespace, NUL characters, overlong text, or raw body content
- WHEN DTO validation runs
- THEN safe labels and identifiers are trimmed and stored in normalized form
- AND NUL characters, overlong text, and unsafe raw text leakage are rejected unless a bounded raw/body policy explicitly admits the body.

### Requirement: Supported version admission

r[agent_harness_runtime.boundary_versions.version_admission] DTO consumers that parse contract identity values MUST admit supported versions explicitly and MUST fail closed on missing or unsupported versions unless a reviewed compatibility mapping exists.

#### Scenario: Unsupported version fails before dispatch

r[agent_harness_runtime.boundary_versions.version_admission.fail_closed]
- GIVEN a host, serializer, replay, or adapter bridge receives a DTO with a missing or unsupported contract identity
- WHEN version admission runs
- THEN parsing fails with a typed version diagnostic
- AND no adapter dispatch, success receipt, or serialization compatibility claim is produced from the unsupported shape.

### Requirement: Boundary versioning verification

r[agent_harness_runtime.boundary_versions.verification] Boundary versioning and normalization MUST include focused positive and negative tests for identity availability, normalization, raw-body policy, unsupported version rejection, Preserves fixtures where applicable, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers boundary versioning

r[agent_harness_runtime.boundary_versions.verification.closeout]
- GIVEN maintainers intend to archive boundary versioning and normalization
- WHEN closeout validation runs
- THEN evidence names contract identities, normalization, raw-body policy, version admission, Preserves fixtures where applicable, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
