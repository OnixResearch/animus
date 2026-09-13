# Design: Host capability-scoped Wasm component plugins

## Context

Animus has three useful boundaries already. Plugin manifests describe claims without execution. The backend planner can choose an embedded component. Runtime authority seals bind one dispatch before effects. The new host must compose these boundaries instead of replacing them.

## Success Contract

Given an exact admitted descriptor, bundle, backend plan, runtime seal, resource profile, and input, Animus runs one import-free component method under named bounds. It revalidates returned output and effect intents before existing tool stages can perform an effect.

## Decisions

### Keep descriptor parsing runtime-neutral

Existing plugin-manifest validation remains data-only. A new component-plugin contract consumes a validated descriptor plus separately observed bundle, WIT, runtime, authority, and resource facts.

The descriptor never grants loading or dispatch authority. Unknown, missing, stale, duplicate, or mismatched facts produce typed blockers.

### Add a focused no-std contract family

A separate `animus-adapter-wasm-component` family owns component identity, world and method identity, bundle reference, runtime profile, input and output envelopes, effect-intent envelope, limits, admission decisions, normalized outcomes, and metadata-only receipts.

The family performs no file, process, network, clock, credential, provider, runtime, terminal, persistence, or product effect. It supports `no_std + alloc` and the existing compile rail.

### Keep the host separate and non-default

A separate `animus-std-wasm-component-host` crate owns the shared runtime adapter, Mantle-verifier shell, cancellation, cleanup, and bounded execution. It is absent from default members and default feature graphs.

The host cannot discover plugins, registries, stores, credentials, or authority. Callers inject exact descriptors, bundle roots, current observations, and services.

### Use an import-free first world

The first WIT world accepts one bounded request envelope and returns one bounded result envelope. It exposes no WASI or host imports.

The result may contain validated output plus typed effect intents. Effect intents are proposals only. Existing Animus tool-execution stages bind, admit, commit, execute, and observe any later effect.

### Bind backend plan and runtime seal

Admission requires an embedded-component backend plan with exact adapter, application-host binding, runtime profile, bundle, WIT, resource, and enforcement identities. It also requires one current runtime-authority seal for the same dispatch.

The host rechecks those identities immediately before instantiation. Narrowing, widening, stale observations, or unavailable runtime state block dispatch.

### Deny fallback

A component denial, trap, timeout, cancellation, invalid output, or cleanup failure stays on the component attempt. Animus does not retry through stdio, host process, Steel, native code, or another backend.

A caller must submit a new explicit backend plan and authority seal for another route.

### Keep receipts safe

Receipts bind safe identities, normalized outcome, resource observations, effect-intent counts, cleanup status, and non-claims. They omit component payloads, prompts, responses, credentials, host paths, environment values, raw engine errors, and tool bodies.

## Functional Core and Imperative Shell

- **Core**: descriptor-to-runtime admission, identity construction, effect-intent validation, outcome normalization, fallback denial, and receipt payloads.
- **Shell**: root and file access, bundle remeasurement, Wasmtime execution, deadlines, cancellation, teardown, and report persistence.
- **Existing Animus owners**: backend selection, authority seals, tool stages, effect execution, session state, and product evidence.

## Risks and Controls

- Effect intents can bypass tool policy. Returned intents must enter existing stages as untrusted proposals.
- A non-default host can leak into default graphs. Dependency and feature-isolation rails reject that edge.
- Runtime evidence can be overclaimed. Receipts retain strict non-claims.

## Non-Claims

The host does not prove plugin honesty, component correctness, sandbox completeness, effect success, provider behavior, session correctness, production readiness, or release eligibility.
