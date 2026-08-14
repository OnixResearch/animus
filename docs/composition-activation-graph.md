# Composition activation graph

Animus validates each maintained product composition before it starts external effects. The pure graph core is `animus_session_core::composition_activation_graph`. `animus-runtime` re-exports this module.

<!-- r[impl agent_harness_runtime.composition_activation_graph.catalog] -->
## Catalog source

`.cairn/component-catalog.ncl` is the source for components, application-owned capability seams, and event routes. Each port records its owner, providers, consumers, scope, multiplicity, lifecycle owner, activation posture, feature gate, authority class, and focused check.

Each event records its producer, consumers, delivery class, durability class, lifecycle owner, activation posture, cycle policy, and focused check. The catalog rejects unknown references, invalid multiplicity, authority mismatch, missing lifecycle ownership, unsupported cycles, and unsafe evidence labels.

<!-- r[impl agent_harness_runtime.composition_activation_graph.projections] -->
`docs/component-catalog.md` contains the generated component, dependency, capability-seam, and event-ownership maps. `scripts/cairn/check-component-catalog --root .` rejects documentation drift and missing product-profile bindings.

<!-- r[impl agent_harness_runtime.composition_activation_graph.preflight] -->
## Pure preflight

A composition root supplies a selected component set and current readiness observations. The functional core validates generations, features, authorities, required providers, consumers, event routes, and readiness.

The result has one of these states:

- `ready`: all selected required and optional components are current and ready.
- `degraded_optional`: one or more optional components are unavailable, but all required components are ready.
- `blocked`: a required component, seam, event route, feature, authority, or readiness fact is invalid.

The core does not read configuration, open stores, discover services, or execute effects. It does not use ambient fallback providers.

<!-- r[impl agent_harness_runtime.composition_activation_graph.fail_loud] -->
## Product integration

`animus-live-steel-agent` checks its maintained profile before credential, store, provider, or tool effects. A blocked profile returns `LiveApplicationError::Activation`.

`animus-live-terminal` checks its profile before it calls the connection or terminal host ports. A blocked profile returns `TerminalHostError::Activation`.

<!-- r[impl agent_harness_runtime.composition_activation_graph.inspection] -->
## Safe inspection

An activation decision projects to a bounded metadata-only snapshot. The snapshot contains the graph identity, selected generation, decision status, safe component labels, issue classes, counts, and non-claims.

The projection excludes service handles, configuration bodies, endpoints, credentials, private paths, prompts, payloads, and mutation commands. The Host Control mapping fixture uses only these safe fields. It does not add a runtime dependency on an unpublished Host Control package.

## Reviewed source

The design review used DeepSeek Harness commit `47f943859bef60e4160492346772ded9b24f765a`. The reviewed source has an MIT license and developer-preview status.

Animus adapted its explicit capability-seam, producer-to-consumer event map, and fail-loud boot concepts. Animus did not adopt Cordis contexts, ambient dependency injection, or plugin-owned policy authority.

## Baseline evidence

The pre-change component-catalog and request-lifecycle rails passed. The runtime-facade baseline reported 18 existing unclassified Models.dev exports. The composition baseline also exposed an existing feature-closure error for `animus_adapter_rlm_kernel`.

The final inventory classifies the Models.dev exports under their real feature. The runtime now keeps its required model-profile contract dependency active. The runtime-facade, injected-service, composition-conformance, and evidence-safety rails pass.

The repo-wide pinned format check still reports existing drift in untouched live Steel files. All Rust files changed by this work pass direct `rustfmt`, focused Clippy, workspace checks, and workspace Clippy.

<!-- r[impl agent_harness_runtime.composition_activation_graph.verification] -->
## Verification

Run the focused rail:

```sh
scripts/cairn/check-composition-activation-graph --root .
```

The local rail covers positive and negative graph cases, required and optional preflight, bounded event cycles, stale readiness, startup ordering, event routes, safe inspection, ambient-fallback denial, no-std Wasm compilation, and generated catalog drift.

The Nix check runs the sandbox-safe catalog, projection, fixture, and product-profile binding subset. Existing live-host Nix rails and the local activation rail own compiled product evidence.

The rail proves only the checked graph, preflight, integration, and metadata projection behavior. It does not prove implementation correctness, runtime liveness, event delivery, effect success, authority, production readiness, or release eligibility.
