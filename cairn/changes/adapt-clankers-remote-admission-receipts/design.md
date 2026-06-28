# Design: Adapt Clankers remote admission receipts for Animus

## Context

Clankers uses public UCAN and Basalt policy for remote daemon admission. Animus already treats Basalt/UCAN as the intended authority shape for guarded actions. The transferable part is the admission receipt contract: an entrypoint request is authorized before side effects and leaves safe evidence. The non-transferable parts are Clankers' specific daemon, iroh, Matrix, and CLI surfaces.

## Decisions

### 1. Admission is an entrypoint gate

**Choice:** Remote/headless entrypoints create typed admission requests before session create, attach/resume, prompt submission, tool execution, memory mutation, artifact delivery, or workflow command side effects.

**Rationale:** Entry surfaces must not bypass the same capability model used inside the runtime.

### 2. Requests carry audience and capability shape

**Choice:** Admission requests include audience, actor, entrypoint, session/workspace scope, requested action, resource/ability labels, credential evidence, local caveats, policy hash, and requested expiry/window metadata.

**Rationale:** Remote credentials are only meaningful for the intended audience and attenuated scope.

### 3. Local caveats narrow authority

**Choice:** Local policy may deny, narrow, or require confirmation for admitted actions. It may not expand authority beyond supplied verified credentials or session grants.

**Rationale:** This aligns with the existing UCAN-first capability direction.

### 4. Transports and stores are adapters

**Choice:** Daemon sockets, iroh, Matrix, MCP, HTTP, OAuth stores, credential persistence, and revocation databases remain host adapter behavior. Reusable admission logic validates typed evidence and receipts.

**Rationale:** Animus should support multiple entry transports without making any product transport mandatory.

### 5. Receipts are redacted admission evidence

**Choice:** Receipts record verdict, reason class, entrypoint label, resource/ability labels, audience hash, actor hash, credential hash, caveat summary, policy hash, expiry class, and BLAKE3 receipt hash. Raw tokens, private keys, prompts, provider payloads, transport headers, Matrix room IDs, and full paths are excluded.

**Rationale:** Admission evidence is security-sensitive and must be safe to inspect.

## Risks / Trade-offs

- Admission receipts can overclaim security if credential verification is stubbed. Tests must distinguish missing/verifier-unavailable from denied/authorized.
- Transport adapters may want extra fields. Keep adapter metadata separate from core receipt fields.
- Mandatory Basalt dependencies could make core less embeddable. Keep compatibility behind optional features or adapter boundaries.
