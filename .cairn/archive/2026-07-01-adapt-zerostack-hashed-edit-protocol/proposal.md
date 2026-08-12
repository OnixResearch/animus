# Proposal: Adapt Zerostack hashed edit protocol for Animus

## Why

Zerostack's `hashedit` mode addresses a common agent failure: search/replace edits are brittle when copied text is stale, partial, or whitespace-shifted. It tags read output lines and requires a file-level check-and-set token before applying edits. Animus should adapt this as an Animus-owned edit contract using BLAKE3-derived tags instead of CRC-32 and keeping filesystem mutation in host/tool adapters.

Adapting the protocol gives future tool hosts a safer edit surface with deterministic validation and clear stale-file failures.

## Scope

- Define file snapshot identity and line tag descriptors based on BLAKE3-derived hashes.
- Define edit requests that reference tagged spans and replacement text without requiring large old-text reproduction.
- Define pure edit-plan validation for stale snapshots, duplicate tags, invalid spans, overlapping edits, and newline normalization.
- Keep file reads/writes, locking, and persistence in tool host adapters.
- Emit bounded edit receipts with snapshot identity, changed span metadata, and redacted path/resource identifiers.

## Impact

- **Files**: tool contract DTOs, edit-plan validation core, host adapter interfaces, tests, and docs.
- **Testing**: positive single/multi-span edit tests; negative stale CAS, duplicate tag, overlapping span, missing tag, binary/oversize file, and unsafe path receipt tests.
- **Non-claims**: this does not copy Zerostack's CRC implementation, make hashed edits the only edit mode, or allow reusable core crates to read/write files.
