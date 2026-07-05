# Qualifier-inspired quality records

Animus treats Qualifier as a bounded reference for safe ambient review annotations, deterministic record IDs, append-only sidecar interoperability, annotation lifecycle state, VCS-neutral span freshness, and compact AGENTS-CLI-style guidance. Qualifier is not a core runtime dependency, session ledger, transcript store, Git-only diff engine, or scoring authority.

The reusable surface lives in `animus-runtime::quality_records` and is re-exported by `animus-runtime`. Filesystem sidecars, JSONL storage, clocks, Git, discovery, and CLI execution remain host/adapter concerns.

## Safe envelopes

`QualityRecordEnvelope` stores version, record type, subject, issuer URI, optional issuer classification, shell-supplied creation label, BLAKE3 content-addressed ID, and typed body. Canonicalization sorts references, superseded IDs, tags, and metadata fields so equivalent source ordering produces the same ID.

Validation rejects unsupported versions, empty subjects, empty or non-URI issuers, unknown issuer classifications, mismatched IDs, missing body fields, invalid spans, and unsafe body markers.

## Sidecar boundary

`export_quality_sidecar_record` projects a validated safe record into sidecar-compatible metadata without exposing raw sensitive content. Unknown future sidecar lines are represented by `QualityUnknownSidecarRecord`, which preserves a BLAKE3 raw-line hash and round-trip intent while keeping raw unknown bodies out of reusable core. `quality_dependency_isolation_report` records that default runtime contracts do not require Qualifier crates, sidecar discovery, Git diff libraries, or sidecar compaction.

## Lifecycle

`active_quality_records` supports references, supersession, resolve records, waiver records, same-subject checks, duplicate-ID diagnostics, self-reference rejection, and cycle detection. References add thread context without closing either record. Supersession hides only prior same-subject records from the active set. `report_quality_after_edit` surfaces active records after edits without emitting resolve records automatically.

## Span freshness

`classify_quality_span_freshness` hashes full caller-supplied lines with BLAKE3 and classifies spans as fresh, drifted, missing, or no-hash. File reading is shell-owned. Invalid one-indexed ranges fail closed with typed diagnostics.

## Agents guidance

`resolve_quality_agents_guidance` provides compact deterministic markdown pages for orientation, concepts, workflows, and pitfalls. Unknown topics return a structured error listing every available topic, and guidance lookup never reads network resources.

## Focused verification

Run the focused tests with:

```sh
nix develop -c cargo test -p animus-runtime quality_records
```

The tests cover bounded scope, dependency isolation, unknown sidecar passthrough, deterministic canonical IDs, malformed envelope rejection, unsafe body rejection, sidecar export, lifecycle references and supersession, no-auto-resolution behavior, span freshness states, invalid spans, and deterministic agents guidance.
