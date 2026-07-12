## Context

Mesh LLM presents a local OpenAI-compatible endpoint while routing requests over its own Iroh mesh. Its current agent guidance prefers exact model IDs and Chat Completions for broad compatibility, while its certification path also probes the Responses API. It additionally exposes an experimental virtual `mesh` model, smart routing, and fallback behavior.

Animus already owns canonical model requests/responses, provider capability metadata, model-router execution, remote admission, exact runtime-profile composition, retry/fallback separation, and metadata-only receipts. The integration must preserve those owners instead of treating OpenAI compatibility as permission to bypass them.

## Decisions

### 1. Keep Mesh LLM external to Animus reusable crates

**Choice:** Add a non-default std-host adapter/profile around a caller-owned HTTP transport service. Reusable core, adapter, runtime, router, Steel, Preserves schema, and default std surfaces do not start Mesh, construct Iroh endpoints, read Mesh state, parse invite tokens, discover peers, configure relays, or persist Mesh identity.

**Rationale:** Mesh is an inference substrate, not Animus communication infrastructure. HTTP transport and process lifecycle are shell authority.

### 2. Separate semantic model selection from physical placement

**Choice:** Animus selects an exact provider-qualified model ID after capability, privacy, role, budget, authority, and fallback policy. Mesh may route that exact target to a local host, full-model peer, or separately qualified split topology, but it may not change the semantic model under the initial profile.

**Rationale:** Physical placement is a provider execution detail. Model substitution changes prompt/tool capability and evidence semantics and therefore requires fresh Animus composition.

### 3. Deny virtual and ambient model selection initially

**Choice:** The initial profile rejects `auto`, the experimental `mesh` virtual model, unspecified model IDs, and provider-side cross-model fallback. A response that identifies a model other than the admitted target fails normalization.

**Rationale:** Mesh smart routing and Mixture-of-Agents would duplicate or bypass Animus router, role, quorum, fallback, and receipt policy.

### 4. Treat the model catalog as an observed snapshot

**Choice:** The host fetches `/v1/models` and supplies an already-loaded snapshot plus endpoint/config identity to a pure normalizer. The normalizer validates bounded counts, safe IDs, duplicate IDs, capability classes, modality/tool/stream metadata, context/output bounds, freshness metadata supplied by the host, and explicit unknowns. Missing or unsupported metadata denies required capabilities rather than inventing them.

**Rationale:** Discovery is an effect; compatibility decisions are deterministic core logic. A catalog observation is not proof that a peer will honestly execute the advertised model.

### 5. Require an explicit OpenAI API style

**Choice:** The profile binds either Responses or Chat Completions explicitly. Responses may reuse the existing reviewed projection only after Mesh-specific fixtures pass. If Chat Completions is required, it receives a separate projection/conformance inventory rather than being treated as Responses-compatible by URL shape alone.

**Rationale:** OpenAI-compatible gateways differ in developer roles, tools, streaming, usage, structured output, and error shapes.

### 6. Keep HTTP execution in an injected shell

**Choice:** A thin host service owns request serialization, network contact, deadlines, cancellation, bounded response reads, streaming frame acquisition, and provider-native diagnostics. Pure functions own profile validation, catalog normalization, request/response parity, exact-model checks, error-class mapping, and receipt projection.

**Rationale:** Every policy decision remains testable without standing up Mesh or a network.

### 7. Reuse Animus authority before effects

**Choice:** Mesh dispatch requires the existing admitted model/provider capability and remote-admission decision for the exact resource and ability. Mesh invite tokens, endpoint public keys, owner attestations, release attestations, and peer membership never substitute for UCAN/Basalt-backed or host-supplied Animus authority.

**Rationale:** Transport identity and build provenance are not inference-use authority.

### 8. Compose with retry without widening fallback

**Choice:** Mesh-native transient failures map into the canonical provider failure classes owned by the provider-retry change when that change is available. Automatic retry remains same-target and pre-semantic-commit only. Retry exhaustion returns to Animus router policy; the Mesh adapter never selects another model.

**Rationale:** The active provider-retry work and this adapter share error/stream seams but must not create duplicate retry or fallback engines.

### 9. Keep receipts safe and claims bounded

**Choice:** Receipts bind safe provider/model labels, endpoint/config/catalog identities, API style, capability decision, request/response identities, finish/usage classes, authority/policy refs, and redaction markers. They omit raw prompts, responses, tool bodies, hidden reasoning, credentials, invite tokens, endpoint keys, relay data, headers, peer addresses, and unbounded logs. Physical route is recorded as unknown unless independently supplied and validated; even a supplied route observation remains recorded-only.

**Rationale:** An HTTP response does not prove which peer, stages, binary, model bytes, or host executed it.

### 10. Keep live smoke separate from deterministic conformance

**Choice:** Required closeout evidence is fixture-backed and deterministic. An optional live smoke may target an operator-supplied loopback/private Mesh endpoint and exact model, but its receipt remains host/run scoped and cannot promote production or split-inference claims.

**Rationale:** CI should not require GPUs, model downloads, public discovery, or external relays to validate adapter semantics.

## Cross-Repo Coordination

- `onixos:add-mesh-llm-inference-service` owns the pinned package, private Mesh roles, state/secrets, listener policy, supervision, and local endpoint export.
- This Animus change owns semantic model selection, API conformance, inference admission, and provider receipts; it MUST NOT acquire a build-time workspace-relative dependency on OnixOS.
- End-to-end dogfood promotion requires evidence from both changes, while each repository's deterministic fixture gates remain independently runnable.
- Existing Basalt/UCAN and Valence/Preserves boundaries are reused initially; new repository-specific Cairns are required only if their public policy or receipt schemas must change.

## Risks / Trade-offs

- Exact-model denial prevents use of Mesh smart routing and MoA until Animus can represent their semantics explicitly.
- Mesh capability metadata may be incomplete or version-sensitive; conservative denial can reduce usable features.
- Responses and Chat Completions behavior may diverge despite both endpoints existing.
- Concurrent provider-retry work may touch nearby modules and docs; implementation should isolate Mesh code and rebase after the shared retry contracts stabilize.
- Hash-only request/response evidence does not prove provider behavior and may require stronger privacy treatment for low-entropy content.

## Non-Goals

- Embedding or forking Mesh LLM, Skippy, llama.cpp, or Mesh Iroh protocols.
- Starting, stopping, upgrading, configuring, or supervising Mesh processes from Animus.
- Public mesh discovery, Nostr, blackboard, plugins, peer artifact transfer, or owner-key management.
- Adopting Mesh smart routing, `model = "mesh"`, provider-side model fallback, or split-topology policy.
- Proving model correctness, peer honesty, remote runtime integrity, physical route truth, confidentiality, latency, throughput, or production readiness.
