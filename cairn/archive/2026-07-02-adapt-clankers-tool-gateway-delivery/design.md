# Design: Adapt Clankers tool gateway delivery and outbox for Animus

## Context

Tool calls often produce artifacts that need to be shown, saved, attached, or sent to an external platform. Clankers' Tool Gateway keeps this safe by treating delivery as a policy-checked adapter operation with receipt-only metadata. Animus should adopt the boundary before product surfaces grow their own delivery paths.

## Decisions

### 1. Delivery is a first-class artifact operation

**Choice:** Represent delivery requests with artifact handle, artifact kind, target kind, active context label, requested toolset/capability, and safe policy metadata.

**Rationale:** Delivery is not just a side effect after a tool result. It needs authorization, target validation, and receipts.

### 2. Raw destinations are rejected by reusable policy

**Choice:** Reusable policy validation rejects raw webhooks, cloud URLs, Matrix room IDs, credentials, headers, and arbitrary platform payloads unless a host adapter has already converted them into an active context handle.

**Rationale:** Platform destinations often contain secrets. Animus should route through host-owned context, not raw destination strings.

### 3. Approved delivery calls injected adapters

**Choice:** The gateway calls an injected delivery adapter only after policy approval. Missing adapters, unsupported targets, unavailable contexts, and denied capabilities fail before side effects.

**Rationale:** Product shells may support different delivery targets. The reusable runtime should not know how to send files to any one platform.

### 4. Outbox records safe attempts

**Choice:** Each delivery attempt has an opaque attempt ID, status, retryability, target kind, artifact kind, safe artifact label, optional platform handle, error class, and BLAKE3 receipt hash. Raw payloads and destinations are excluded.

**Rationale:** Operators need observability and retry without re-entering secrets or exposing payloads.

### 5. Retry revalidates current policy

**Choice:** Retry by attempt ID loads safe attempt metadata, rechecks current policy and adapter availability, and executes only retryable failures.

**Rationale:** Retry should not bypass policy changes or convert a prior policy denial into execution.

## Risks / Trade-offs

- A receipt-only success path can overclaim delivery. Require adapter-backed success for supported targets.
- Outbox persistence is host-owned. Reusable logic should validate attempt records and retry decisions without owning storage.
- Platform context handles can leak meaning. Keep handles opaque or redacted unless a host marks them safe.
