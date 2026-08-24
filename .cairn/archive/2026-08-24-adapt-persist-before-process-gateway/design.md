## Context

Animus separates pure session and queue reducers from daemon, storage, clock, transport, and provider shells. A gateway can reuse these contracts only when durable admission precedes effects and ordering is explicit.

`digitallyinduced/haskell-agent` persists incoming Telegram updates and pending replies before processing. It continues polling while turns run, processes each conversation in order, allows separate conversations to run concurrently, and resumes pending work after restart.

## Goals

- Admit inbound work durably before agent processing.
- Admit reply intent durably before transport delivery.
- Preserve per-conversation order with named cross-conversation concurrency bounds.
- Resume pending work idempotently after restart.
- Keep remote mutation authority denied by default.

## Non-goals

- Telegram protocol, bot tokens, webhook hosting, or polling implementation.
- Proof of remote identity, delivery, or exactly-once transport behavior.
- Global scheduling policy for every Animus queue.
- Provider, model, or tool correctness claims.

## Decisions

### Decision: Durable admission precedes processing

**Choice:** The pure reducer produces an admission plan for each inbound update. The storage shell commits the update identity, conversation identity, payload identity, and pending state before the agent turn can start.

**Rationale:** Processing first can lose the only copy after a crash.

### Decision: Reply intent precedes delivery

**Choice:** The session records a pending reply intent and exact content identity before the transport sends it. Delivery observations update that record but do not replace it.

**Rationale:** A restart needs to distinguish unsent, uncertain, and observed-delivered replies.

### Decision: Ordering is per conversation

**Choice:** One conversation processes admitted updates in order. Separate conversations can progress concurrently under an explicit host bound. The core emits plans and never owns threads or executors.

**Rationale:** Global serialization wastes capacity, while same-conversation concurrency can reorder user meaning.

### Decision: Restart resumes idempotently

**Choice:** Recovery reconstructs pending updates and replies from durable records. Stable identities prevent duplicate processing and shape uncertain delivery as a visible state.

**Rationale:** A crash cannot prove whether an external transport accepted a send. Recovery must not invent certainty.

### Decision: Remote mutation is denied by default

**Choice:** Gateway sessions expose read-only tools unless an exact current authority profile admits mutation for the session, user binding, tool, and request.

**Rationale:** Transport admission does not grant filesystem, process, credential, or external-service authority.

## Dependency Direction

```text
remote transport and durable store shells
                   ^
                   |
gateway DTOs -- pure admission/order/recovery reducers -- session authority
```

The core does not poll, send messages, read credentials, spawn tasks, read clocks, or write storage.

## Adversarial Audit

The audit must try to show that Animus:

- begins processing before durable inbound admission;
- sends a reply before recording its pending intent;
- reorders two updates in one conversation;
- exceeds the named cross-conversation concurrency bound;
- duplicates processing after restart;
- reports uncertain delivery as delivered;
- exposes a mutating tool without exact gateway-session authority.

Any successful counterexample blocks closeout.

## Risks and Trade-offs

- Persist-before-process adds latency and storage writes. It removes crash windows.
- Exactly-once delivery is impossible without transport support. The state model must preserve uncertainty.
- Per-conversation ordering can let one blocked turn delay later messages in that conversation. Cancellation and parking remain explicit.

## Non-Claims

This change does not prove remote identity, transport authentication, exactly-once delivery, message freshness, tool safety, host sandboxing, or external service correctness.
