# Design: Split adapter families into focused crates

## Context

`animus-adapters` already keeps host authority out, but it groups unrelated contract families behind one dependency. The repo has precedent for this style of split: `animus-router` was extracted from the adapter crate while preserving a transition shim. The same pattern can be repeated for adapter families that have clear owner modules and focused rails.

## Decisions

### 1. Use a wave-based split

**Choice:** Extract families in waves: safe receipts and observability first, then context/tool/workspace, then memory/policy/orchestration as their dependency edges are made explicit.

**Rationale:** Safe receipts and observability are small, central, and low-authority. Larger families have more cross-links and should move only after catalog dependency checks make those links visible.

### 2. Preserve the existing facade

**Choice:** `animus-adapters` remains a workspace crate and re-exports moved family items for a transition window.

**Rationale:** This keeps downstream imports stable and lets one family move at a time without forcing a repo-wide call-site rewrite.

### 3. New family crates stay reusable and no-std when possible

**Choice:** Extracted contract crates use `#![no_std]` with `alloc` unless the family already has an accepted reason to require std, and they forbid filesystem, process, network, environment, credential, terminal, clock, and persistence authority.

**Rationale:** Adapter-family crates should be Lego bricks that embedders can use in Wasm, tests, and provider-neutral contexts.

### 4. Every extraction updates conformance metadata

**Choice:** Each moved family must update the component catalog, contracts index, runtime facade inventory, and adapter conformance matrix in the same change wave.

**Rationale:** A split is only useful if embedders can discover it and if validation proves the facade and direct crate surfaces remain aligned.

## Risks / Trade-offs

- More crates increase workspace maintenance. Mitigate with facade re-exports, generated catalog data, and focused checks.
- Re-export shims can mask dependency cycles. Mitigate with catalog dependency validation and parity tests.
- Large families may expose hidden cross-family coupling. Mitigate by extracting smallest independent subfamilies first and recording blockers instead of forcing a move.
