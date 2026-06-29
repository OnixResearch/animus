# Design: Adapt agent status authority for Animus

## Context

Herdr's transferable design is its status authority model. Agents may be detected heuristically from a terminal, or they may report lifecycle state through an integration. When a lifecycle integration is authoritative, Herdr avoids mixing it with screen fallback. Animus should adapt that precedence and explainability while keeping all terminal/process inspection outside reusable crates.

## Decisions

### 1. Semantic state is a typed runtime fact

**Choice:** Represent agent status as a typed DTO with semantic state, source identifier, agent label, optional message, optional custom visual status, optional session identity reference, sequence, and optional expiration.

**Rationale:** TUI, headless waits, notifications, receipts, and orchestration need one shared state class rather than host-specific strings.

### 2. Lifecycle authority suppresses heuristic fallback

**Choice:** A trusted lifecycle authority owns semantic state for its target while its report is current. Heuristic host observations are recorded as evidence or fallback candidates but do not override the authoritative lifecycle source.

**Rationale:** Competing state sources make waits and rollups nondeterministic.

### 3. Presentation is separate from lifecycle

**Choice:** Titles, display agent names, compact activity text, and custom labels are presentation metadata only. They cannot change semantic state or satisfy waits.

**Rationale:** Operators need useful labels without accidentally granting lifecycle authority to cosmetic hooks.

### 4. Explain reports are redacted and bounded

**Choice:** Status decisions produce optional explanation records naming the selected authority, rejected candidates, freshness decisions, and safe evidence summaries.

**Rationale:** Debugging requires visibility into why a pane or agent is blocked, working, or idle, but raw terminal output and prompts are unsafe durable evidence.

### 5. Detection remains host-owned

**Choice:** Reusable runtime crates accept typed observations from adapters. They do not inspect `/proc`, read terminals, parse screen buffers, or load screen-detection manifests.

**Rationale:** Animus crate boundaries require a functional core over explicit inputs and a thin host shell for I/O and ambient state.

## Risks / Trade-offs

- Too much status metadata can leak prompt or terminal content. Tests should reject raw evidence by default.
- Strict authority precedence can hide a useful fallback when an integration stalls. TTL and sequence handling must make stale reports explicit.
- Hosts may disagree about heuristic labels. The reusable contract should validate observations, not standardize every detector.
