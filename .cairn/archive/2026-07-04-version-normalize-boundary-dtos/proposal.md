# Proposal: Version and normalize boundary DTOs

## Why

Animus has many typed boundary DTOs, but not every public request, response, stream, receipt, and adapter payload carries an explicit contract identity. Some constructors validate normalized text while preserving raw input text, which makes it harder to know when raw text retention is deliberate. As adapter surfaces grow, embedders need stable version identifiers and consistent text normalization rules.

This change adds versioned contract identities and makes text normalization/raw-retention policy explicit at adapter boundaries.

## What Changes

- Define stable contract identity constants for model requests/responses, tool requests/responses, stream frames, capability descriptors, runtime event envelopes, and safe receipts.
- Add DTO accessors or receipt fields that expose contract identity without requiring serialization-specific crates.
- Audit boundary constructors so required safe labels store normalized text, while raw bodies use explicit raw/body types and bounded policies.
- Add Preserves/golden fixture checks for versioned DTO shape where serialization exists.
- Add tests for whitespace normalization, NUL rejection, overlong values, raw-body retention, version mismatch, and unknown future version handling.

## Impact

- **Files**: adapter DTO constructors, contract identity constants, Preserves schema notes/fixtures, runtime/event/receipt docs, tests, and Cairn evidence.
- **Testing**: positive versioned DTO construction and round trips; negative malformed text, unsafe raw retention, and version mismatch cases.
- **Non-claims**: this does not require live providers, runtime Nickel evaluation, or a new wire format for every DTO in one step.
