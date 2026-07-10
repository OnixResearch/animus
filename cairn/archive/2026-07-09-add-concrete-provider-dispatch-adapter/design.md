# Design: Concrete provider dispatch adapter

## Context

Provider execution currently stops at reusable contracts and fixtures. `docs/clankers-provider-contract-parity.md` defines future rails: shared request fields, constructor inventory, provider-native message conversion, parser-entrypoint streaming tests, and known provider prefix fail-closed behavior. The std shell also has provider registry and streaming parser seams, but no complete provider dispatch adapter claim.

## Decisions

### 1. Host-owned dispatch shell

**Choice:** Concrete provider dispatch lives in `animus-std` or a host crate. Reusable crates accept typed requests, capability matrices, and outcomes only.

**Rationale:** Network clients, credentials, retries, endpoints, provider SDKs, and live checks are host authority.

### 2. One provider family first

**Choice:** Start with one selected provider-family fixture path before claiming broad parity.

**Rationale:** A narrow vertical slice proves the pattern and avoids accidental provider abstraction overreach.

### 3. Dispatch prerequisites are explicit data

**Choice:** The adapter plan requires selected provider/model registry entry, canonical request, projected provider request, credential handle state, service readiness, timeout/deadline metadata, cancellation metadata, and receipt policy.

**Rationale:** Missing prerequisites must fail closed before provider contact.

### 4. Parity rails guard drift

**Choice:** Add shared-field and constructor inventory checks once concrete request structs exist.

**Rationale:** Provider-specific builders can silently drop cross-cutting fields such as model, messages, tools, session metadata, reasoning, stream intent, and extra provider parameters.

### 5. Live smoke is optional evidence

**Choice:** Live provider tests run only with explicit host configuration and produce safe evidence summaries.

**Rationale:** Default checks must not require credentials, network, or provider availability.

## Risks / Trade-offs

- A single provider family can be over-read as general parity. Docs and receipts must scope the claim.
- Provider SDKs can bring large dependencies. Keep the first adapter host-owned and feature-gated.
- Live smoke can be flaky. Deterministic fixture tests remain the archive gate; live smoke is optional evidence.
