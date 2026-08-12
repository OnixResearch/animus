# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Mesh LLM remains a host-owned provider boundary [r[agent_harness_runtime.mesh_llm_provider.boundary]]

Animus MUST integrate Mesh LLM through a non-default host provider adapter or injected service, and reusable core, adapter, runtime, router, Steel, Preserves-schema, and default std surfaces MUST NOT start Mesh processes, construct HTTP or Iroh clients, read Mesh state or credentials, parse invite tokens, configure relays, discover peers, or persist Mesh endpoint identity.

#### Scenario: Valid host adapter receives an injected transport [r[agent_harness_runtime.mesh_llm_provider.boundary.scenario.injected]]

- GIVEN a host has selected a reviewed Mesh profile and supplied an injected bounded HTTP transport service
- WHEN Animus prepares a catalog or inference call
- THEN the host adapter MAY emit the validated transport request
- AND reusable runtime logic MUST require no live Mesh process, network, credential, filesystem, environment, clock, or Iroh dependency.

#### Scenario: Ambient Mesh discovery is attempted [r[agent_harness_runtime.mesh_llm_provider.boundary.scenario.ambient]]

- GIVEN no Mesh transport or endpoint descriptor was injected
- WHEN provider dispatch is requested
- THEN Animus MUST return a typed missing-service or missing-endpoint failure before network effects
- AND it MUST NOT run `mesh-llm`, inspect process-global configuration, discover public meshes, or construct an Iroh endpoint as fallback.

### Requirement: Mesh model catalogs normalize conservatively [r[agent_harness_runtime.mesh_llm_provider.catalog]]

An already-fetched Mesh `/v1/models` snapshot MUST be normalized by pure deterministic logic into bounded provider/model metadata with safe unique IDs, explicit capability support or unknown status, context and output bounds, private-gateway posture, deterministic ordering, endpoint/config/catalog identity, and host-supplied freshness metadata.

#### Scenario: Valid catalog becomes a provider registry snapshot [r[agent_harness_runtime.mesh_llm_provider.catalog.scenario.valid]]

- GIVEN a bounded catalog contains unique safe model IDs and explicit capability metadata sufficient for the selected request
- WHEN catalog normalization runs
- THEN it MUST produce deterministic provider registry entries and a catalog identity
- AND catalog observation MUST NOT be treated as proof that a peer honestly hosts or executes the advertised model.

#### Scenario: Required capability is unknown [r[agent_harness_runtime.mesh_llm_provider.catalog.scenario.unknown]]

- GIVEN a catalog omits or ambiguously reports a capability required by the canonical request
- WHEN model compatibility is evaluated
- THEN dispatch MUST be denied unless an explicit reviewed downgrade policy covers that optional feature
- AND the adapter MUST NOT infer support from model names, provider marketing, or another peer's metadata.

### Requirement: Animus owns the exact semantic model [r[agent_harness_runtime.mesh_llm_provider.exact_model]]

The initial Mesh profile MUST require one exact admitted model ID selected by Animus and MUST reject an omitted model, provider-selected `auto`, the experimental `mesh` virtual model, ambient cross-model fallback, and a normalized response whose reported model differs from the admitted target.

#### Scenario: Exact model is physically routed by Mesh [r[agent_harness_runtime.mesh_llm_provider.exact_model.scenario.physical-route]]

- GIVEN Animus admits an exact model after capability, privacy, budget, authority, and profile composition
- WHEN Mesh executes that exact target locally, on a full-model peer, or through a separately qualified split topology
- THEN Animus MAY accept the normalized result when the semantic model identity remains exact
- AND physical placement MUST remain provider execution metadata rather than a new Animus model-selection decision.

#### Scenario: Mesh substitutes another model [r[agent_harness_runtime.mesh_llm_provider.exact_model.scenario.substitution]]

- GIVEN Animus admitted one exact model but the response identifies another model or a provider fallback class
- WHEN response normalization runs
- THEN the adapter MUST reject the response as target substitution
- AND it MUST require fresh router composition instead of accepting the fallback silently.

### Requirement: Mesh API compatibility is explicit and fixture-backed [r[agent_harness_runtime.mesh_llm_provider.api_conformance]]

Each supported Mesh provider profile MUST bind an explicit OpenAI API style and MUST have fixture-backed parity for required canonical request fields, response items, tools, streaming channels, usage, structured output, roles, cancellation, and error classes before dispatch uses that style.

#### Scenario: Responses profile passes Mesh-specific parity [r[agent_harness_runtime.mesh_llm_provider.api_conformance.scenario.responses]]

- GIVEN the selected Mesh version passes reviewed Responses request, response, tool, streaming, usage, and failure fixtures
- WHEN a compatible canonical request is projected
- THEN the existing Responses family MAY be used for that versioned profile
- AND unsupported optional or required fields MUST follow explicit downgrade or denial policy.

#### Scenario: API style is ambiguous [r[agent_harness_runtime.mesh_llm_provider.api_conformance.scenario.ambiguous]]

- GIVEN an endpoint is merely labeled OpenAI-compatible but no reviewed Responses or Chat Completions profile was selected
- WHEN dispatch planning runs
- THEN it MUST fail before provider contact
- AND URL shape alone MUST NOT choose a request or streaming parser.

### Requirement: Animus authority precedes Mesh provider effects [r[agent_harness_runtime.mesh_llm_provider.authority]]

Mesh inference dispatch MUST require existing admitted model/provider capability, privacy, budget, remote-admission, resource, ability, deadline, cancellation, and policy decisions before the host transport is invoked, and Mesh membership, invite possession, Iroh endpoint identity, owner trust, or release attestation MUST NOT grant inference authority.

#### Scenario: Authorized exact-model request reaches transport [r[agent_harness_runtime.mesh_llm_provider.authority.scenario.allowed]]

- GIVEN the exact model and request pass capability and profile checks and the supplied authority covers the selected resource and ability
- WHEN provider dispatch is planned
- THEN the host transport MAY be invoked with the validated request
- AND the authority decision MUST remain separately identifiable from Mesh transport/build evidence.

#### Scenario: Iroh identity is supplied without inference authority [r[agent_harness_runtime.mesh_llm_provider.authority.scenario.transport-only]]

- GIVEN a reachable Mesh peer, valid invite, owner attestation, or release attestation exists but Animus authority is missing or denied
- WHEN dispatch admission runs
- THEN dispatch MUST fail before HTTP or provider effects
- AND transport or build evidence MUST NOT widen the denied authority.

### Requirement: Mesh provider receipts are metadata-only and non-authoritative [r[agent_harness_runtime.mesh_llm_provider.receipts]]

Mesh provider receipts MUST bind safe provider/model labels, endpoint/config/catalog identities, API style, capability and authority decisions, request/response identities, bounded finish/usage classes, status, redaction markers, and domain-separated BLAKE3 identity while excluding raw prompts, responses, tools, reasoning, credentials, invites, keys, headers, relays, peer addresses, private paths, and unbounded diagnostics.

#### Scenario: Physical route is not independently evidenced [r[agent_harness_runtime.mesh_llm_provider.receipts.scenario.route-unknown]]

- GIVEN an inference call succeeds but no independently validated per-request route evidence identifies a peer or split stages
- WHEN receipt projection runs
- THEN physical route MUST be recorded as unknown or omitted
- AND the receipt MUST NOT claim remote runtime integrity, peer honesty, exact model bytes, split execution, or a specific serving node.

#### Scenario: Receipt input leaks provider material [r[agent_harness_runtime.mesh_llm_provider.receipts.scenario.leak]]

- GIVEN receipt input contains a raw prompt, response, tool body, hidden reasoning, credential, invite token, endpoint key, authorization header, relay URL, peer address, private path, or unbounded log
- WHEN receipt validation runs
- THEN it MUST reject the receipt without echoing the sensitive value
- AND no hash or status field may be described as proof of model correctness or physical execution truth.

### Requirement: Mesh provider adoption has positive and negative evidence [r[agent_harness_runtime.mesh_llm_provider.verification]]

Mesh provider adoption MUST include positive and negative tests for boundary isolation, catalog normalization, exact-model fidelity, API-style parity, tool and stream behavior, capability and authority admission, cancellation/timeouts, retry/fallback separation, response substitution, default dependency isolation, and receipt safety.

#### Scenario: Deterministic adapter evidence is ready for archive [r[agent_harness_runtime.mesh_llm_provider.verification.scenario.closeout]]

- GIVEN maintainers intend to sync and archive the Mesh provider change
- WHEN closeout validation runs
- THEN focused provider, runtime, std, projection/parser, policy, composition, safe-receipt, dogfood-hygiene, Cairn, and relevant Nix checks MUST pass
- AND absent live Mesh, GPU, OnixOS deployment, split, WAN, or production evidence MUST remain explicit non-claims rather than blocking fixture-scoped conformance.
