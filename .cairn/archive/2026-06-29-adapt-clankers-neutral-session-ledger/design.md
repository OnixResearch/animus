# Design: Animus neutral session ledger

## Context

Clankers separates reusable ledger replay from desktop storage. Animus should copy the architecture, not the types. The ledger is the functional core for reconstructing model-visible context; host storage remains an imperative shell concern.

## Decisions

### 1. Animus owns the DTOs

Ledger entries are Animus-defined and portable: message, summary, usage observation, safe receipt, and unsupported. Message roles include user, assistant, and tool. The first DTOs avoid clocks, file paths, database handles, daemon events, and provider-native payloads.

### 2. Replay is pure

Replay takes an in-memory slice of entries and returns model-visible messages plus safe metadata counts, or a typed error. It does not read files, inspect environment, allocate timestamps, spawn processes, call providers, or mutate storage.

### 3. Unsupported entries fail closed

Adapter-owned or future records must not be silently dropped. Replay returns a typed unsupported-entry error that includes a reason class but not the raw payload.

### 4. Resume services are injected

Runtime paths that require resume must receive a ledger service from the host. Missing or unavailable services fail before model/tool dispatch.

### 5. Branch queries stay pure and optional

If this change includes parent/head relationships, ancestry and dead-end helpers operate over Animus DTOs and return typed graph errors for duplicates, missing parents, and cycles.

## Risks / Trade-offs

- Too much transcript detail can leak product assumptions. Keep DTOs minimal until runtime tests require fields.
- Unsupported-entry failures can be strict for embedders. That is preferable to silently losing model-visible context.
- Persistence adapters may want IDs/timestamps; keep them as shell metadata, not replay requirements.