# Runtime facade inventory

Animus keeps a maintained inventory of runtime-facing public exports so embedders can distinguish default-safe reusable contracts from host-owned injection seams and shell-only adapters. This adapts Clankers' runtime-facade inventory discipline without adopting Clankers runtime crates, daemon/TUI/product shells, provider router, plugin runtime, auth flows, process manager, or desktop storage.

The source of truth is `cairn/runtime-facade-inventory.ncl`. The focused rail is `scripts/cairn/check-runtime-facade-inventory --root .`.

The high-level component catalog lives in `cairn/component-catalog.ncl` and projects to [component-catalog.md](component-catalog.md). Use the catalog for block-level owner, authority, dependency, feature-gate, and focused-check selection; use this inventory for runtime-facing public export classification.

Each inventory record carries composition metadata: `feature_gate`, `required_authority`, `depends_on`, `composes_with`, and `capability_family`. The checker validates controlled feature/authority/family labels, known dependency targets, known composition links, reusable-to-shell dependency direction, and safe evidence fields.

## Classes

| Class | Meaning | Allowed side effects |
| --- | --- | --- |
| `reusable-contract` | Deterministic DTOs, pure validation helpers, error types, constants, optional bridge envelopes, and no-ambient boundary values. | `none` |
| `host-injection-surface` | Traits or ports where a caller supplies authority. Missing services must fail closed and must not discover ambient backends. | `injected-only` |
| `shell-adapter` | `std`, terminal, process-facing, or feature-gated shell conveniences that may touch host authority only in the owning shell crate. | `shell-owned` |

## Current summary

- `animus-core` exports are reusable contracts: `Intent`, `AnimusState`, `CoreError`, and intent bounds.
- `animus-adapters` request/response DTOs, current-shape tool request contracts, bounded tool response contracts, canonical model contracts, provider projection conformance fixtures, document-ingest DTOs, model-evidence DTOs, optional gateway, node, session-isolation, guardrail, and orchestration DTO validation, public contract identity constants/parser, typed adapter-call outcome envelopes, model-router execution dispatch/receipt DTOs, status-observation DTOs, skill-manifest descriptors, progressive-disclosure summaries, and validation errors are reusable contracts; `ComponentAdapter`, `ComponentOutcomeAdapter`, `ModelAdapter`, `ToolAdapter`, typed outcome adapter traits, `HostStatusAdapter`, orchestration service contracts, and model-router execution service contracts are host-injection surfaces.
- `animus-preserves-schema` owns authority-free schema identities/catalog metadata; `animus-preserves-adapter` is the runtime-free adapter DTO bridge; and `animus-preserves` remains the optional compatibility facade with explicit `adapter`, `runtime`, `provenance`, `policy`, and review-only family modules. These are reusable codec contracts, not host authority or behavioral-correctness claims.
- `animus-steel` exports are optional reusable loop-fabric boundary contracts; Steel decisions still revalidate through adapter DTOs before dispatch.
- `animus-runtime` exports reusable event/report/config DTOs, pure transition helpers, exact capability-registry descriptors, model dispatch planning, pure per-model profile validation/resolution, prompt/skill/existing-memory-policy/tool composition, the `agent_roles` module for capability-bound role validation, role/model composition, non-union capability intersection, separation, immutable transitions, bounded outputs, and metadata-only receipts, pure tool dispatch planning, the `tool_executor_kit` metadata/admission/receipt core plus its explicit injected-service boundary, capability-negotiation DTOs, pure status-authority selection/explanation helpers, typed workflow command manifest/registry/binding/output DTOs, the runtime DAG contract kit for pure graph validation/ordering/receipts, the bounded portfolio-search registry/state/audit/receipt core, the `subagents` facade module for pure child-run validation/coordination reducers, the orchestration adapter-family compatibility bridge, and host-injection dispatch surfaces over caller-supplied fabrics and adapters.
- `animus-std` exports shell adapters plus the pure authority catalog and fail-closed composition planner. It owns explicit-path generated model-profile and semantic-role loading behind feature gates, process-facing runtime loading, shell orchestration, provider registry selection, credential-source diagnostics, document extraction, model-evidence ingestion, optional concrete provider-streaming parser/dispatch seams, compatibility facade re-exports, and the feature-gated packaged headless dogfood smoke runner.
- `animus-std-observability` exports the direct deterministic no-op and in-memory sink implementations. The crate owns no ambient filesystem, process, network, credential, clock, terminal, global subscriber, or persistence authority; richer telemetry backends remain separate host shells.
- `animus-tui` exports are shell adapters because terminal UI authority belongs outside reusable contracts.
- The root `animus` crate mirrors these classes through feature-gated reexports.

## Forbidden reusable-contract authority

A `reusable-contract` inventory entry must not require ambient filesystem, environment, credentials, clocks, randomness, network, provider discovery, TUI state, daemon frames, plugin launch, process execution, or concrete persistence. If behavior needs that authority, it belongs behind a `host-injection-surface` or `shell-adapter`.

## Evidence

The rail validates the Nickel inventory against runtime-facing crate root exports and extended composition metadata, checks positive and negative fixtures, and prints class, feature, authority, family, dependency-link, composition-link counts plus a BLAKE3 inventory hash when `b3sum` is available. Contract identities, model-dispatch plans, per-model profile and invocation plans, semantic role assignments/intersections/separation/plans/outputs, tool-dispatch plans, executor catalog metadata, pure executor admission, and safe receipt validation are recorded as reusable contracts; outcome-returning adapter and executor traits are recorded as injected surfaces. Durable evidence should include only schema/status, checked crates, counts, fixture names, safe diagnostics, and content hashes. It must omit raw prompts, provider payloads, credentials, environment values, full host paths, raw tool output, and unbounded tool bodies.

See [runtime composition preludes](runtime-composition-preludes.md) for domain prelude, capability registry, family feature, and Steel integration guidance. Optional dataspace additions are recorded as Preserves `dataspace_dto` reusable contracts and as `animus-std` `syndicate-host` shell seams; reusable core/runtime/adapter crates remain free of live transport dependencies.
