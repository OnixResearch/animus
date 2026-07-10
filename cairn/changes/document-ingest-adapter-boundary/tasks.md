## Tasks

- [ ] [serial] r[document-ingest-adapters.report-dto] Define document-ingest report DTOs carrying source artifact identity, admitted format, extractor family, sections, tables, metadata, truncation status, diagnostics, redaction labels, and non-claims.
- [ ] [depends:document-ingest-dto] r[document-ingest-adapters.core-validation] Add pure validation over already-loaded document-ingest DTOs without filesystem, process, environment, clock, parser, or network effects.
- [ ] [depends:document-ingest-validation] r[document-ingest-adapters.host-shell] Add a host-owned extraction seam for local files and optional extractors while keeping timeout enforcement, process isolation, redaction, and receipt writes in the shell.
- [ ] [depends:document-ingest-validation] r[document-ingest-adapters.positive] Add positive fixtures for admitted local document reports with bounded section summaries, metadata, budget status, diagnostics, and non-claim boundaries.
- [ ] [depends:document-ingest-validation] r[document-ingest-adapters.negative] Add negative fixtures for unsupported format, stale source hash, over-budget output, parser failure, missing non-claims, raw absolute host path, and trusted-content labels.
- [ ] [depends:document-ingest-validation] r[document-ingest-adapters.validation] Preserve document-content non-claims through context-reference assembly and runtime consumption.
- [ ] [depends:document-ingest-fixtures] r[document-ingest-adapters.validation] Run DTO tests, context-reference checks, safe-receipts checks, Cairn gates, and focused Animus validation.
