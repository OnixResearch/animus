## Context

Animus already has durable ledger replay, ACP ordered updates, conversation cursors, active-turn recovery, model streaming, and safe receipts. These surfaces use different local DTOs and do not yet share one event replay contract for clients that disconnect and reconnect.

The selected external references are Sandbox Agent session offsets and agentOS durable semantic event classification. Their APIs, storage choices, automatic reconnection behavior, and remote transport are not compatibility targets.

## Decisions

### Decision: One ledger and many projections

**Choice:** Derive every replayable session event from the existing admitted ledger and runtime event order. Do not create a second session event database or transcript.

**Rationale:** A second ledger can disagree with tool correlation, active leaf selection, repair, redaction, and model context.

### Decision: Classify durability before projection

**Choice:** Mark each event class as durable semantic, live-only delta, or metadata-only. Only durable semantic and admitted metadata events can appear in reconnect replay.

**Rationale:** Replaying partial model deltas can duplicate text or invent a completed semantic message.

### Decision: Fence cursors by generation and policy

**Choice:** A cursor binds session, branch or active leaf, ledger generation, stream identity, last sequence, policy identity, and redaction profile. Drift produces a bounded rebuild or denial, never silent continuation.

**Rationale:** A numeric offset alone cannot identify a changed branch, repaired ledger, or policy.

### Decision: Keep transport recovery host-owned

**Choice:** The core returns replay plans and bounded outcomes. HTTP, SSE, WebSocket, stdio, timers, retries, persistence, and client authentication remain host or transport adapter effects.

**Rationale:** Reusable crates must remain transport-neutral and authority-free.

## Risks / Trade-offs

- Durable classification can drift from provider or ACP semantics. Exact conversion fixtures must fail closed.
- Sequence gaps can expose missing data without proving why it is missing.
- Large replay windows can exhaust memory. Policy must name event and byte limits.
- The selected upstream references must be pinned to exact reviewed revisions before implementation.
