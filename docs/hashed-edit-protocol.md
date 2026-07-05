# Hashed edit protocol

Animus hashed edits use a pure check-and-set contract for host-owned text mutation. The reusable contract validates only in-memory snapshot metadata and edit requests; host adapters still own file reads, locks, writes, permission checks, and atomic replacement.

## Contract

A host supplies a `HashedEditSnapshot` with a safe resource class, safe resource identity, full-content BLAKE3 snapshot hash, text normalization label, line count, tag-generation metadata, and tagged line descriptors. Each tag maps to one one-based line number and one BLAKE3 line/tag-input hash.

A request supplies the expected snapshot hash, the current host-reported snapshot hash, a newline policy, and ordered tagged spans with bounded replacement text. Validation fails before host mutation when the current hash differs from the expected hash, tags are unknown or ambiguous, spans are reversed, duplicated, overlapping, or out of order, newline policy is unsupported, or replacement text exceeds the adapter field bound.

## Host boundary

`HashedEditHost` receives only a validated `HashedEditPlan`. The plan contains ordered spans and bounded replacement text, but it performs no filesystem, process, clock, environment, terminal, or persistence work. Hosts may apply the plan atomically where supported and then produce a safe receipt.

## Receipts

`HashedEditReceipt` records the resource class, redaction mode, omitted/relative/hash-only resource identity, before and after snapshot hashes, plan hash, selected span metadata, replacement hashes/counts, safe diagnostics, and receipt hash. Receipts do not embed raw private file bodies, absolute private paths, URLs, credentials, provider payloads, or unbounded tool output by default.

## Non-claims

This contract does not make hashed edits the only edit mode, copy Zerostack's CRC tag format, or move filesystem mutation into reusable crates. It is a deterministic adapter boundary for safer host-owned mutation.
