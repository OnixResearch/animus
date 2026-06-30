# Preserves Boundary Schemas

Animus treats Preserves as a boundary format, not as core business logic. Stable external payloads are decoded into typed Rust DTOs before adapter, runtime, policy, provenance, or export decisions run.

## Coverage tiers

Preserves payload families declare one coverage tier before compatibility expectations are published:

- `internal_dto_round_trip`: typed DTO round trips without external compatibility promises.
- `stable_external_wire_contract`: versioned external payloads with text and binary fixtures plus malformed-input tests.
- `compatibility_import_export_adapter`: bounded import/export payloads that may preserve safe future metadata.
- `review_only_fixture`: durable review examples without a stable wire guarantee.

`animus_preserves::schema_catalog()` records the current catalog. Adapter messages, runtime events, and runtime reports are stable external wire contracts. Error payloads, session/provenance records, and policy-visible DTOs are compatibility import/export families until their standalone surfaces stabilize. Dataset/export rows remain review-only fixtures.

## Versioning

Stable records use `BoundaryRecord` around the typed boundary payload:

- `schema.family` selects the payload family.
- `schema.version` is the supported schema version.
- `payload` is the typed boundary message.
- `metadata` carries optional safe compatibility fields.

Decoding fails closed when the schema version is unsupported or when the schema family does not match the typed payload. The legacy `to_text`/`from_text` and `to_bytes`/`from_bytes` helpers remain internal DTO round trips. External callers should use `to_versioned_text`, `from_versioned_text`, `to_versioned_bytes`, or `from_versioned_bytes`.

## Compatibility metadata

Unknown or future metadata is preserved only through explicit `CompatibilityField` values. Fields are rejected when they are unbounded, named like raw prompts or credentials, look like provider JSON, contain private full paths, or otherwise fail safe identifier checks. Diagnostics identify the safe field class and reason without echoing raw metadata values.

## Fixtures and validation

The Preserves crate includes positive text and binary round-trip fixtures for stable boundary records, plus negative fixtures for invalid text, missing payloads, unsupported versions, wrong schema families, wrong kinds, unbounded metadata, unsafe raw material, and compatibility leak attempts.

Core-isolation evidence is checked by keeping raw `preserves::value` writer/parser usage in the Preserves codec boundary and by converting decoded payloads through typed adapter/runtime constructors before behavior decisions run.
