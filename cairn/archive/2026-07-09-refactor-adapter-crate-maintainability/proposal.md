# Proposal: Refactor adapter crate maintainability hotspots

## Why

The adapter layer is well-covered by conformance indexes, but several modules have grown large and rely on broad lint allowances or mechanically named shards. This raises review cost and makes it harder to see which adapter family owns validation, receipts, fixtures, provider projection, prompt assembly, cost routing, memory, and tool gateway behavior.

A maintainability-focused Cairn should preserve public behavior while improving module boundaries, semantic naming, local lint justification, and conformance coverage for future adapter work.

## What Changes

- Inventory large adapter modules, blanket lint allowances, mechanical shard names, and cross-family coupling hotspots.
- Rename mechanical shards to semantic adapter-family names where public API compatibility can be preserved or migration aliases can be documented.
- Split large modules into functional-core submodules and thin host/admission shells.
- Replace broad `allow` blocks with local justifications or remove them when no longer needed.
- Add maintainability checks that keep conformance indexes and public export docs synchronized after refactors.

## Impact

- **Files**: adapter crate module tree, public re-export docs, lint attributes, conformance indexes, tests, and Cairn evidence.
- **Testing**: no behavior changes intended; run adapter conformance plus targeted unit/doc tests and negative compile/lint fixtures where available.
- **Non-claims**: this does not add new runtime behavior, provider clients, tool executors, persistence, dogfood shell execution, or product features.
