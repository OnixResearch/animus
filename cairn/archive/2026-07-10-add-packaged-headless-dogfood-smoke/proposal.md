# Proposal: Add packaged headless dogfood smoke

## Why

Animus now has a fake-backed dogfood core, but the next readiness rung requires a packaged non-default shell that proves the fake-backed path can be invoked as a host-facing CLI without promoting live providers, tools, persistence, daemon, TUI, plugin runtime, production readiness, or Clankers parity.

A small gated smoke binary gives maintainers and embedders a runnable command that parses prompt input at the shell boundary, calls deterministic fake services, and emits safe receipt metadata only.

## What Changes

- Add a non-default `animus-dogfood-smoke` binary behind the `animus-std/dogfood-smoke` feature.
- Parse bounded CLI prompt/config arguments into typed dogfood DTOs, then call the existing fake-backed runtime core.
- Render safe line-oriented smoke evidence with packaged/core evidence rungs, event counts, BLAKE3 identities, ordered event labels, and omitted-surface non-claims.
- Add a focused rail and Nix check for positive packaged smoke and negative missing prompt/service/capability/malformed/unsafe-label cases.
- Update docs and claim-hygiene fixtures to identify the packaged headless smoke rung without overclaiming omitted surfaces.

## Impact

- **Files**: `animus-std` feature/bin/module, focused rail script, flake check wiring, dogfood docs, readiness fixtures, and this Cairn change package.
- **Testing**: std-shell unit tests, CLI positive/negative rail, dogfood readiness rail, Cairn validation/gates, and focused Nix check.
- **Non-claims**: this does not add live-provider execution, credential discovery, network dispatch, default product tools, persistence, daemon/attach, TUI dogfood, plugin runtime behavior, production readiness, release tagging, or Clankers parity.
