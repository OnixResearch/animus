# Proposal: Add current-head release evidence index generator

## Why

Animus has a fixture-backed release evidence index validator, but maintainers still need a deterministic shell-owned way to render an index for the actual repository head. Without that generator, current-head evidence remains hand-authored and easy to stale, dirty, or overclaim.

The generator should collect VCS head metadata, accept explicit check statuses from already-run rails, emit safe line-oriented evidence, and validate dirty/stale/missing-mode behavior without running hidden test suites or implying omitted optional host surfaces.

## What Changes

- Add `scripts/cairn/generate-current-head-release-evidence-index` to render evidence index lines from actual or supplied head metadata.
- Add `scripts/cairn/check-current-head-release-evidence-index` to validate generated positive, dirty, missing-mode, and stale synthetic indexes plus actual-head draft generation.
- Extend the release evidence schema/checker with the packaged headless dogfood smoke as a recognized pure selected check.
- Wire a Nix check for the current-head generator rail.
- Update release evidence docs, focused rails, README verification commands, and lifecycle artifacts.

## Impact

- **Files**: release evidence scripts/schema/fixtures/docs, flake check wiring, roadmap classification, and Cairn lifecycle artifacts.
- **Testing**: existing release-evidence fixture rail, generated current-head rail, Nix checks, Cairn validation/gates.
- **Non-claims**: this does not run full workspace tests, Clippy, flake checks, live providers, TUI/daemon dogfood, plugin runtime, VM checks, production readiness, release tagging, or any hidden harness. It only renders and validates supplied evidence metadata.
