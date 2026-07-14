# Preserves boundary schemas

Animus treats Preserves as a typed boundary format, not as core business logic. Family-specific crates and modules decode external values into Rust DTOs, re-run the owning DTO validators, and only then return data to adapter, runtime, policy, provenance, or export logic.

The broad `animus-preserves` package remains a compatibility facade. New adapter-only consumers should depend on `animus-preserves-adapter`; schema tooling that needs no codec should depend on `animus-preserves-schema`.

## Family ownership matrix

| Family | Owner | Explicit dependencies | Coverage | Safe metadata | Fixture set / check |
| --- | --- | --- | --- | --- | --- |
| adapter request/response | `animus-preserves-adapter` | `animus-adapters`, `animus-preserves-schema`, `preserves`, `serde` | stable text and binary wire | bounded `CompatibilityField` values; no raw prompts, provider JSON, credentials, private paths, file bodies, raw tool bodies, or unbounded output | `preserves.adapter.v1`; `cargo test -p animus-preserves-adapter` |
| runtime event/report/error | `animus_preserves::runtime` compatibility module | adapter bridge, schema primitives, `animus-runtime`, `preserves`, `serde` | stable event/report wire; compatibility error payloads | bounded safe metadata and owning runtime/adapter DTO revalidation | `preserves.runtime-event.v1`, `preserves.runtime-report.v1`, `preserves.error.compat.v1`; `cargo test -p animus-preserves` |
| session/provenance | `animus_preserves::provenance` | schema primitives, `animus-runtime`, `preserves`, `serde` | compatibility import/export | safe session labels and hash identities only; stack-owned producers should use BLAKE3; no transcript or storage bodies | `preserves.session-provenance.compat.v1`; `cargo test -p animus-preserves provenance` |
| policy-visible DTO | `animus_preserves::policy` | `animus-adapters`, schema primitives, `preserves`, `serde` | compatibility import/export | safe policy hashes, decision classes, reason classes, and bounded metadata only | `preserves.policy-visible.compat.v1`; `cargo test -p animus-preserves policy` |
| dataspace DTO | `animus_preserves::dataspace` | `animus-adapters`, schema primitives, `preserves`, `serde` | stable text and binary wire | bounded scope IDs, assertion labels, counts, leases, hashes, and redaction markers | `preserves.dataspace.v1`; `cargo test -p animus-preserves dataspace` |
| Syndicate turn | `animus_preserves::syndicate` | `animus-adapters`, schema primitives, `preserves`, `serde` | stable capability-free `A`/`R`/`M` turn profile | validated dataspace record bodies, non-negative OIDs/handles, bounded event count; no WireRefs, sync, caveats, extensions, errors, or arbitrary bodies | `preserves.syndicate-turn.v1`; `cargo test -p animus-preserves syndicate` |
| dataset/export row | `animus_preserves::review` | schema primitives only at the declaration layer | Review-only | fixture metadata is review evidence only | `preserves.dataset.review.v1`; no encoder or compatibility importer |

`animus_preserves::schema_catalog()` records owner, family identity, coverage tier, dependency set, safe-metadata policy, wire status, and fixture-set identity for every row.

## Smallest package selection

Use the narrowest package that owns the needed boundary:

```toml
# Adapter requests and responses only; does not pull animus-runtime.
animus-preserves-adapter = "0.1"

# Schema identities and catalog metadata only.
animus-preserves-schema = "0.1"

# Existing broad imports or runtime/dataspace/provenance/policy families.
animus-preserves = "0.1"
```

The direct adapter package intentionally has no `animus-runtime` or broad `animus-preserves` dependency. `scripts/cairn/check-preserves-boundary-schemas --root .` validates that isolation and the family catalog. The compatibility facade re-exports the direct path as `animus_preserves::adapter` while retaining the existing root helpers.

## Coverage tiers and wire claims

- `internal_dto_round_trip`: typed DTO round trips without external compatibility promises.
- `stable_external_wire_contract`: versioned external payloads with positive text and binary fixtures plus malformed-input tests.
- `compatibility_import_export_adapter`: bounded import/export payloads with explicit version and compatibility handling.
- `review_only_fixture`: durable review examples without a stable wire guarantee.

Review-only means **not stable wire**. In particular, dataset/export rows have no published encoder, decoder, migration promise, or compatibility-import claim. The absence of a stable claim is deliberate and checked.

## Versioning and compatibility

Every stable or compatibility record carries a `SchemaIdentity`:

- `family` selects the owning boundary.
- `version` selects the supported shape.
- typed payload fields carry only that family's data.
- `metadata` carries optional safe compatibility fields where the family permits them.

Decoding fails closed on unsupported versions, wrong families, malformed text or binary, missing payloads, unknown required fields, invalid owner DTOs, or unsafe compatibility material. Diagnostics name only safe field classes and reasons; they do not echo rejected prompt bodies, provider payloads, credentials, file bodies, private paths, or unbounded output.

Compatibility changes follow this sequence:

1. Add the new version and positive/negative fixtures in the owning family boundary.
2. Keep the old version decoder only when an explicit migration is implemented and tested.
3. Re-run the owning adapter/runtime validator after migration.
4. Update `animus-preserves-schema` catalog metadata and this matrix.
5. Keep review-only shapes review-only until both text and binary contracts, malformed-input tests, and migration posture are published.

Unknown fields are never ambiently accepted. Safe future metadata survives only through explicit bounded `CompatibilityField` values.

## Core-isolation invariants

- `animus-core`, adapter contracts, and runtime decision cores do not parse or write `preserves::value` directly.
- The Syndicate turn codec uses `preserves::value` only inside the family boundary to preserve exact top-level sequence and `A`/`R`/`M` record shapes.
- Codec calls remain in family boundary crates/modules.
- Adapter-only Preserves users do not gain runtime dependencies.
- Decoded DTOs pass their owning constructors before behavior decisions run.
- Schema catalogs and fixtures are evidence of boundary shape, not proof of provider behavior, runtime correctness, release eligibility, or arbitrary cross-language semantic equivalence.

## Validation

Run the focused rail:

```console
nix develop -c cargo test -p animus-preserves-schema -p animus-preserves-adapter -p animus-preserves
nix develop -c cargo test -p animus-preserves syndicate --locked
nix develop -c cargo clippy -p animus-preserves-schema -p animus-preserves-adapter -p animus-preserves --all-targets -- -D warnings
nix develop -c scripts/cairn/check-preserves-boundary-schemas --root .
```

The fixture suites cover positive text/binary round trips and negative malformed input, missing payloads, unsupported versions, wrong families, wrong kinds, invalid owning DTOs, unbounded metadata, private paths, provider JSON, raw prompt fields, and leak-safe diagnostics.
