# Design: Safe receipts and BLAKE3 artifact identity

## Context

Clankers keeps observability receipts as bounded metadata and uses BLAKE3 artifact identity for content correlation. Animus should copy the safety invariants while keeping its DTOs no-std-friendly where possible and independent from Clankers crates.

## Decisions

### 1. Receipts are safe metadata only

Runtime/provenance receipts include schema, status, issue code, actor/session/trace labels, bounded counts, artifact hashes, effect/mutation kind, backend label, interchange label, and redaction markers. They do not include raw prompts, provider payloads, credentials, full paths, raw tool input/output, raw model output, or unbounded data.

### 2. Artifact hashes use BLAKE3 canonical envelopes

Animus-owned content identity uses BLAKE3 over a versioned canonical envelope: kind, envelope version, redaction class, normalized payload, and sorted/deduplicated dependencies. Compatibility adapters may use required external algorithms only at boundaries.

### 3. Redaction classes govern display

Public artifacts may display payload bodies. Metadata-only, redacted-payload, and secret artifacts display only safe summaries, hashes, counts, or explicit redaction markers.

### 4. Leak validation is negative-test driven

Receipt and review renderers must have fixtures that intentionally include prompt text, provider JSON, token-like values, absolute paths, raw tool/model output, and oversized payloads. These fixtures must fail or be redacted before evidence is accepted.

## Risks / Trade-offs

- Hash fixtures can ossify too early. Use versioned envelopes and keep initial payload shapes minimal.
- Over-aggressive redaction can reduce debug value. Receipts can expose counts, labels, and hashes without exposing raw content.
- Secret detection cannot be perfect; explicit field allowlists and negative fixtures reduce risk.