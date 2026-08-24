## Context

The live terminal core already owns typed command descriptors and pure control reduction. The selected product requires explicit kernel clear and bounded state inspection.

Using the generic `session` class would hide distinct authorization and audit meaning. Product workers need exact classes before they decode private payloads.

## Decisions

### Decision: Add dedicated command variants

**Choice:** Add `ClientCommand::KernelClear` and `ClientCommand::StateInspection`. Each variant contains only a BLAKE3 correlation identity.

**Rationale:** Exact classes let a worker admit, deny, audit, and route each operation without inferring meaning from a generic session payload.

### Decision: Keep construction in the pure core

**Choice:** Add matching `ControlIntent` variants and construct descriptors through `command_from_control`.

**Rationale:** The terminal remains a deterministic descriptor producer. It gains no kernel, journal, provider, tool, session, or persistence effect.

### Decision: Preserve the private routing boundary

**Choice:** Serialize the typed descriptor through the existing daemon or direct connection. The supervisor continues to route bytes unchanged.

**Rationale:** The worker remains the only owner of authorization and execution.

### Decision: Fail closed on malformed descriptors

**Choice:** Reuse exact serde tags, deny unknown variants through enum decoding, and require a valid BLAKE3 correlation identity before routing.

**Rationale:** Malformed or forged commands must not reach the worker-owned effect boundary.

### Decision: Return inspection through the event stream

**Choice:** Add a dedicated `WorkerEvent::StateInspection` variant. It carries only request and observation identities, worker and queue generations, restore status, BLAKE3 heads, and `u32` counts.

**Rationale:** The existing command result is identity-only. The event stream already supplies ordered, bounded, snapshot-first delivery without granting terminal effect authority.

### Decision: Keep state events exact across the wire

**Choice:** Add a tagged daemon event variant and reject unknown outer or nested fields. The supervisor and terminal bind each result to its request, payload, generation, restore status, root, and ordered sequence.

**Rationale:** A typed variant prevents private prompt, source, result, credential, or path fields from hiding inside a generic event class.

### Decision: Render but do not receipt private state

**Choice:** Store the latest validated inspection in terminal view state and render its safe fields. Terminal completion receipts remain unchanged.

**Rationale:** Operators can inspect bounded state without expanding durable evidence or exposing session content.

### Decision: Keep private events in the ordered worker stream

**Choice:** Add a bounded opaque `WorkerPrivateEvent` to `WorkerEvent`. Bind its root, sequence, class, size, and payload bytes with BLAKE3.

**Rationale:** One stream preserves snapshot-first order and attachment cursors. A separate private stream can drift from the safe event sequence.

### Decision: Return the completed runtime-event snapshot to the worker

**Choice:** `LiveSteelAgentRunOutput` returns the exact ordered `RuntimeEvent` snapshot through a private accessor. The receipt keeps only its bounded count.

**Rationale:** The product worker needs the authoritative conversation, tool, and status events before it can encode them for private delivery. Reconstructing events from final text or receipt metadata would lose meaning.

### Decision: Reuse the versioned Preserves runtime contract

**Choice:** The worker encodes `RuntimeEvent` values with `animus-preserves`. The terminal admits only the `runtime_event` class and decodes after delivery.

**Rationale:** The existing contract already revalidates provider, tool, stream, session, and runtime event types. A second wire DTO would duplicate these rules.

### Decision: Keep the supervisor opaque

**Choice:** The supervisor validates bounds, order, root, class, and payload identity. It does not decode or retain private bytes. Debug output shows only byte counts.

**Rationale:** The daemon owns transport and attachment policy, not transcript, tool-output, provider, or session meaning.

### Decision: Keep live resources inside one persistent session owner

**Choice:** `LiveSteelAgentComposition::open_session` moves the provider, snapshot store, coding-session persistence, Steel tool, kernel persistence, and validated configuration into one owning session.

**Rationale:** An owning value avoids self-references and prevents the product worker from reconstructing provider, tool, or durable-ordering policy.

### Decision: Separate command reduction from effect progress

**Choice:** The persistent session begins exact prompt, follow-up, steering, queue-CAS, and cancellation commands without driving. `drive_one` executes at most one existing `CodingSessionHost` effect.

**Rationale:** A worker can interleave private commands between effects while all core policy, authority, commit, and cancellation checks remain unchanged.

### Decision: Bound private incremental snapshots

**Choice:** Event reads require a monotonic cursor and a non-zero maximum no larger than `MAX_LIVE_SESSION_EVENT_BATCH`. The returned batch has no `Debug` implementation.

**Rationale:** Workers can relay exact ordered events without adding content to receipts, logs, or safe metadata.

### Decision: Delegate executor controls through the persistent owner

**Choice:** Add a narrow executor-control port for approval and child status or cancellation. `PersistentLiveHost` delegates through its existing `CodingSessionHost` tool owner.

**Rationale:** The terminal and product shell must not acquire provider, tool, child, or session authority.

### Decision: Keep lifecycle commands out of worker-private routing

**Choice:** Refresh, detach, and stop use explicit connection methods. Daemon requests carry these operations as distinct protocol variants.

**Rationale:** Attachment and worker lifecycle belong to the selected connection and supervisor, not the private worker command decoder.

### Decision: Bound the daemon request loop

**Choice:** One server loop accepts an explicit finite client and request count. It authenticates each peer, applies frame deadlines, and detaches on disconnect.

**Rationale:** A bounded shell composes existing transport and supervisor services without adding discovery, fallback, or session effects.

## Rejected alternatives

- A separate private-event port was rejected because two event cursors can diverge.
- Direct terminal access to the session store was rejected because it adds persistence authority.
- Rebuilding a `CodingSessionHost` for every private command was rejected because pending effects and queue state would lose one owner.
- Decoding lifecycle commands inside the supervisor was rejected because the supervisor must keep private command payloads opaque.
- Letting the terminal call child or approval services directly was rejected because it would duplicate worker authority.
- A product-owned copy of live provider and tool adapters was rejected because it duplicates Animus authority and durable ordering.
- Structured data in safe event classes was rejected because it violates safe-label and receipt boundaries.

## Risks / Trade-offs

- New public enum variants require exhaustive consumers to update.
- The product worker must still authorize inspection and construct truthful metadata.
- State inspection remains a request. It does not permit private state in terminal receipts.
- Local authenticated transport carries private bytes in memory and on the Unix socket.
- The relay does not claim remote security or end-to-end encryption.
- One provider or tool effect remains synchronous. External commands are admitted between effects, not inside an adapter call.
