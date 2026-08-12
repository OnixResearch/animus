# Proposal: Introduce a Steel-backed agent runtime loop

## Why

Animus now has crate barriers for core state, adapter contracts, Preserves boundary payloads, and Steel Scheme turn decisions, but it still lacks the runtime crate that actually drives an agent harness loop. Without that loop, embedders can call individual pieces but cannot reuse a typed orchestration surface that dispatches Steel decisions to swappable model and tool adapters.

## What Changes

- Add an `animus-runtime` crate that owns the agent turn loop while keeping provider, filesystem, process, network, and UI concerns outside the runtime core.
- Make `animus-steel` the default scriptable loop fabric for turn decisions, with runtime code accepting an injectable fabric interface so other fabrics can be swapped later.
- Dispatch validated `LoopDecision` values to `ModelAdapter` and `ToolAdapter` implementations and return typed final reports.
- Add typed runtime turn events and reports that can later be encoded through `animus-preserves`.
- Add a Preserves schema for adapter/runtime boundary messages once the runtime event model stabilizes.
- Keep OpenAI, filesystem, process, and Syndicate integrations as follow-on adapter crates rather than direct runtime dependencies.

## Impact

- **Files**: new `crates/animus-runtime/`, root feature/dependency wiring, README architecture docs, safety contract docs, flake checks, optional Preserves schema files.
- **Testing**: positive and negative runtime-loop tests, model/tool adapter fakes, Steel decision failure tests, max-turn/stop behavior tests, Preserves schema round-trip tests when schema is added, clippy, Octet/Tigerstyle, pre-commit, and flake checks.
- **Non-claims**: this change does not add real model providers, process/filesystem tools, UI, networking, or a Syndicate dataspace implementation. Those remain adapter-crate follow-ups.
