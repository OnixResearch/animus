# agent-harness-runtime Change Spec: Preserves dataspace projections

## Requirements

### Requirement: Dataspace Preserves schema coverage

r[agent_harness_runtime.preserves_dataspace.schema] Animus MUST provide optional Preserves schema identities and typed record coverage for dataspace scopes, assertions, subscriptions, and orchestration events before those DTOs are advertised as portable external dataspace wire contracts.

#### Scenario: Dataspace records have versioned schema identities

r[agent_harness_runtime.preserves_dataspace.schema.versioned]
- GIVEN a dataspace scope, assertion, subscription, cancellation event, expiry event, or ownership-transfer event crosses a Preserves boundary
- WHEN it is encoded as a versioned Preserves record
- THEN the record carries a schema family, supported version, and typed dataspace payload kind
- AND the schema identity is available without enabling a live Syndicate transport.

### Requirement: Typed dataspace decoding

r[agent_harness_runtime.preserves_dataspace.typed_decoding] Preserves dataspace decoding MUST produce validated `animus-adapters` dataspace DTOs before any runtime, host, policy, provenance, or future live dataspace behavior observes the payload.

#### Scenario: Raw Preserves syntax stays at the codec boundary

r[agent_harness_runtime.preserves_dataspace.typed_decoding.no_ast_logic]
- GIVEN a behavior decision depends on dataspace scope, assertion status, subscription shape, cancellation, expiry, or ownership transfer
- WHEN Preserves decoding succeeds
- THEN the decision receives typed validated dataspace DTOs
- AND reusable core, runtime, adapter, policy, or host orchestration logic does not inspect raw Preserves syntax trees.

### Requirement: Safe dataspace payload set

r[agent_harness_runtime.preserves_dataspace.safe_payloads] Dataspace Preserves records MUST contain only bounded safe coordination material such as stable IDs, normalized scope labels, status classes, assertion kinds, redaction markers, BLAKE3 hashes, counts, lease/expiry metadata, and ownership-transfer labels.

#### Scenario: Sensitive material is rejected before broadcast

r[agent_harness_runtime.preserves_dataspace.safe_payloads.rejects_raw]
- GIVEN a Preserves dataspace record attempts to carry prompt text, provider payloads, credentials, full private paths, file bodies, raw tool input/output, model output, or unbounded logs
- WHEN dataspace Preserves validation runs
- THEN decoding fails with a typed safe-payload diagnostic
- AND no dataspace assertion, subscription result, orchestration event, success receipt, or compatibility metadata echoes the raw material.

### Requirement: Dataspace Preserves round trips

r[agent_harness_runtime.preserves_dataspace.round_trip] Dataspace Preserves work MUST include positive text and binary round-trip fixtures for valid scope, assertion, subscription, cancellation, expiry, and ownership-transfer records.

#### Scenario: Valid dataspace fixture round trips

r[agent_harness_runtime.preserves_dataspace.round_trip.valid]
- GIVEN a valid dataspace DTO fixture with safe scope and redaction metadata
- WHEN it is encoded and decoded through supported Preserves text and binary forms
- THEN the decoded typed DTO is equivalent to the source value
- AND ordering, scope, status class, redaction markers, and safe hashes are preserved.

### Requirement: Dataspace Preserves malformed rejection

r[agent_harness_runtime.preserves_dataspace.malformed] Dataspace Preserves work MUST include negative fixtures for unsupported versions, wrong schema families, missing required fields, unsafe compatibility metadata, cross-scope subscriptions, raw sensitive material, and unbounded payloads.

#### Scenario: Malformed dataspace payload fails safely

r[agent_harness_runtime.preserves_dataspace.malformed.rejects]
- GIVEN a malformed, wrong-version, wrong-family, cross-scope, unsafe, or unbounded dataspace Preserves payload
- WHEN boundary decoding runs
- THEN decoding fails with a typed validation or compatibility error
- AND no partial assertion publish, subscription match, orchestration event, or live transport side effect occurs.

### Requirement: Dataspace Preserves core isolation

r[agent_harness_runtime.preserves_dataspace.core_isolation] Dataspace Preserves support MUST NOT add Preserves dependencies to `animus-core`, `animus-adapters`, default `animus-runtime`, or default `animus-std` dependency graphs.

#### Scenario: Optional bridge does not leak dependencies

r[agent_harness_runtime.preserves_dataspace.core_isolation.no_leak]
- GIVEN dependency isolation checks inspect reusable default crates
- WHEN dataspace Preserves support is present in the workspace
- THEN Preserves serialization dependencies remain isolated to the optional Preserves bridge or explicit opt-in facade features
- AND implementation-free dataspace adapter contracts remain usable without Preserves.

### Requirement: Dataspace Preserves documentation and closeout

r[agent_harness_runtime.preserves_dataspace.closeout] Dataspace Preserves work MUST update schema notes and facade inventory, run positive and negative fixtures, run dependency-isolation checks, run workspace checks, and record Cairn gate evidence before archive.

#### Scenario: Dataspace Preserves evidence is reviewable

r[agent_harness_runtime.preserves_dataspace.closeout.evidence]
- GIVEN dataspace Preserves work is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN evidence names schema coverage, typed decoding, safe-payload rejection, round-trip fixtures, malformed fixtures, dependency isolation, workspace checks, and Cairn gates
- AND any skipped live-transport validation is marked as out of scope rather than a release claim.
