# agent-harness-runtime Change Spec: Preserves boundary schemas

## Requirements

### Requirement: Schema coverage tiers

r[agent_harness_runtime.preserves_boundary.coverage_tiers] Animus MUST classify Preserves payload coverage as internal DTO round trip, stable external wire contract, compatibility import/export adapter, or review-only fixture before publishing schema compatibility expectations.

#### Scenario: Boundary declares compatibility level

r[agent_harness_runtime.preserves_boundary.coverage_tiers.declared]
- GIVEN a payload family crosses an adapter, runtime, session, provenance, policy, dataset, or external protocol boundary
- WHEN maintainers add Preserves coverage
- THEN the payload family declares its coverage tier
- AND stable external wire contracts have stricter versioning and fixture requirements than internal DTO round trips.

### Requirement: Versioned schema identities

r[agent_harness_runtime.preserves_boundary.versioning] Stable Preserves boundary payloads MUST carry or bind a schema family and version identity with explicit additive, breaking, deprecated, and unsupported-version behavior.

#### Scenario: Unsupported version fails closed

r[agent_harness_runtime.preserves_boundary.versioning.unsupported]
- GIVEN a Preserves payload has an unsupported schema version
- WHEN boundary decoding runs
- THEN decoding returns a typed unsupported-version error
- AND the runtime does not reinterpret the payload as a different family.

### Requirement: Positive round-trip fixtures

r[agent_harness_runtime.preserves_boundary.round_trip] Stable Preserves boundary work MUST include positive text and binary round-trip fixtures for valid adapter messages, runtime events, final reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows when those families are in scope.

#### Scenario: Valid payload round trips

r[agent_harness_runtime.preserves_boundary.round_trip.valid]
- GIVEN a valid boundary payload fixture
- WHEN it is encoded and decoded through supported Preserves forms
- THEN the decoded typed Rust value is equivalent to the source value
- AND canonical fixture output is deterministic.

### Requirement: Malformed payload rejection

r[agent_harness_runtime.preserves_boundary.malformed] Stable Preserves boundary work MUST include negative fixtures for wrong kind, missing required fields, invalid text, unsupported versions, unbounded payloads, unsafe raw material, and compatibility leak attempts.

#### Scenario: Malformed payload is rejected safely

r[agent_harness_runtime.preserves_boundary.malformed.rejects]
- GIVEN a malformed, wrong-kind, wrong-version, unbounded, or unsafe Preserves payload
- WHEN boundary decoding runs
- THEN decoding fails with a typed boundary error
- AND diagnostics omit raw private payload bodies.

### Requirement: Unknown and future fields

r[agent_harness_runtime.preserves_boundary.unknown_fields] Animus MAY preserve unknown or future fields only when they are bounded, structurally safe metadata; otherwise boundary code MUST fail with typed compatibility errors.

#### Scenario: Unsafe unknown field is rejected

r[agent_harness_runtime.preserves_boundary.unknown_fields.rejects_unsafe]
- GIVEN a payload includes an unknown field carrying raw prompt, credential, provider JSON, full path, file body, tool input/output, or unbounded content
- WHEN compatibility handling evaluates the field
- THEN the field is rejected or redacted according to policy
- AND it is not silently preserved into model-visible context, receipts, or review output.

### Requirement: Core isolation

r[agent_harness_runtime.preserves_boundary.core_isolation] Core business logic MUST consume typed Rust values after Preserves boundary validation and MUST NOT make policy, runtime, or provenance decisions by inspecting raw Preserves syntax trees directly.

#### Scenario: Raw AST stays at boundary

r[agent_harness_runtime.preserves_boundary.core_isolation.no_ast_logic]
- GIVEN a behavior decision depends on adapter, runtime, policy, session, provenance, or dataset fields
- WHEN maintainers inspect the implementation
- THEN raw Preserves parsing and validation is isolated to boundary modules
- AND the decision logic receives typed validated values.

### Requirement: Preserves closeout evidence

r[agent_harness_runtime.preserves_boundary.closeout] Preserves schema work MUST run positive round-trip fixtures, negative malformed fixtures, unknown-field tests, core-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Schema evidence is reviewable

r[agent_harness_runtime.preserves_boundary.closeout.evidence]
- GIVEN Preserves boundary work is ready to archive
- WHEN reviewers inspect evidence
- THEN fixture, compatibility, isolation, workspace, and Cairn gate checks are named with status
- AND unavailable checks include a blocker and next best validation path.
