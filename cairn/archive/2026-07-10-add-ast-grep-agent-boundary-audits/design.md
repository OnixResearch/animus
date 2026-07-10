## Design

1. Define ast-grep audit profiles by Animus boundary: no-std core, adapter DTOs, runtime orchestration, std shell, Steel loop fabric, TUI host, and generated config surfaces.
2. Start with inventory-only rules for ambient authority, direct process/filesystem/network calls, unsafe blocks, panics, direct provider calls, and host effects inside pure reducers.
3. Bind every scan receipt to ast-grep version, rule bundle BLAKE3 identity, scan scope, finding summary, and non-claim labels.
4. Require positive and negative fixtures before warning or blocking posture.
5. Preserve findings as safe receipts or conformance-index entries without claiming runtime safety or behavioral correctness.
6. Keep ast-grep invocation in host-owned scripts or shell rails; reusable crates must not depend on scan state.
7. Allow future codemod recipes only as dry-run migration artifacts with focused checks and reviewed diffs.

## Non-Goals

- No replacement for Octet, Nickel config checks, Steel policy checks, adapter conformance rails, or release-evidence indexes.
- No proof that a clean scan makes an agent run safe.
- No automatic edits to generated config artifacts without source-of-truth regeneration.
