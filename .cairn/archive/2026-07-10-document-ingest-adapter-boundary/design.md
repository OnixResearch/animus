## Design

1. Add a provider-neutral `DocumentIngestReport` DTO family under the adapter boundary. The report includes source artifact identity, admitted format, extractor family label, page/section summaries, optional table cells, metadata, truncation status, diagnostics, and non-claims.
2. Keep `animus-core`, `animus-adapters`, and `animus-runtime` free of filesystem reads and parser execution. They validate DTO shape, budgets, and receipt boundaries only.
3. Keep `animus-std` as the imperative shell for reading files, invoking optional extractors, enforcing host limits, applying redaction, and writing receipts.
4. Model budgets as named policy fields: input byte cap, output byte cap, maximum document count, per-extraction timeout class, and report payload cap. No hard-coded thresholds are introduced in reusable core.
5. Treat unsupported and detected-but-unsupported formats as first-class safe failures. Unknown bytes do not silently fall back to text.
6. Make process isolation an adapter capability label, not a sandbox claim. Receipts state whether isolation was requested, whether it ran, and that OS/container limits remain host-owned.
7. Add fixture rows for positive DTO validation and negative rejection of unsupported format, stale source hash, over-budget extraction, parser error, missing non-claims, raw absolute path leakage, and unsafe trust labels.

## Non-Goals

- No mandatory dextract/dpdf dependency in reusable crates.
- No OCR, remote extraction service, arbitrary archive recursion, macro execution, or trust in extracted text.
- No claim that extracted document text is complete, semantically correct, or safe to execute.