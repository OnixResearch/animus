## Phase 1: Boundary and policy

- [x] [serial] Inventory the existing canonical model, provider registry, concrete dispatch, streaming parser, router/fallback, remote admission, UCAN/Basalt, retry, model-profile, and receipt owners; document exact overlap and implementation order with active provider-retry work. r[agent_harness_runtime.mesh_llm_provider.boundary] r[agent_harness_runtime.mesh_llm_provider.verification]
- [x] [serial] Author a typed Nickel Mesh gateway profile for explicit API style, private-gateway posture, exact-model requirements, virtual-model denial, capability requirements, safe catalog bounds, and receipt/non-claim policy; generate and revalidate its Rust projection. r[agent_harness_runtime.mesh_llm_provider.boundary] r[agent_harness_runtime.mesh_llm_provider.exact_model]
- [x] [serial] Define already-loaded catalog snapshot, normalized model row, profile decision, exact-target decision, transport request/outcome, provider observation, and safe receipt DTOs with named bounds and domain-separated BLAKE3 identities. r[agent_harness_runtime.mesh_llm_provider.catalog] r[agent_harness_runtime.mesh_llm_provider.receipts]

## Phase 2: Pure validation and projection

- [x] [serial] Implement pure Mesh profile and catalog normalization for safe IDs, bounded rows, duplicate rejection, explicit unknown capabilities, context/output bounds, host-supplied freshness, privacy posture, and deterministic ordering. r[agent_harness_runtime.mesh_llm_provider.catalog]
- [x] [serial] Implement exact-model admission that rejects missing targets, `auto`, experimental `mesh`, provider-side cross-model fallback, and response model substitution while leaving physical placement opaque. r[agent_harness_runtime.mesh_llm_provider.exact_model]
- [x] [serial] Add explicit Responses compatibility fixtures for Mesh request, response, tools, streaming, usage, structured output, developer-role, cancellation, and error behavior; add a separate Chat Completions projection only if the Responses profile cannot satisfy the admitted contract. r[agent_harness_runtime.mesh_llm_provider.api_conformance]
- [x] [serial] Bind provider contact to existing model capability, privacy, budget, authority, remote-admission, deadline, cancellation, and policy decisions before an injected transport request may be emitted. r[agent_harness_runtime.mesh_llm_provider.authority]
- [x] [serial] Implement metadata-only receipt projection and validation with explicit remote-runtime, peer-honesty, physical-route, model-correctness, split-parity, and production-readiness non-claims. r[agent_harness_runtime.mesh_llm_provider.receipts]

## Phase 3: Thin host shell

- [x] [serial] Add a non-default std-host Mesh adapter around a caller-owned bounded HTTP transport service for `/v1/models` and the selected explicit inference API style; keep endpoint discovery, credentials, process lifecycle, Iroh, relay, ticket, and persistence behavior outside reusable crates. r[agent_harness_runtime.mesh_llm_provider.boundary]
- [x] [serial] Normalize transport outcomes and provider-native failures through existing canonical response, stream, cancellation, timeout, and retry-class boundaries without implementing adapter-local fallback or retry policy. r[agent_harness_runtime.mesh_llm_provider.api_conformance] r[agent_harness_runtime.mesh_llm_provider.exact_model]
- [x] [serial] Wire the adapter through provider registry/model runtime composition and feature exports without making Mesh, HTTP, Iroh, or a live endpoint a default dependency. r[agent_harness_runtime.mesh_llm_provider.boundary]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive fixtures for a bounded private-gateway catalog, exact text model, tool-capable model, Responses request/response, streaming content/tool/usage frames, explicit Chat Completions profile if implemented, admitted authority, same-target retry handoff, and deterministic safe receipts. r[agent_harness_runtime.mesh_llm_provider.verification]
- [x] [parallel] Add negative fixtures for malformed/oversized/stale catalogs, duplicate or unsafe IDs, unknown required capabilities, `auto`, `mesh`, missing model, response substitution, unsupported roles/tools/streaming/structured output, ambiguous API style, malformed stream/error payloads, missing/denied authority, timeout/cancellation, cross-model fallback, ambient discovery, and receipt leaks. r[agent_harness_runtime.mesh_llm_provider.catalog] r[agent_harness_runtime.mesh_llm_provider.exact_model] r[agent_harness_runtime.mesh_llm_provider.api_conformance] r[agent_harness_runtime.mesh_llm_provider.authority] r[agent_harness_runtime.mesh_llm_provider.verification]
- [x] [parallel] Add dependency and source-shape guards proving reusable crates do not construct HTTP or Iroh clients, read Mesh config/state, inspect environment credentials, launch Mesh, parse invite tokens, or persist endpoint identity. r[agent_harness_runtime.mesh_llm_provider.boundary] r[agent_harness_runtime.mesh_llm_provider.verification]
- [x] [parallel] Add an opt-in live smoke profile for an operator-supplied loopback/private endpoint and exact model; emit a bounded run receipt and keep absence or failure separate from deterministic adapter conformance. r[agent_harness_runtime.mesh_llm_provider.verification]

## Phase 5: Documentation and closeout

- [x] [serial] Update provider dispatch, streaming, router execution, model runtime profiles, remote admission, request lifecycle, component catalog, runtime facade, dogfood readiness, and README reference documentation with Mesh ownership and non-claims. r[agent_harness_runtime.mesh_llm_provider.boundary] r[agent_harness_runtime.mesh_llm_provider.receipts]
- [x] [serial] Add a focused Mesh provider rail covering Nickel drift, catalog normalization, exact-model fidelity, API-style parity, authority-before-effect, fallback separation, default dependency isolation, and safe receipts. r[agent_harness_runtime.mesh_llm_provider.verification]
- [x] [serial] Run focused adapter/runtime/std/provider tests, formatting, Clippy, Nickel checks, projection/parser parity, retry integration where available, composition conformance, safe-receipt and dogfood-hygiene rails, Cairn validation/gates, and relevant Nix checks; record live smoke and OnixOS deployment as separate evidence. r[agent_harness_runtime.mesh_llm_provider.verification]
