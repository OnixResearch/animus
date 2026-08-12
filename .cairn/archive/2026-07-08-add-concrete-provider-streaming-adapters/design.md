# Design: Add concrete provider streaming adapters

## Context

The accepted `agent_harness_runtime.model_streaming.*` requirements define canonical stream frames, capability negotiation, runtime event projection, parser-entrypoint fixture obligations, and thought/reasoning redaction defaults. They deliberately do not add a concrete network provider.

This change owns the next boundary: optional host-owned provider adapters that can prove real provider stream chunks become canonical frames without weakening crate boundaries or safety policy.

## Decisions

### 1. Concrete provider streaming is shell-owned and feature-gated

**Choice:** Concrete provider stream clients and network transports live in host/shell-owned code paths, likely `animus-std` or an optional host crate, behind explicit features or adapter registration. Reusable `animus-core`, `animus-adapters`, `animus-router`, and runtime core continue to depend only on DTOs, traits, pure parser cores, and injected services.

**Rationale:** The current architecture keeps credentials, endpoints, network I/O, retries, and provider SDK choices outside reusable crates. Provider streaming should not become the exception.

### 2. Parsers have a pure core and a thin transport shell

**Choice:** Provider-specific stream parsing is split into a deterministic parser over admitted chunks and a thin imperative shell that reads bytes from the provider transport. Tests feed raw chunks through the same parser entrypoint used by the shell.

**Rationale:** Stream bugs usually appear in fragmentation, terminal frames, error frames, and partial records. A pure parser core makes those cases testable without live credentials or network timing.

### 3. Capability negotiation gates streaming before dispatch

**Choice:** A host request for content, thought/reasoning, tool-call delta, or usage streaming must resolve against selected model capabilities before provider dispatch. Unsupported modes fail closed under reject policy or downgrade only through an explicit content-only policy.

**Rationale:** Silent fallback would make runtime events and UI behavior lie about what the selected provider supports.

### 4. Thought/reasoning channels keep the accepted visibility policy

**Choice:** Provider-exposed reasoning, thinking, or summary chunks map to `ThoughtDelta` only with policy-gated or redacted default visibility unless a host policy creates a safe projection. Raw hidden reasoning is never merged into assistant-visible final text, receipts, dataset rows, review bodies, or logs by default.

**Rationale:** Providers use incompatible names for reasoning. The adapter must preserve channel separation without creating a chain-of-thought leak.

### 5. Live-provider checks are optional evidence, not the default rail

**Choice:** Required verification uses checked-in fixtures and deterministic parser/runtime tests. Optional live-provider tests may exist behind explicit credentials and ignored/manual gates, but release evidence must distinguish them from fixture-backed support.

**Rationale:** Animus should be testable in CI without provider credentials while still allowing downstream hosts to attach live conformance evidence.

## Risks / Trade-offs

- Provider stream formats differ and change over time. The mitigation is small provider-family parser modules with raw fixture coverage and fail-closed unknown records.
- Feature-gated provider adapters increase host-shell surface area. The mitigation is keeping all effects in the imperative shell and keeping reusable crates provider-client-free.
- Thought/reasoning naming varies by provider. The mitigation is canonical frame kinds plus explicit visibility policy rather than provider-native public APIs.
