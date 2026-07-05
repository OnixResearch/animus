# Proposal: Add domain prelude facades for composable APIs

## Why

Animus exposes large flat public facades for runtime and adapter contracts. That keeps compatibility convenient, but it makes embedders work harder to discover the smallest composable surface for a task. Domain-oriented preludes can make composition explicit without removing the existing compatibility layer.

This change adds curated prelude modules for common composition domains so users can import only loop, tool, headless, provenance, worker, policy, or UI-neutral contracts they actually need.

## What Changes

- Add small documented prelude modules for runtime loop, tool contracts, headless protocol, provenance, worker-team delegation, policy/middleware, and status selection.
- Keep the existing flat exports as compatibility surfaces.
- Classify each prelude in the runtime facade inventory with owning crate, feature requirements, and side-effect class.
- Add compile examples or tests proving preludes compose without importing unrelated shell or terminal types.
- Add negative checks rejecting accidental shell, provider, filesystem, process, or terminal dependencies in reusable preludes.

## Impact

- **Files**: future crate root modules, prelude modules, facade inventory, docs, compile tests, and Cairn artifacts.
- **Testing**: positive import/compile tests for each prelude and negative dependency-boundary checks for shell/TUI leakage.
- **Non-claims**: this does not remove existing exports, rename public DTOs, or make preludes responsible for runtime behavior.
