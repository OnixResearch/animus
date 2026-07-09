# Design: Adapt Goose message queue and interrupt controls

Goose's queue and interrupt ergonomics are useful because they let users steer long-running work without losing task sequencing. Animus should model the same intent as reusable state transitions and host-owned control effects.

### 1. Queues are explicit runtime state

**Choice:** Define `QueuedMessageDraft`, `QueuedMessage`, `QueueOperation`, `QueueState`, `QueueAck`, and `QueueReceipt`-style DTOs with source identity, sequence, active-turn reference, visibility class, safety class, budget class, and policy identity.

**Rationale:** Continuation policy already handles queued prompts, but queue editing and UX controls need first-class state transitions that can be tested without a host UI.

### 2. Queue mutations use a reducer

**Choice:** Add a pure reducer for append, edit, reorder, delete, clear, dispatch-next, dispatch-ack, and cancel-pending operations. The reducer rejects stale active-turn IDs, duplicate queue IDs, unauthorized source labels, max-queue overflow, and unsafe queued material.

**Rationale:** Queue behavior must be deterministic across TUI, headless, and SDK hosts.

### 3. Interrupts are control events, not prompts by default

**Choice:** Represent interrupts as `InterruptSignal` and `ControlEvent` data. Policy owns deterministic keyword classes and explicit UI send modes. Interrupts can request pause, cancel, redirect, or immediate prompt dispatch only when a host supplies matching active-work and cancellation capabilities.

**Rationale:** A message like `stop` should not be silently appended to model-visible context when the user's intent is control.

### 4. Receipts prove safe control handling

**Choice:** Queue and interrupt receipts record operation kind, actor/source label, active-work hash, queue item hash, decision class, cancellation handle class, budget class, policy hash, and BLAKE3 receipt hash.

**Rationale:** Durable evidence should prove the control action and denial class without storing raw queued messages or command output.

### Risks

- Keyword interruption can misclassify normal text. Mitigate with policy-owned confidence classes and explicit host send mode overriding heuristic classification.
- Queues can hide dangerous delayed instructions. Mitigate with per-item safety validation at enqueue and again at dispatch.
- Cancellation may be unavailable for some adapters. Mitigate with typed missing-cancellation-handle outcomes.
