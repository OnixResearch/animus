# Design: TUI conversation render model

## Context

The archived `add-ratatui-tool-host-surface` change established the boundary: `animus-tui` is the only crate that may depend on terminal packages, reusable crates stay terminal-free, and `ToolRenderModel` is a pure reducer over `ToolHostEvent` and `RuntimeEvent` values. That reducer, however, models tool lifecycle and progress, not the conversation transcript a pi-style harness shows the user.

`animus-runtime` emits `RuntimeEvent` values for turn lifecycle, model requests/responses, and tool dispatch. `animus-adapters` owns model/tool request and response DTOs, multimodal image payloads, and tool output bounds. A conversation render model should consume those existing downstream surfaces and derive visible transcript state, keeping the reducer pure and testable without a terminal.

## Decisions

### 1. The conversation model is a pure reducer over downstream events

**Choice:** Add a `ConversationRenderModel` in `animus-tui` that is advanced by applying `RuntimeEvent` and adapter DTO snapshots. Terminal I/O, polling, and redraw stay in the shell.

**Rationale:** The reducer can be unit-tested with recorded events and reused by a live Ratatui shell without changing the contracts it consumes.

### 2. Transcript entries are typed and bounded

**Choice:** Transcript entries are typed variants (user message, assistant text, thinking, tool-use, tool-result, image, error) carrying bounded model-visible content plus safe metadata (entry kind, sizes, truncation status, optional BLAKE3 content hash). Raw provider bodies, credentials, and unbounded tool output are never stored verbatim.

**Rationale:** Bounded entries keep the render model safe for display and replay while preserving identity evidence through hashes, matching the repo's receipt/boundary conventions.

### 3. The tool render model composes under the conversation model

**Choice:** The existing `ToolRenderModel` remains the tool-local view; the conversation model references active tool runs and composes their rendered state into transcript tool entries.

**Rationale:** The archived tool host surface stays valid and independently testable; the conversation layer adds ordering and transcript context on top.

### 4. Downstream-only: no reusable-crate contract changes

**Choice:** `animus-tui` consumes `RuntimeEvent`, `animus-adapters` DTOs, and `animus-runtime`/`animus-steel` decision surfaces as-is. If a needed field is absent, the TUI degrades gracefully rather than adding a field to a reusable crate.

**Rationale:** Keeps the TUI a downstream consumer and preserves the explicit crate barrier baseline.

### 5. Deterministic text snapshot for tests

**Choice:** `render_text_snapshot` reproduces the visible transcript as deterministic text from the model, independent of any terminal backend.

**Rationale:** Focused tests can assert transcript shape, ordering, truncation, and failure entries without a real terminal, mirroring the archived tool-host test rail.

## Risks / Trade-offs

- Conversations can grow unbounded; the model must keep a bounded visible window with safe metadata for evicted entries.
- Reusing `RuntimeEvent` shapes ties the reducer to runtime event evolution; the TUI must tolerate unknown event variants gracefully.
- Composing `ToolRenderModel` could duplicate state if boundaries are unclear; the conversation model owns transcript ordering, the tool model owns tool-local progress.
