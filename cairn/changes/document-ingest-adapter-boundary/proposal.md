## Why

Animus needs host-owned tool adapters that can ingest local documents for agent context without giving the reusable core filesystem, process, network, OCR, or parser authority. The dextract repository is a strong reference for fail-closed format detection, explicit unsupported-format boundaries, extraction budgets, batch JSON reports, and process-isolation caveats.

Reference sources: `https://github.com/joweeba/dextract`, `https://github.com/joweeba/dpdf`, and `https://github.com/joweeba/dKNOW`.

## What Changes

- Define an Animus document-ingest adapter contract for bounded text/table/metadata extraction reports.
- Keep extractor execution in host-owned `animus-std` adapters; core/runtime receive typed DTOs and safe receipts only.
- Require explicit format admission, unsupported-format diagnostics, input-size limits, extracted-text budgets, redaction class labels, and no raw path leakage in reusable receipts.
- Add positive fixtures for supported document report DTOs and negative fixtures for unsupported formats, stale hashes, over-budget inputs, parser failure, and weakened non-claims.

## Impact

- **Surfaces**: `animus-adapters` DTOs, `animus-std` host adapter, context-reference resolver integration, safe-receipt artifacts, docs/contracts index.
- **Non-claims**: This does not adopt dextract/dpdf as required dependencies, prove parser correctness, perform OCR, or make document contents trusted.
- **Validation**: adapter conformance fixtures, safe-receipts checks, positive/negative DTO tests, Cairn gates/validate.