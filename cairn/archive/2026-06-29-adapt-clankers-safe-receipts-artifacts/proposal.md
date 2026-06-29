# Proposal: Adapt Clankers safe receipts and artifact identity for Animus

## Why

Animus needs evidence for runtime decisions without storing transcripts, credentials, full paths, provider payloads, raw tool bodies, or unbounded outputs. Clankers' artifact and receipt patterns show how to use BLAKE3 content identity, redaction classes, canonical envelopes, and leak-rejecting fixtures. Animus should adopt these concepts as Animus-owned DTOs.

## What Changes

- Define minimal safe runtime/provenance receipt DTOs with status, issue code, labels, bounded counts, BLAKE3 content hashes, and redaction markers.
- Define versioned canonical artifact envelopes with kind, version, redaction class, normalized payload, sorted/deduplicated dependencies, and BLAKE3 identity.
- Add validation that rejects raw prompts, provider JSON, credential-looking values, absolute path dumps, raw tool input/output, raw model output, and unbounded outputs.
- Add display/parse tests for artifact hashes and redacted review output.

## Impact

- **Files**: possible core/provenance artifact module, receipt module, tests/fixtures, docs, Cairn artifacts.
- **Testing**: positive safe receipt/artifact tests and negative leak fixtures.
- **Non-claims**: this does not add artifact storage, Toolpath export, provider session importers, or full transcript archival.