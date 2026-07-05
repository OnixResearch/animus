# Proposal: TUI streaming markdown view

## Why

The conversation render model produces a typed transcript, but a pi-style agent TUI must render streaming model output as it arrives: text and thinking deltas, content-block start/stop lifecycle, markdown formatting, and multimodal image payloads. This change adds the streaming markdown view that applies incremental deltas to the conversation model and renders accumulated blocks through the host-only Rats markdown widgets, without leaking terminal or Rats types into reusable contracts.

## What Changes

- Add a streaming view in `animus-tui` that applies model-streaming deltas (text, thinking) and content-block lifecycle events to the conversation render model incrementally.
- Render accumulated markdown blocks through host-only `rat-markdown` widgets, bounded by the conversation model's visible window.
- Render multimodal image payloads from adapter DTOs as bounded display images, never embedding raw provider bytes in render state.
- Add a bounded scrollback window with safe eviction metadata so streaming does not grow unbounded.
- Compose on the conversation render model from `add-tui-conversation-render-model`; add no new reusable-crate contracts.

## Impact

- **Files**: new `animus-tui` streaming/markdown/image view modules, scrollback windowing, host-only Rats markdown wiring, and snapshot/streaming tests.
- **Testing**: positive tests for delta accumulation, block lifecycle, markdown snapshot, and image rendering; negative tests for oversized deltas, malformed image payloads, unknown block kinds, and Rats-type leakage into reusable crates.
- **Non-claims**: this change does not call providers, does not own prompt assembly, does not add a render loop or terminal backend, and does not bypass output bounds.
