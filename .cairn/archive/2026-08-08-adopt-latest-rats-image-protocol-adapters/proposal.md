## Why

Animus pins Rats revision `e9cb7865fe400de6220aea8bfc08ddf4f75da3d3`, which predates the upstream `rat-image` crate. The current TUI stores bounded image metadata and renders a text description, but it cannot transmit, place, or remove images through terminal graphics protocols. The implementation baseline observed upstream default-branch revision `3b498c3d7c248c335d3e7faf12f133a18c87d632`, which contains a bounded `rat-image` preparation and injected-writer seam for Kitty, iTerm2, and Sixel. Direct adoption is not sufficient because Animus also needs explicit capability observations, named bounds, typed failures, deterministic planning, safe cleanup, and metadata-only evidence.

## What Changes

- Select the latest canonical Rats default-branch revision at implementation baseline time, review its dependency and license changes, and pin that exact immutable revision across Cargo and Nix inputs. r[agent_harness_runtime.terminal_image_protocols.rats_cohort]
- Add a non-default terminal-image feature under `animus-tui` that uses upstream `rat-image` without vendoring or copying its protocol implementation. r[agent_harness_runtime.terminal_image_protocols.boundary]
- Add a pure terminal-image planner over caller-supplied capabilities, image metadata, layout, limits, and lifecycle state. r[agent_harness_runtime.terminal_image_protocols.planning]
- Add bounded static-image lifecycle support for Kitty, iTerm2, and Sixel, with text fallback and explicit unsupported outcomes. r[agent_harness_runtime.terminal_image_protocols.protocols]
- Add host-owned byte resolution and terminal writing, fail-closed cleanup, metadata-only BLAKE3 receipts, and positive and negative conformance rails. r[agent_harness_runtime.terminal_image_protocols.evidence] r[agent_harness_runtime.terminal_image_protocols.verification]

## Impact

- **Files**: root Cargo and Nix inputs, generated lockfiles through their native tools, `crates/animus-tui`, root feature wiring, Nickel configuration, focused fixtures and checks, README links, and terminal-image documentation.
- **Dependencies**: the optional TUI surface gains the exact reviewed upstream `rat-image` package; default, reusable, no-std, Wasm, runtime, Steel, and unrelated host profiles remain unchanged.
- **Testing**: focused TUI tests, pure planner tests, protocol byte fixtures, negative malformed and oversized cases, dependency isolation, generated-configuration checks, Cairn validation and gates, and explicit live-terminal non-claims.

## Non-Goals

- Video decoding, playback scheduling, audio, provider video input, or media persistence.
- A claim of every Kitty transport, compression, composition, or animation extension before its conformance row passes.
- Ambient terminal discovery or direct terminal writes from reusable Animus crates.
- Vendoring, forking, or duplicating Rats protocol code inside Animus.
