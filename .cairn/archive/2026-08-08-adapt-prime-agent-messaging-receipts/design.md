## Context

Prime Agent routes direct messages between active sessions and retained subagents through its daemon. Senders pick `auto`, `steer`, or `follow_up` delivery. Receipts report `delivered` when the message reached an idle target's context and `queued` when it was accepted for later delivery. The daemon derives sender identity, scopes broadcast to the family roster, and enforces message-size, rate, and pending-queue limits.

Animus owns adjacent pieces. `goose-message-queue-interrupts` provides a pure queued-message reducer and interrupt classifier. The Iroh and Syndicate seams provide host-owned communication over caller-owned transports. The orchestration adapter family owns board, task, and handoff contracts. What is missing is the delivery semantics layer: modes, receipts, roster-derived identity, and pre-routing limits as one coherent contract.

Completion requires pure routing decisions with typed modes and receipts, host-derived identity, and fail-closed limits. Model-claimed identity, ambient transport use, or receipts containing message bodies is false completion.

## Decisions

### Decision: delivery modes are pure routing decisions

**Choice:** A focused contract module decides routing from a typed message, the target's supplied state (busy or idle), and policy. `auto` yields steer when busy and deliver when idle. `steer` yields interrupt-class injection. `follow_up` yields enqueue-after-current-turn. The module never performs delivery.

**Rationale:** Mode semantics are deterministic over supplied state. Pure decisions let every transport host share one semantics and one test corpus.

### Decision: receipts distinguish delivered from queued

**Choice:** The receipt type has exactly two success variants. `delivered` requires host attestation that the message entered an idle target's context. `queued` requires attestation of accepted pending storage. No receipt variant claims reading, understanding, or action.

**Rationale:** The two-variant split is the prime-agent insight that keeps senders honest about what happened. More variants invite overclaim; fewer hide the difference.

### Decision: identity is host-derived, membership is roster-bound

**Choice:** The sender field comes from the host session registry, never from message content. Broadcast resolves against the admitted family roster only. Messages naming targets outside the roster fail before routing.

**Rationale:** Spoofed identity and cross-family broadcast are the two authority failures this layer can actually prevent. Both become unrepresentable or fail-closed.

### Decision: limits enforce before routing

**Choice:** Size, rate, and pending-queue checks run as pure functions over supplied counters before any routing decision. Violations return typed limit receipts and never reach a target.

**Rationale:** Pre-routing limits keep a hostile or looping agent from flooding family members, and keep the decision layer total and deterministic.

## Alternatives Considered

### Add modes to the existing interrupt classifier only

Rejected because the classifier decides how an arriving message interrupts, not whether and how it should be delivered. Delivery semantics need their own owner with roster and limit authority.

### Use transport-native delivery semantics

Rejected because Iroh, Syndicate, and stdio transports disagree on semantics. A transport-neutral contract keeps behavior identical across hosts.

### Add richer receipts (read, acted, completed)

Rejected because delivery evidence cannot establish comprehension or action. Richer receipts would be overclaims by construction.

## Risks / Trade-offs

- `steer` injection can derail active work. Mitigation: mode choice is explicit, policy can deny `steer` per roster role, and every steer produces a receipt.
- Roster state can go stale as sessions end. Mitigation: routing consumes host-supplied roster snapshots and fails closed on unknown targets.
- Rate limits need counters; wrong counter inputs weaken them. Mitigation: counter identity appears in limit receipts so hosts can audit which counter decided.

## Non-Claims

This design does not establish transport security, exactly-once or ordered delivery, message comprehension, agent cooperation, cross-machine reachability, production readiness, or release eligibility.
