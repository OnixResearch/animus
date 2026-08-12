# Proposal: Gate runtime capability families

## Why

Animus crates are already split by broad concerns, but `animus-runtime` has grown many optional capability families: headless protocol, provenance, batch jobs, worker teams, Kanban orchestration, ResearchSpec lifecycle, quality records, compaction, trust boundaries, dataset export, and more. Embedders should be able to compose smaller runtime surfaces without paying for unrelated domains.

This change adds capability-family feature gates and compile rails that keep default reusable contracts small while preserving compatibility bundles.

## What Changes

- Define named capability families and map runtime modules, exports, docs, and tests to those families.
- Add feature gates for high-level families where dependency or surface reduction is meaningful.
- Preserve compatibility bundles for existing users who want the current broad surface.
- Update facade inventory with feature requirements and default/optional classification.
- Add compile checks for minimal runtime, selected-family runtime, all-family runtime, and root compatibility features.
- Add negative checks proving disabled families do not leak exports or dependencies into minimal surfaces.

## Impact

- **Files**: future Cargo feature metadata, module gates, exports, docs, facade inventory, compile checks, and Cairn artifacts.
- **Testing**: positive minimal/selected/all feature compile checks and negative disabled-family export/dependency checks.
- **Non-claims**: this does not remove accepted contracts, split crates immediately, or change runtime semantics inside an enabled family.
