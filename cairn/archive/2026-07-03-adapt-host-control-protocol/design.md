# Design: Adapt host control protocol for Animus

## Context

Herdr's socket API is a useful reference because it cleanly separates a control surface from terminal rendering: list workspaces, split panes, read output, send input, wait for agent state, export layout, and subscribe to events. Animus should adapt those semantics as host-neutral DTOs that can be carried by CLI, SDK, Preserves, or another transport.

## Decisions

### 1. Host topology is modeled explicitly

**Choice:** Represent workspaces, tabs, panes, and agents as host-owned resources with typed live selectors, labels, cwd/provenance summaries, status, and revision metadata.

**Rationale:** Orchestration needs a stable vocabulary for host surfaces without coupling reusable runtime code to Ratatui, PTYs, or a specific socket server.

### 2. Selectors are not durable unless marked durable

**Choice:** Public pane, tab, workspace, and agent selectors are live-session selectors. Hosts may expose durable IDs separately, but callers must not infer durability from compact UI IDs.

**Rationale:** Herdr's compact IDs can change when topology changes. Animus should make selector lifetime explicit.

### 3. Reads are bounded views

**Choice:** Host reads return bounded output views with source class, line/byte counts, truncation status, redaction state, and optional BLAKE3 content hash.

**Rationale:** Agents need enough output to coordinate, while receipts and runtime state must avoid unbounded logs and raw transcript persistence.

### 4. Events and waits share semantic state

**Choice:** One-shot waits are a thin form over event subscriptions and semantic states. Waits observe selected status facts, output matches, cancellation, and timeout outcomes.

**Rationale:** Headless scripts and TUI shells should not implement divergent waiting semantics.

### 5. Transport is an adapter detail

**Choice:** The reusable contract defines command, response, error, and event DTOs. Hosts own local sockets, named pipes, CLI wrappers, authentication, process input, PTYs, and terminal rendering.

**Rationale:** Animus must remain embeddable and testable without standing up a terminal multiplexer.

## Risks / Trade-offs

- A broad control protocol can become a product API too early. Start with stable host boundary concepts and reject private UI state.
- Input send operations are inherently effectful. Core validation must stop at typed command admission; host policy must decide whether to execute.
- Bounded output reads can omit relevant context. Responses should expose truncation and safe references instead of silently hiding limits.
