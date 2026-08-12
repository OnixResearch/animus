# Runtime composition preludes and capability features

Animus keeps the broad `animus-runtime` facade for compatibility and adds focused domain preludes for new embedder code. The minimal kernel import path is `animus_runtime::kernel`; it re-exports typed runtime input/config, neutral loop fabric, turn state, runtime events/reports, capability registry, model/tool dispatch planning, and injected adapter orchestration without pulling Steel or optional runtime packs.

## Domain preludes

Use the smallest prelude that matches the integration seam:

| Prelude | Use for | Authority posture |
| --- | --- | --- |
| `kernel` | minimal no-default runtime kernel for typed loop input/config, neutral fabric, turn state, runtime events/reports, capability registry, and model/tool dispatch plans | reusable contracts plus injected model/tool dispatch |
| `loop_prelude` | neutral loop turns/decisions, transition helpers, model dispatch plans, runtime reports, and caller-supplied fabrics | reusable contracts plus injected model/tool dispatch |
| `tool_prelude` | model/tool requests, canonical model message/envelope DTOs, adapters, typed adapter-call outcomes, explicit capability descriptors, registries, and fail-closed outcomes | host-injection surface plus reusable canonical DTOs |
| `headless_prelude` | UI-neutral headless commands, events, responses, run requests, and thread records | reusable contracts |
| `provenance_prelude` | safe provenance ledger, receipt, replay, graph, and Toolpath projection DTOs | reusable contracts |
| `worker_prelude` | worker-team delegation DTOs and supplied worker-service calls | host-injection surface |
| `policy_prelude` | lifecycle hooks, middleware, guards, and trust records | host-injection surface |
| `status_prelude` | semantic status observations, selection, evidence summaries, and presentations | reusable contracts |
| `rlm_kernel` | revalidated host execution for the optional persistent model-facing Steel environment | injected host effects behind the `rlm-kernel` feature |

The flat root exports remain compatibility surfaces. Do not remove or silently rename them when adding a prelude; add migration guidance first.

## Neutral loop fabric

`LoopFabric` consumes `RuntimeLoopTurn` and returns `RuntimeLoopDecision`, so custom Rust, replay, or test fabrics do not construct Steel values. The default Steel runner is an adapter behind the `steel` feature, enabled by the default `compat` bundle. The non-default `lean-loop` feature adds a separately compiled Lean 4 process-backed fabric through the same neutral trait; Rust revalidates every candidate, and Steel remains the canonical scriptable fabric. See [the Lean 4 loop fabric](lean4-loop-fabric.md).

The Nickel-governed Steel policy includes `default-loop` for a simple model-then-response fabric and `ralph-loop` for a bounded Ralph-style profile. The separate non-default `rlm-kernel` feature adds `SteelRlmKernel` and `SteelRlmToolAdapter`. This model-facing profile retains successful Steel cells across tool calls and returns typed host requests instead of performing effects. `ralph-loop` repeats model turns until the previous model message is exactly `COMPLETE` after trimming or the runtime turn budget ends. That exact response is only a completion candidate: controller continuation waits for repository-selected passing evidence and a matching Cairn completion receipt when authoritative evidence is required. All PRD/progress loading, fresh sessions, VCS, validation, shell, browser, Cairn invocation, and persistence effects remain host-owned.

Focused checks:

```sh
scripts/cairn/check-runtime-kernel-split --root .
cargo test -p animus-runtime admission::tests --locked
cargo test -p animus-runtime shell::tests --locked
cargo test -p animus-runtime --locked
cargo test -p animus-runtime --features rlm-kernel rlm_kernel --locked
cargo test -p animus-steel --features rlm-kernel rlm --locked
cargo check -p animus-runtime --no-default-features --locked
cargo test -p animus-runtime kernel --no-default-features --locked
```

## Capability registry

Runtime host authority is represented with `RuntimeCapabilityDescriptor` and `RuntimeCapabilityRegistry`. Dispatch resolves the required capability by exact label, service kind, and safe scope; broad service-kind presence is diagnostic only and does not authorize a model or tool call. Missing, denied, unavailable, malformed, cancelled, and timed-out capability outcomes fail closed and must not fall back to provider discovery, filesystem scans, process spawning, network access, or environment reads.

Existing `RuntimeAdapters::new(model, tool)` remains a compatibility wrapper that creates the documented `DEFAULT_MODEL_CAPABILITY_LABEL` and `DEFAULT_TOOL_CAPABILITY_LABEL` descriptors with no scope. Advanced hosts can replace the registry with `RuntimeAdapters::with_capability_registry`; if they do, the registry must still contain the exact required labels/scopes or dispatch fails before adapter calls.

## Model dispatch planning

`plan_model_dispatch` is a pure final gate before the shell calls a model adapter. The plan records the selected capability identity, `animus.model-request.v1` request contract, provider-neutral contract identity, prompt budget, stream policy, receipt policy, cancellation/timeout posture, and deny-by-default fallback policy. Denied plans return typed fail-closed issues and the shell maps them to adapter outcome classes without contacting providers, reading credentials, scanning the filesystem, spawning processes, loading plugins, or trying another provider.

Structured dispatch can also run `negotiate_model_capability` over `CanonicalModelRequest`, `ProviderModelCapabilityMatrix`, and `ModelCapabilityDowngradePolicy` before adapter execution. Negotiation admits supported requests, records explicit optional downgrades, and denies unsupported roles, content parts, tool calls, structured output, streaming, reasoning controls, budget overflow, unknown capability metadata, privacy mismatch, or fallback attempts without probing providers.

Compatibility defaults preserve the simple model adapter path: non-streaming dispatch, safe receipt policy, no cancellation, within deadline, and fallback denied unless a future explicit provider-router policy admits otherwise. Structured model integrations can use `CanonicalModelRequest` and `CanonicalModelResponseEnvelope` from `tool_prelude`; their legacy bridge only projects back to `ModelRequest`/`ModelResponse` when no tools, structured output, refusal/safety/malformed status, reasoning request, usage, provenance, or non-default output controls would be lost.

## Capability families

`runtime_capability_family_map()` records the maintained family-to-feature map for `loop`, `headless`, `provenance`, `batch`, `worker-team`, `policy`, `quality`, `research`, `scaffold`, `compaction`, `dataset-export`, `status`, and `compatibility`. Validate map edits with `validate_runtime_capability_family_map` tests and the inventory rail.

Cargo exposes matching feature labels plus the default `compat` bundle and `all-families`. Selected family preludes are guarded by `compat` or their family feature, while the broad flat facade remains available for compatibility until a planned migration rewires internal facade references. `steel` is an actual optional integration dependency. `.cairn/component-catalog.ncl` records the runtime kernel plus headless, provenance, worker-team, policy, batch, compaction, dataset-export, quality, research, scaffold, status, and orchestration feature-pack entries with owner, dependency, authority, and focused-check metadata.
