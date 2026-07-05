# Design: Plan model/provider dispatch before execution

## Context

Animus currently keeps concrete provider clients outside reusable runtime code, and model requests/responses are typed. However, the tool side has a richer pure planning model than model/provider dispatch. Future multi-provider, streaming, multimodal, and router paths need an explicit admission plan before execution.

## Decisions

### 1. Model dispatch planning is pure

**Choice:** Add a pure model/provider dispatch plan that takes validated request metadata, capability descriptors, budget policy, stream policy, and receipt policy as inputs and returns an admitted or denied decision with safe evidence.

**Rationale:** Hosts can test provider-dispatch safety without live providers, credentials, network access, or async runtime setup.

### 2. Plans include request and provider contract identities

**Choice:** The plan records the selected model/provider capability label, request contract identity, optional provider-shape identity, stream policy, and output/receipt policy.

**Rationale:** This prevents lossy provider conversion and makes future provider parity checks easier.

### 3. Runtime executes only admitted plans

**Choice:** Runtime model dispatch first builds or receives a model dispatch plan. Non-admitted plans map to typed fail-closed outcomes and do not call the adapter.

**Rationale:** Planning must be the last pure gate before host effects.

### 4. No fallback remains explicit

**Choice:** Plans carry an explicit fallback policy that defaults to deny. Known-but-unavailable providers, missing credentials, disabled backends, unsupported stream modes, and budget failures must not silently route elsewhere.

**Rationale:** Fallback can widen authority and hide safety failures.

## Risks / Trade-offs

- Provider planning can overlap with future router parity work. Keep the first DTO provider-neutral and avoid live provider fields.
- Adding budget and stream policy can grow request metadata. Start with bounded labels/counts and extend under versioned contract identities.
- Runtime wiring must preserve existing simple model adapter behavior. Use compatibility defaults and fixture tests.
