# Proposal: Dogfoodable host readiness roadmap

## Why

Animus has strong reusable contracts, adapter DTOs, focused rails, and crate boundaries, but the comparison with `../clankers` shows a clear operational gap: Clankers is a dogfooded agent product with a CLI, real provider stack, built-in tools, durable sessions, daemon/attach paths, plugin/MCP runtime, release harness, and user-facing examples, while Animus mostly stops at reusable contracts and host-owned seams.

That gap is partly intentional. `docs/clankers-product-surface-rejection.md` rejects copying Clankers product surfaces as default Animus behavior. The problem is that Animus does not yet have a maintained path for becoming dogfoodable through optional, host-owned adapters without overclaiming parity or weakening the reusable-core boundary.

This change creates that path as a reviewable roadmap package: define what “dogfoodable” means for Animus, identify the missing operational surfaces, sequence successor Cairns, and require evidence gates before any host shell claims readiness.

## What Changes

- Add a maintained Clankers-gap inventory that separates intentional non-claims from actionable dogfoodability blockers.
- Define a minimum non-default host shell target for first dogfood: headless/CLI first, fake-provider deterministic smoke before live providers, and no daemon/TUI/plugin default claims.
- Define a readiness matrix for provider execution, tool execution, session persistence, receipts, docs/examples, and release evidence.
- Require each successor product-surface Cairn to name host ownership, feature gating, adapter boundaries, safe receipts, and focused validation before claiming behavior.
- Add an evidence ladder for deterministic checks, fake-backed integration, optional live-provider smoke, optional TUI dogfood, and current-head evidence indexing.
- Update roadmap sequencing so product-readiness work lands only after reusable runtime, injected services, receipts, ledger, policy, and concrete adapter contracts are accepted.

## Impact

- **Files**: roadmap/spec Cairn artifacts, a future dogfoodability roadmap doc, possible roadmap-index updates, and focused validation fixtures for parity-claim hygiene.
- **Testing**: Cairn proposal/design/tasks gates, lifecycle validation, roadmap validation, and future focused checks that reject parity/readiness claims without evidence.
- **Non-claims**: this change does not implement live providers, OAuth, daemon/attach, Matrix/P2P, plugin runtimes, global session stores, a product TUI, or Clankers-compatible behavior. It defines the path and evidence required before those optional host-owned surfaces can be claimed.
