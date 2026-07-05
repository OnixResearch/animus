# Design: Preserve typed adapter call outcomes

## Context

Animus already keeps model/tool request and response DTOs typed, but the runtime shell can reduce adapter errors to `Display` output when building `AdapterOutcome::Err`. That is safe enough for compatibility, but it prevents downstream reports and receipts from distinguishing denied capability, unavailable service, malformed output, cancellation, timeout, and execution failure without parsing prose.

## Decisions

### 1. Add an explicit adapter-call outcome DTO

**Choice:** Add a reusable outcome/envelope type in `animus-adapters` with variants for completed response and fail-closed classes. Failure variants carry stable class labels, bounded safe messages, optional request correlation identity, and optional safe receipt/reference metadata.

**Rationale:** Runtime can preserve machine-readable failure semantics without trusting raw adapter error strings.

### 2. Keep the old trait path as compatibility

**Choice:** Keep `ComponentAdapter<Request, Response>::call` unchanged and introduce an opt-in typed-call helper or v2 trait. The runtime path can normalize legacy errors into a conservative execution-failure class.

**Rationale:** Existing embedders should not break while new adapters can provide richer receipts.

### 3. All envelope evidence is safe by construction

**Choice:** Validate safe failure text and metadata with the existing adapter field bounds, NUL rejection, safe receipt rules, and BLAKE3 hashes for raw payload identity. The envelope must not copy raw prompts, provider payloads, credentials, private paths, or unbounded outputs.

**Rationale:** Diagnostics are useful only if they preserve the adapter boundary safety contract.

### 4. Runtime reports use typed outcomes first

**Choice:** Runtime dispatch should prefer typed adapter outcomes when an adapter supplies them, map failure classes to `OutcomeKind`/`RuntimeFailure`, and preserve compatibility behavior for old adapters.

**Rationale:** This produces better evidence while retaining current behavior.

## Risks / Trade-offs

- Adding a second trait can increase API surface. Mitigate with focused preludes and migration docs.
- Mapping legacy errors is necessarily lossy. Treat them as conservative execution failures and do not infer capabilities from strings.
- Safe receipt metadata can become verbose. Keep receipt fields bounded and summarize large payloads with BLAKE3 identity only.
