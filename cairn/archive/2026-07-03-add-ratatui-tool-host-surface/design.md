# Design: Add a Ratatui-compatible tool host surface for Animus

## Context

The current Animus workspace already has the right crate split for a terminal shell:

- `animus-core` is pure `no_std` state.
- `animus-adapters` exposes validated model and tool request/response contracts.
- `animus-steel` can choose `CallTool` loop decisions.
- `animus-std` owns process-facing runtime orchestration.

A Ratatui interface should therefore be an embedding shell over existing contracts, not a dependency that leaks into tools or core DTOs. Tools need a way to report progress, request approval, and receive cancellation or approval decisions while staying usable by headless, RPC, and test hosts.

## Decisions

### 1. The TUI is a host implementation, not a tool dependency

**Choice:** Ratatui, Rats, and terminal backends live only in an optional host crate or binary such as `animus-tui`. Reusable crates expose UI-neutral DTOs and traits.

**Rationale:** A tool that depends directly on Ratatui or Rats cannot be reused by headless protocols, SDK embedders, tests, or future non-terminal shells.

### 2. Rats is a host-only widget dependency

**Choice:** The optional TUI crate depends on the Rats workspace through an SSH-authenticated Cargo Git dependency, pinned by revision, and uses its Ratatui widget package only at the host-rendering boundary.

**Rationale:** Rats provides reusable terminal widget primitives while preserving Animus' boundary rule: reusable runtime crates do not learn about terminal widgets.

### 3. Tools communicate through a typed host bridge

**Choice:** Add a tool host bridge around validated `ToolRequest` and `ToolResponse` values. The bridge carries typed events such as started, progress, output chunk, approval request, cancellation, result, and failure, plus typed commands such as approve, deny, continue, and cancel.

**Rationale:** Structured events let Ratatui render rich state while preserving deterministic dispatch, bounded output, safe receipts, and mode-neutral runtime events.

### 4. Rendering state is reduced from events

**Choice:** The Ratatui shell maintains a render model by applying a pure reducer to tool/runtime events. Crossterm setup, terminal cleanup, polling, channels, and redraw timing stay in the imperative shell.

**Rationale:** The render model can be tested without a terminal, and terminal failures do not contaminate runtime logic.

### 5. Interactive requests are finite and cancellable

**Choice:** Tool prompts for human input must expose finite command choices and a correlation identity. The host returns a typed decision or cancellation; tools must not read stdin, block on terminal events, or invent unbounded free-form prompts through this bridge.

**Rationale:** Approval, denial, and cancellation semantics need to be replayable, auditable, and safe for noninteractive hosts.

### 6. Runtime integration remains adapter-neutral

**Choice:** `animus-std` or a host-owned runtime runner may wrap ordinary `ToolAdapter` implementations with a hosted execution path. A Ratatui host may subscribe to the same runtime events used by headless or receipt surfaces.

**Rationale:** The existing simple tool adapter remains valid, while interactive tools can opt into richer host services without changing Steel decisions or core state.

## Risks / Trade-offs

- A rich event vocabulary can become too UI-specific. Start with lifecycle, progress, bounded output, approval, cancellation, result, and failure events only.
- Terminal rendering can hide unsafe raw payloads if it bypasses output bounds. The TUI must render bounded model-visible output and safe metadata by default.
- A Git dependency can drift if left floating. Pin the Rats dependency by revision and use SSH auth for access.
- Async/channel choices can leak into contracts. Keep reusable contracts synchronous and data-oriented; host shells choose their own transport.
- Adding a TUI could appear to contradict product-surface rejection. This change keeps it host-owned, optional, and dependency-isolated rather than default reusable behavior.
