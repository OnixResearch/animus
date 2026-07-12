## Why

Animus has provider-neutral model contracts, explicit model routing, capability negotiation, remote admission, safe receipts, and fixture-backed OpenAI-family projections, but it does not have a bounded host adapter profile for Mesh LLM. Mesh LLM exposes an OpenAI-compatible API while independently owning peer discovery, Iroh transport, physical model placement, optional model fallback, and experimental multi-model behavior.

A direct but unbounded integration could let Mesh virtual models or provider-side fallback bypass Animus model-role composition, exact-target policy, separation of duty, authority checks, and receipt claims. Animus should therefore adapt Mesh LLM only as a host-owned private-gateway provider: Animus selects one exact semantic model, while Mesh may place that model locally, on a peer, or—after separate operational qualification—across split stages.

## What Changes

- Add a non-default Mesh LLM provider profile and host adapter boundary without adding Iroh, endpoint, discovery, credential, or process authority to reusable crates. r[agent_harness_runtime.mesh_llm_provider.boundary]
- Normalize an already-fetched `/v1/models` snapshot into typed provider/model capability metadata and fail closed on malformed, stale, ambiguous, or unsupported capability claims. r[agent_harness_runtime.mesh_llm_provider.catalog]
- Require an exact admitted model ID and reject initial use of provider-selected `auto`, experimental `mesh`, ambient cross-model fallback, or response model substitution. r[agent_harness_runtime.mesh_llm_provider.exact_model]
- Bind dispatch to an explicit tested OpenAI API style and add fixture parity for request, response, streaming, tool-call, usage, and unsupported-feature behavior. r[agent_harness_runtime.mesh_llm_provider.api_conformance]
- Reuse existing Animus admission and capability policy before provider contact; Mesh membership, Iroh identity, owner trust, and release attestation remain transport/build evidence rather than inference authority. r[agent_harness_runtime.mesh_llm_provider.authority]
- Emit metadata-only provider receipts with explicit physical-route and remote-runtime non-claims. r[agent_harness_runtime.mesh_llm_provider.receipts]
- Add positive and negative fixtures plus an optional externally supplied live smoke profile. r[agent_harness_runtime.mesh_llm_provider.verification]

## Impact

- **Reusable core/adapters**: gain only transport-neutral DTO or pure validation changes that are required for canonical provider mapping; they do not gain HTTP or Iroh behavior.
- **Std host shell**: owns Mesh catalog retrieval, HTTP transport injection, response parsing, cancellation/deadline plumbing, and provider-native error classification behind a non-default feature/profile.
- **Router**: remains the semantic model and fallback authority; Mesh owns only physical placement for the admitted exact model.
- **Policy**: typed Nickel remains the source of truth for the supported Mesh gateway profile, virtual-model denial, capability requirements, and safe receipt shape.
- **Claims**: fixture conformance does not establish Mesh availability, model correctness, peer honesty, physical route truth, remote runtime integrity, split parity, WAN performance, or production readiness.

## Affected Specs

- `agent-harness-runtime`: Mesh provider boundary, catalog normalization, exact-model ownership, API conformance, authority reuse, receipts, and verification.
