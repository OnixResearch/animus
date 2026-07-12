## Why

Animus already has typed prompt sources, progressive-disclosure skills, opt-in memory policies, trusted tool manifests, a capability-aware model registry, model-router plans, and a host-injected tool executor kit. Those pieces are not yet composed into one deterministic contract that says which skills, prompt sources, memory access, and callable tools a particular selected model receives.

Without that binding, hosts must assemble model context ad hoc. A router fallback can accidentally inherit the first model's prompt or tool catalog, a model without native tool-call support can be shown unusable tools, and profile configuration can be mistaken for authority. Animus needs an explicit per-model composition plan that is testable without providers or host effects and that remains subordinate to trust, capability, UCAN, confirmation, and executor policy.

## What Changes

- Add a typed, Nickel-authored model runtime profile contract that binds an exact provider/model target, with an optional router-role refinement, to prompt-source references, typed skill-resource posture, an exact existing memory-policy identity and access posture, tool or toolset references, an explicit native tool-calling mode, and finite named context/tool budget-policy references.
- Add deterministic profile resolution with exact target-and-role, exact target, and explicit default tiers; duplicate, stale, untrusted, or missing required bindings fail closed.
- Add a pure model invocation composition plan that intersects profile requests with admitted prompt sources, trusted skill resources, existing opt-in memory policy, model capability metadata, host tool catalogs, active capability policy, available executors, and budget limits.
- Preserve progressive skill disclosure, mandatory host/policy prompt sources, canonical native tool declarations, exact tool-call admission, and safe BLAKE3 profile/plan identities.
- Re-resolve and re-compose profiles for router fallback, retry, resume, and replay boundaries so one model never silently inherits another model's skills, prompts, memory plan, or tools.
- Add positive and negative fixtures, focused lifecycle rails, safe receipts, documentation, and component/conformance inventory updates.

## Impact

- **Surfaces**: `animus-runtime` model dispatch planning, `animus-adapters` prompt/skill/memory/tool DTO composition, `animus-std` host configuration loading, provider capability negotiation, router execution, and tool executor admission.
- **Configuration**: a typed Nickel source and checked export/fixture for model runtime profiles; reusable Rust crates do not execute Nickel or discover profile files.
- **Testing**: distinct model profiles, role refinement, explicit defaults, progressive skill loading, mandatory prompt preservation, exact memory-policy intersection, native tool projection, unsupported tool calling, missing resources/executors, authority escalation, fallback rebinding, stale plan identity, and receipt leak rejection.
- **Compatibility**: per-model profiles are enabled by explicit host policy. Existing hosts may remain in compatibility-disabled mode or provide an explicit compatibility default; enabled mode never falls back to ambient or implicit profile behavior.
- **Non-claims**: this does not add provider clients, credentials, model discovery, plugin execution, prompt-emulated tool parsing, automatic skill script execution, filesystem discovery in reusable crates, or authority beyond existing host policy and verified grants.
