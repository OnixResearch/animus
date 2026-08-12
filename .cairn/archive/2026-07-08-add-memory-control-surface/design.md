# Design: Add memory control surface

## Context

A memory system users cannot inspect or correct will become untrusted. A memory system tied to one backend will be hard to embed. The control surface should therefore operate over normalized memory DTOs, adapter status values, and safe receipts while hosts decide how to present it in a TUI, CLI, RPC API, or MCP bridge.

## Decisions

### 1. Commands are adapter-neutral DTOs

**Choice:** Define memory control requests for list, show, search, why, approve, reject, forget, update/supersede, and safe export summary over normalized identities and scopes.

**Rationale:** The same command should work for markdown, SQLite, vector, MCP, remote, or null backends when the injected adapter supports it.

### 2. Validation happens before mutation

**Choice:** Command validation checks scope, capability, admission policy, freshness, required evidence, and safe output posture before invoking store/update/delete adapters.

**Rationale:** A UI or remote caller should not be able to mutate memory just by naming a backend record.

### 3. Candidate review is explicit

**Choice:** Extracted memory candidates move through pending, approved, rejected, expired, superseded, or unavailable states before persistence.

**Rationale:** This preserves opt-in memory generation and prevents hidden auto-write behavior.

### 4. Forget and update return receipts

**Choice:** Deletion, tombstone, update, and supersession operations return adapter-neutral receipts with typed complete, partial, denied, stale, unavailable, or unsupported outcomes.

**Rationale:** Users need evidence that memory was actually removed or why it could not be removed.

### 5. `why` explains influence, not private bodies

**Choice:** Influence explanation uses prompt-injection ledger entries, match reasons, ranks, scope, confidence, policy decisions, and safe hashes without exposing redacted body content.

**Rationale:** Explainability should not bypass redaction or leak hidden memory.

## Risks / Trade-offs

- Some backends may not support update or hard delete. Model this as typed unsupported/partial outcomes rather than faking success.
- Showing too much metadata can leak source details. Keep receipts and explanations safe by default.
- Remote control surfaces need admission. Reuse existing remote admission patterns before any memory mutation.
