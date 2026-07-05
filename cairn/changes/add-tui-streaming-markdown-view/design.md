# Design: TUI streaming markdown view

## Context

`add-tui-conversation-render-model` provides a pure transcript reducer and bounded entries. A live agent harness streams assistant output token-by-token: text deltas, thinking deltas, and content-block start/stop boundaries, plus multimodal image payloads. Rendering that stream richly requires markdown widgets, but those widgets are terminal packages that must stay host-only.

## Decisions

### 1. Streaming deltas advance the conversation reducer incrementally

**Choice:** The streaming view applies text/thinking deltas and content-block lifecycle events to the conversation render model one event at a time, producing incremental visible-state updates.

**Rationale:** Keeping the reducer the single source of truth means the live shell and the test backend observe the same state transitions.

### 2. Markdown rendering is host-only

**Choice:** `rat-markdown` rendering lives only in `animus-tui` and consumes the conversation model's accumulated text blocks. Reusable crates never import Rats or Ratatui markdown types.

**Rationale:** Preserves the archived dependency boundary and keeps the render model testable without a terminal.

### 3. Image payloads are bounded display images

**Choice:** Multimodal image payloads from adapter DTOs are rendered as bounded display images with safe metadata; raw provider bytes are not retained in render state beyond the display window.

**Rationale:** Bounds memory and avoids leaking unbounded multimodal content into render state.

### 4. Bounded scrollback with eviction metadata

**Choice:** The visible window keeps a bounded number of entries; evicted entries leave safe metadata (counts, hashes) rather than full content.

**Rationale:** Streaming over long sessions must not grow render state unbounded; the snapshot path must still report evicted-entry summaries.

### 5. Downstream-only composition

**Choice:** The streaming view composes on the conversation render model and consumes model-streaming and multimodal-image-payload DTOs as-is. No new field is added to a reusable crate.

**Rationale:** Keeps the TUI strictly downstream.

## Risks / Trade-offs

- Markdown rendering can hide unsafe payloads if it bypasses output bounds; the view must render only bounded model-visible content.
- Streaming deltas can race with block lifecycle; the reducer must tolerate out-of-order or orphaned deltas gracefully.
- Image decoding belongs in the shell, not the reducer; the reducer stores bounded display descriptors only.
