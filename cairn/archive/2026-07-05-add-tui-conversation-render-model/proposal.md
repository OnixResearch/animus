# Proposal: TUI conversation render model

## Why

Animus already has a UI-neutral tool host bridge and a pure `ToolRenderModel` reducer from the archived Ratatui tool host surface, but that reducer is tool-centric. A pi/Claude-Code-style agent TUI needs a conversation-level render model: the visible transcript of user messages, assistant content blocks, thinking, tool calls, tool results, and errors, derived purely from `RuntimeEvent` and adapter DTOs. This change adds that functional core so the rest of the TUI (streaming view, panels, app shell) can render a live conversation without terminal I/O in the reducer and without adding any contract to reusable crates.

## What Changes

- Add a pure conversation render reducer in `animus-tui` that consumes `RuntimeEvent` values, model-streaming events, and `animus-adapters` model/tool DTOs to derive a visible transcript model.
- Define typed transcript entries for user messages, assistant content blocks (text, thinking, tool-use, tool-result, image, error) with bounded model-visible content and safe metadata (sizes, truncation status, BLAKE3 hashes where identity is needed).
- Keep the deterministic `render_text_snapshot` test-backend path so the reducer is testable without a terminal; live Ratatui consumes the same model.
- Preserve the existing `ToolRenderModel` as the tool-local view and compose it under the conversation model.
- Add no new types to `animus-core`, `animus-adapters`, `animus-steel`, `animus-runtime`, or `animus-std`; `animus-tui` is a downstream consumer only.

## Impact

- **Files**: new `animus-tui` view/reducer modules for the conversation transcript, transcript entry DTOs bounded within the TUI crate, reducer tests, and text-snapshot tests.
- **Testing**: positive tests for transcript assembly from recorded runtime/adapter events; negative tests for oversized/bounded output, malformed events, unknown block kinds, and dependency leakage into reusable crates.
- **Non-claims**: this change does not add terminal I/O to the reducer, does not render markdown or stream deltas (later cairn), does not call providers or tools, and does not introduce reusable-crate contracts.
