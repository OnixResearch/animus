# Proposal: Add packaged TUI dogfood smoke

## Why

Animus has accepted headless dogfood and TUI contracts. It does not have packaged evidence that composes these surfaces through a real Ratatui render path.

A deterministic `TestBackend` smoke can prove this bounded composition without raw terminal mode, provider credentials, network access, or production claims.

## What Changes

- Add a non-default `animus-tui-dogfood-smoke` binary behind an explicit `animus-tui` feature.
- Reuse the accepted packaged headless dogfood path before TUI projection.
- Render safe headless metadata through a fixed Ratatui `TestBackend` viewport.
- Emit metadata-only evidence with BLAKE3 identities and explicit non-claims.
- Add positive and negative tests for terminal loss, denied capability, oversized receipt input, cleanup failure, and unsafe output leaks.
- Add a focused rail, Nix check, roadmap entry, claim-hygiene fixture, and operator documentation.

## Impact

- **Files**: `animus-tui`, focused rail scripts, flake checks, dogfood docs, roadmap data, fixtures, and Cairn artifacts.
- **Testing**: focused TUI tests, packaged CLI fixtures, readiness checks, Cairn gates, Clippy, and the focused Nix check.
- **Non-claims**: no live terminal, raw mode, input polling, daemon attach, live provider, plugin runtime, production, release, or Clankers-parity claim.
