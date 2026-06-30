# Design: Adapt Pi runtime event lifecycle for Animus

## Context

Pi's extension lifecycle is useful because it names every important transition: input, context construction, model request, streaming message updates, tool preflight, tool result, compaction, tree navigation, and shutdown. Animus should adapt the event vocabulary and ordering guarantees while keeping the implementation Rust-native, typed, and host-injected.

## Decisions

### 1. Events are runtime boundary DTOs

**Choice:** Define Animus runtime event enums/structs for lifecycle transitions and ensure event payloads carry only typed IDs, statuses, bounded text, hashes, and DTO references appropriate for that boundary.

**Rationale:** Events are reusable across CLI, RPC, tests, and embedders. A typed contract prevents hidden stringly-typed extension coupling.

### 2. Middleware is injected by the host

**Choice:** Runtime accepts middleware/hook implementations as injected services or trait objects. It does not discover extensions or load code from disk.

**Rationale:** Animus is a framework core. Host applications decide plugin loading, trust, sandboxing, and UI.

### 3. Hook ordering is deterministic

**Choice:** Hooks run in configured order. Mutation hooks chain explicitly, and each later hook sees the previous accepted mutation.

**Rationale:** Permission, policy, and tracing hooks need reproducible behavior for tests and receipts.

### 4. Sensitive hooks fail safe

**Choice:** Errors in hooks that guard model calls, tool calls, persistence, compaction, or session replacement deny or abort the protected operation by default.

**Rationale:** A broken guard should not silently allow unsafe tool or provider activity.

### 5. Events are mode-neutral

**Choice:** The same event stream backs direct SDK embedding, headless RPC, receipts, and future UI integrations.

**Rationale:** Pi's TUI and RPC modes share event semantics. Animus can keep UI out of scope while preserving that portability.

## Risks / Trade-offs

- Too many event types can freeze internals prematurely. Start with stable boundaries only.
- Mutation hooks can make control flow hard to reason about. Limit mutation to explicit pre-dispatch surfaces and prefer immutable observe events elsewhere.
- Hook failures need careful semantics. Tests must prove fail-safe behavior where security or data integrity is involved.
