# Stream-host composition design

## Context

Rust futures already release executor threads across asynchronous waits. Bounded queues do not bound every task, retained body, or subscription lifetime. A wakeup is not an event count or durable delivery acknowledgement.

## Decisions

### Decision: Conformance before another abstraction

**Choice:** Reuse existing stream, replay, authority-seal, and local-daemon contracts. Select one concrete host path and list every production symbol that the fixture exercises. Add only missing invariants to existing owners.

**Rationale:** Baz continuations do not justify another executor. `component-lifecycle` remains a reuse candidate for cleanup decisions, not an automatic dependency. Any adoption needs an immutable publication and a consumer-specific parity proof.

### Decision: Explicit resource profiles

**Choice:** A default-off host conformance profile defines `max_subscriptions`, `max_retained_bytes`, `max_pending_events`, `max_events_per_drain`, `stream_deadline_ms`, and `shutdown_deadline_ms`. Typed Nickel requires positive values and checked aggregate arithmetic. Host shells receive admitted values. The core receives explicit capacity, generation, time, and authorization facts.

A successful permit reserves both task capacity and output capacity before new model or tool dispatch. Existing durable attempt and receipt gates remain separate prerequisites. Reserve-before-effect logic must reuse existing tool stages and consider published `transactional-reconciliation-core` before another mechanism. A progress subscriber cannot veto cleanup or erase authoritative operation state.

### Decision: Wakeups only schedule observation

**Choice:** Producers publish synchronized state before notification. Consumers drain state with a fixed budget. Remaining work schedules another bounded wakeup. Coalesced notifications do not imply lost events. An old generation cannot wake a replacement subscription. Overflow emits a typed lag or resnapshot outcome rather than silently deleting semantic events.

**Rationale:** These cases exercise lost-wakeup and stale-handle risks without introducing a shared queue implementation.

### Decision: Fixed lifetime and authorization

**Choice:** The original stream deadline persists across flush, wait, and reconnect operations within one admitted stream. A new stream requires new admission. Current authorization is checked before each protected batch and reconnect. Revocation closes delivery without discarding committed operation results.

The existing replay planner keeps session, generation, branch, policy, and redaction fences. It excludes live-only deltas from durable replay. No second transcript or transport-owned replay ledger is allowed.

### Decision: Cancellation is a request, not a receipt

**Choice:** Terminal cleanup runs logically once after all dependent accesses end. Cancellation and drain are distinct states. Outstanding effects remain pending or uncertain until the owning port supplies an observation. A shutdown timeout records unresolved ownership and forbids slot reuse.

Before publication, a failed response draft can become an error. After publication, an error terminates the stream. Output pressure cannot rerun a tool, provider call, or durable commit. Enqueue acceptance, local flush, peer delivery, and durable completion remain distinct observations.

## Risks / Trade-offs

A fake-backed conformance pass proves the selected contract, not provider cancellation or durable storage. Runtime-specific scheduling remains in hosts. An uncooperative external tool can outlive cancellation. Existing host authority and process containment remain required.

## Deferred work

HTTP/SSE transport selection belongs in a future consumer change after the bounded inbound evaluation. Copy reduction requires measured retained-byte pressure. This package does not duplicate `receipt-live-agent-actions` or change its tasks.
