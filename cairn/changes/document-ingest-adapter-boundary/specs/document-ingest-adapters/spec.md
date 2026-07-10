# Document Ingest Adapters Specification

## Purpose

Define a bounded host-owned document-ingest adapter boundary for safe agent context intake.

## Requirements

### Requirement: Document ingest report DTO
r[document-ingest-adapters.report-dto] Animus MUST define provider-neutral document-ingest report DTOs that carry source artifact identity, admitted format, extractor family, extracted sections, optional table cells, metadata, truncation status, diagnostics, redaction labels, and explicit non-claims.

#### Scenario: Supported extraction report validates
r[document-ingest-adapters.positive]
- GIVEN a host adapter produces a document-ingest report for an admitted local document format
- WHEN Animus validates the report DTO
- THEN validation MUST accept the source identity, format label, extracted section summaries, metadata, budget status, diagnostics, and non-claim boundary without requiring parser execution in reusable core crates.

### Requirement: Reusable core validates only typed inputs
r[document-ingest-adapters.core-validation] Animus reusable crates MUST validate document-ingest DTOs as already-loaded typed inputs and MUST NOT read files, execute parsers, spawn processes, inspect the environment, access clocks, or perform network I/O.

#### Scenario: Host shell owns extraction effects
r[document-ingest-adapters.host-shell]
- GIVEN a document-ingest adapter reads a local file or invokes an optional extractor
- WHEN the report is produced
- THEN filesystem access, extractor execution, timeout enforcement, process isolation, redaction, and receipt writes MUST remain in the host-owned shell.

### Requirement: Unsafe or unsupported intake fails closed
r[document-ingest-adapters.negative] Animus MUST reject document-ingest reports that are unsupported, stale, over budget, malformed, privacy unsafe, or missing required non-claims.

#### Scenario: Malformed extraction report is rejected
- GIVEN a document-ingest report declares an unsupported format, mismatched source hash, over-budget output without truncation status, parser failure without diagnostics, missing non-claims, raw absolute host path, or trusted-content label
- WHEN Animus validates the report
- THEN validation MUST fail with deterministic diagnostics and MUST NOT admit the report as agent context.

### Requirement: Document content remains untrusted context
r[document-ingest-adapters.validation] Animus MUST treat accepted document-ingest reports as untrusted context artifacts rather than proof of document truth, parser correctness, semantic completeness, or safe execution.

#### Scenario: Accepted report preserves non-claims
- GIVEN a document-ingest report passes validation
- WHEN runtime context assembly consumes it
- THEN the resulting context reference MUST preserve non-claims for parser soundness, extraction completeness, OCR coverage, semantic truth, safety of embedded content, release eligibility, and downstream task correctness.