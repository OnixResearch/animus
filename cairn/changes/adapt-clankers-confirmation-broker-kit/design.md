# Design: Adapt Clankers confirmation broker kit for Animus

## Context

Many Animus features need approvals: filesystem mutation, tool execution, checkpoint rollback, artifact delivery, remote session admission, memory persistence, and workflow automation. The confirmation logic should not be hidden in UI code or individual tools. Clankers' embedding boundary provides a useful pattern: reusable runtime emits typed confirmation requests, and hosts decide how to ask users or policy engines.

## Decisions

### 1. Confirmation requests are typed protected-action envelopes

**Choice:** A confirmation request includes request ID, protected action kind, resource/ability, safe summary, policy hash, session/run identity, required confirmation level, expiration, and redaction class.

**Rationale:** Confirmation should bind to the exact action being approved, not to unstructured prompt text.

### 2. Default deny is mandatory

**Choice:** Missing brokers, unavailable brokers, timeouts, cancellations, stale decisions, and mismatched decisions all return deny/fail-closed outcomes.

**Rationale:** Confirmation is a safety boundary. Absence of approval must never become approval.

### 3. Decisions are replay-resistant

**Choice:** Decisions reference the request ID, action identity, resource/ability, policy hash, and optional expiry. Runtime rejects decisions that do not match the pending request.

**Rationale:** Approvals for one action must not authorize another action after state changes.

### 4. Host adapters own interaction and persistence

**Choice:** The reusable broker contract does not display UI, send notifications, persist approval logs, or contact external approval services. Hosts inject those behaviors.

**Rationale:** Animus must work in embedded, headless, TUI, and service contexts without a required UI dependency.

### 5. Receipts are safe summaries

**Choice:** Confirmation receipts record request ID hash, action kind, resource/ability labels or hashes, decision status, broker class, policy hash, timing class, and BLAKE3 receipt hash. Raw prompts, credentials, full paths, provider payloads, and secret summaries are excluded.

**Rationale:** Approval evidence must be auditable without leaking the protected action payload.

## Risks / Trade-offs

- Too many confirmation prompts can block automation. Policy profiles should decide which actions require confirmation.
- Safe summaries can under-explain risk. Hosts may show richer local-only details, but receipts stay redacted.
- Expiry and stale-state checks add complexity. Keep matching rules pure and deterministic.
