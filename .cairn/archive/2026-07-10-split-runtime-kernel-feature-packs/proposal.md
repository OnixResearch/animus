# Proposal: Split the runtime kernel from feature packs

## Why

`animus-runtime` exports a reusable loop runtime, but it also contains many optional domains: headless protocol, provenance, worker teams, hooks, batch, compaction, dataset export, research, quality records, scaffold policy, orchestration bridges, and compatibility surfaces. The default `compat` bundle is convenient but makes the runtime feel like a large kit instead of a small harness kernel with snap-on blocks.

A kernel/feature-pack split lets embedders depend on the minimal loop and event machinery first, then opt into domain packs intentionally.

## What Changes

- Define a minimal runtime kernel boundary for loop turns, decisions, events, reports, capability registry, model/tool dispatch planning, and injected adapter orchestration.
- Move or gate optional runtime domains as feature packs with documented dependencies and component catalog entries.
- Preserve `animus-runtime` flat exports as a compatibility facade while adding focused direct imports for new code.
- Add checks that the kernel does not depend on Steel, headless protocol, provenance, worker teams, batch, research/eval, TUI, std shell, provider clients, storage, or host authority.

## Impact

- **Surfaces**: `animus-runtime` crate features, possible new `animus-runtime-kernel` crate or internal kernel module, runtime preludes, component catalog, facade inventory, composition conformance rail.
- **Testing**: kernel-only cargo check, feature-pack cargo checks, facade parity tests, composition conformance cases, and Cairn gates.
- **Non-claims**: this does not remove existing flat exports, change runtime behavior silently, add providers/tools/storage, or require Steel in the kernel.
