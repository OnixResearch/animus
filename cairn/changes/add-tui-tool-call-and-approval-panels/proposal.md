# Proposal: TUI tool call and approval panels

## Why

The archived Ratatui tool host surface defines UI-neutral `ToolHostEvent`/`ToolHostCommand` bridge DTOs and a pure `ToolRenderModel`, but a pi-style harness shows rich tool-call cards: collapsible bounded output, file-edit/diff views, progress, and interactive approval prompts. This change adds those panels to `animus-tui`, extending the existing tool host bridge and rendering hashed-edit-protocol DTOs as diffs, without letting tools draw to the terminal or bypass the finite/correlated approval contract.

## What Changes

- Add tool-call cards with collapsible, bounded output rendering from `ToolHostEvent` progress and output events.
- Add file-edit and diff views rendered from hashed-edit-protocol DTOs (BLAKE3 snapshots and tagged spans) and tool-gateway-delivery receipts, host-only.
- Add progress rendering for long-running tools using the existing tool host progress events.
- Add interactive approval prompts that extend the archived `ToolHostEvent`/`ToolHostCommand` bridge, keeping finite command choices and correlation identities.
- Compose on the conversation render model and the archived tool host surface; add no new reusable-crate contracts.

## Impact

- **Files**: new `animus-tui` tool-card, diff-view, progress, and approval-panel modules; tests using the existing tool host bridge and hashed-edit fixtures.
- **Testing**: positive tests for card rendering, diff rendering, progress, and approval flow; negative tests for denied approvals, mismatched correlation, oversized output, malformed edit DTOs, and direct-draw leakage.
- **Non-claims**: this change does not let tools draw to the terminal, does not bypass the finite/correlated approval contract, does not retain raw tool payloads, and does not add reusable-crate contracts.
