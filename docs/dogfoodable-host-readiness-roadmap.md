# Dogfoodable host readiness roadmap

Animus dogfoodability means evidence that a specific non-default host shell can run a bounded prompt path through accepted Animus contracts. It does not mean Clankers parity, production readiness, or default adoption of Clankers product surfaces.

Current status: roadmap defined, claim-hygiene fixtures added, fixture-backed provider stream parsers and model-router execution contracts exist, the reusable runtime includes exact per-model profile resolution/composition with fallback isolation, bounded same-plan provider retry contracts with injected timer/dispatch seams, durable active-turn acceptance and crash-recovery contracts, capability-bound semantic role composition with explicit separation and non-authoritative outputs, and a coherent metadata-only tool executor kit with fake-backed execution and safe receipts. The non-default `animus-dogfood-smoke` still proves the original fake-backed headless prompt rung. The separate non-default `animus-coding-session-smoke` now proves a bounded durable fake-provider session over an explicit local store, with a canonical authority binding before the first provider boundary, current fake-host enforcement observations, restart drift admission, commit-before-effect ordering, ordered shared events, and metadata-only receipts. The non-default `animus-codebase-tool-smoke` now exercises one concrete read-only syntactic query through exact request binding and an explicit capability root. No packaged smoke exercises provider retry or the live OpenAI host. Live-provider evidence, daemon/attach, plugin runtime, TUI dogfood, global state, Matrix/P2P, OAuth, release tagging, broader persistent-session behavior, and Clankers-compatible behavior remain explicit non-claims.

## Gap inventory

Classifications are intentionally precise:

- **completed evidence rung**: implemented and covered by deterministic smoke evidence for the current bounded claim;
- **actionable blocker**: required before the next bounded dogfood host can make a stronger claim;
- **intentional non-claim**: useful surface, but not part of the current dogfood claim;
- **rejected default product surface**: forbidden as reusable Animus default behavior by [Clankers product-surface rejection](clankers-product-surface-rejection.md);
- **optional future host adapter**: allowed only behind an explicit host-owned, non-default successor Cairn.

| Compared surface | Classification | Current blocker or reason | Successor/evidence gate |
| --- | --- | --- | --- |
| Runnable CLI/headless entrypoint | completed evidence rung | `animus-dogfood-smoke` proves the original fake-backed prompt path; `animus-coding-session-smoke`, gated by `coding-session-host`, proves the separate explicit-store durable fake-backed session rung. | Keep both packaged smoke rails green; stronger claims require separately indexed evidence. |
| Provider execution | completed bounded host-adapter rung | The non-default `animus-std-openai` crate now supplies pure text-only OpenAI Responses admission and one optional bounded HTTPS client. No live smoke result is indexed. | Keep `scripts/cairn/check-live-openai-provider-host --root .` green. A live-provider claim requires a separately indexed explicit smoke result. |
| Per-model profile composition | completed contract rung | Typed Nickel/Rust profiles, exact resolution, prompt/skill/memory/tool authority intersections, transition isolation, and metadata-only receipts pass deterministic fixtures. No packaged or live host currently loads these profiles for provider dispatch. | Keep `scripts/cairn/check-model-runtime-profiles --root .` green; a host-shell successor must bind its loader and dispatch path before claiming exercised model-specific behavior. |
| Provider retry execution | completed contract rung | Typed Nickel/Rust policy, safe classification, exact-plan and semantic-stream gates, injected fake timer/dispatch seams, provider-neutral events, and metadata-only receipts pass deterministic fixtures. No packaged or live provider path exercises retry. | Keep `scripts/cairn/check-provider-retry --root .` green; stronger behavior claims require a separately indexed host smoke. |
| Native evaluation host | completed contract rung | The non-default `animus-std-eval` public entrypoint composes checked native plans with deterministic fake task, model, tool, agent, verifier, artifact, timer, cancellation, cleanup, and result services. It is not a packaged benchmark or live-provider smoke. | Keep `scripts/cairn/check-std-evaluation-host --root .` green. Any live benchmark claim needs separate evidence and policy. |
| Native evaluation composition conformance | completed contract rung | The non-default suite runs 11 paired mechanism families through public core, adapter, host, compatibility, downstream, and receipt APIs. It uses deterministic fakes and records metadata-only evidence. | Keep `scripts/cairn/check-evaluation-composition-conformance --root .` green. Live, benchmark, training, production, and release claims need separate evidence. |
| Capability-bound semantic roles | completed contract rung | Typed Nickel/Rust implementer, reviewer, verifier, judge, coordinator, and observer profiles; exact authority intersections; explicit separation; recomposition; and metadata-only outputs pass deterministic fixtures. No packaged host currently activates these roles, and role output is not authoritative completion. | Keep `scripts/cairn/check-agent-role-profiles --root .` green; a host successor must bind activation/output capabilities and the separate completion gate before claiming exercised role behavior. |
| Tool execution | completed bounded host-adapter rung | The non-default codebase query executor binds exact request values, reads only through an explicit capability-root graph, returns bounded model-visible output, and emits metadata-only receipts. The packaged smoke proves one deterministic syntactic query. | Keep `scripts/cairn/check-codebase-tool-host --root .` and executor-kit composition checks green. Mutation, shell, network, provider-directed tool calls, and product tool suites remain unclaimed. |
| Session persistence | completed bounded evidence rung | `animus-runtime::session_persistence` retains the safe-ledger seam; the non-default durable store supplies capability-rooted synchronized commits and restart recovery; canonical authority profiles block drift and require fresh host observations; active-turn recovery gates provider and steering work on durable transaction evidence; and `animus-coding-session-smoke` composes that store with fake services through exact continuation gates. | Keep `scripts/cairn/check-durable-session-store --root .`, `scripts/cairn/check-active-turn-recovery --root .`, `scripts/cairn/check-coding-session-host --root .`, and `scripts/cairn/check-session-authority-profile --root .` green. Broader, live, global, replicated, encrypted, or production persistence remains unclaimed. |
| Receipts and evidence | completed evidence rung | The packaged smoke emits safe line-oriented receipt metadata with hashes, event counts, redaction status, and omitted-surface non-claims. | Current-head release evidence index remains separate if broader packaged evidence is claimed. |
| Documentation and examples | completed evidence rung | Dogfood docs name the packaged headless smoke rung and non-claims. | Examples must continue to name the highest evidence rung and run claim-hygiene validation. |
| Readiness harness | actionable blocker | No single rail yet ties dogfood readiness claims to indexed evidence. | `scripts/cairn/check-dogfood-readiness --root .` gates fixture-level claim hygiene. |
| TUI | optional future host adapter and rejected default product surface | Ratatui host contracts exist, but terminal dogfood is not default readiness. | Optional TUI Cairn after headless shell, concrete adapters, and TUI-specific evidence. |
| Daemon/attach | optional future host adapter and rejected default product surface | Long-lived daemon ownership is outside reusable crates. | Optional daemon Cairn with admission, persistence, cancellation, and attach receipts. |
| Local ACP editor adapter | completed bounded contract rung | The non-default `animus-acp-stdio-host` pins the reviewed stable cohort, owns bounded local stdio, delegates to one coding-session host, and passes deterministic fixture-client evidence. No live editor smoke or arbitrary-editor compatibility is claimed. | Keep `scripts/cairn/check-acp-stdio-host --root .` green; record any live editor smoke separately. |
| Plugin/MCP runtime | optional future host adapter and rejected default product surface | Manifests and MCP mapping are contracts; process/network transports remain host-owned. | Plugin/MCP Cairn after manifest, transport, permission, and receipt prerequisites. |
| Provider router/OAuth | rejected default product surface | Account flows and provider discovery are host product behavior. | Optional provider-shell Cairn only; no reusable default dependency. |
| Matrix/P2P coordination | rejected default product surface | Network coordination is not required for dogfoodability. | Separate host adapter, if ever, with remote admission and safe transport receipts. |
| Global session store | rejected default product surface | Host-global `.clankers`-style state is incompatible with reusable defaults. | Host persistence Cairn must choose an explicit store descriptor and safe receipt model. |
| Self-evolution apply/rollback | rejected default product surface | Runtime self-mutation is not required for a bounded prompt path. | Separate policy-heavy Cairn; not part of first dogfood. |
| Release tagging | rejected default product surface | Tagging workflow ownership is outside Animus runtime readiness. | Release automation Cairn only after evidence index semantics are accepted. |

## First non-default host-shell target

The first dogfood target is a non-default headless/CLI host shell. It is owned by the host shell, not by reusable core crates. It uses deterministic fake adapters before live providers and proves that the request lifecycle can run without ambient credentials, provider discovery, network access, process execution, daemon ownership, global session storage, plugin loading, router/OAuth flows, Matrix/P2P coordination, terminal UI ownership, self-evolution, or release tagging.

That target is now represented by the `animus-dogfood-smoke` binary behind the non-default `animus-std/dogfood-smoke` feature. The target remains feature-gated so `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, `animus-runtime`, and default `animus-std` contracts do not require product behavior.

## Minimum dogfood prompt path

A minimum dogfood smoke covers this path:

1. Parse typed prompt input into a headless run request with objective, identity, policy profile, output contract, and noninteractive behavior.
2. Admit the request through accepted policy/capability boundaries before any side effect.
3. When per-model profiles are required, resolve the selected target and compose its mandatory prompts, progressive skills, exact existing memory-policy intersection, and exact native-tool intersection from host-loaded values.
4. When semantic roles are required, activate an exact host-authorized role assignment, add its resource ceiling, intersect every capability source, and evaluate explicit separation before role-specific output; do not treat that output as completion.
5. Dispatch a model request through an injected deterministic fake model adapter.
6. Admit tool metadata from a safe catalog without shipping product tools as defaults.
7. Emit ordered runtime events for prompt assembly, model dispatch, optional tool admission, completion status, and final output selection.
8. Produce safe receipt metadata with status class, event counts, adapter labels, redaction markers, and BLAKE3 identities where content identity matters.
9. Exit without reading credentials, opening the network, spawning a process, starting a daemon, writing a global store, or persisting a session by ambient default.

## Operational readiness matrix

| Surface | Minimum prerequisites | Successor Cairn expectation | Evidence required before claim |
| --- | --- | --- | --- |
| Fake-backed headless/CLI | `agent_harness_runtime.claude_code_headless.request`, `agent_harness_runtime.claude_code_headless.events`, `agent_harness_runtime.clankers_injected_services.contracts`, `agent_harness_runtime.clankers_safe_receipts.dto` | Maintain the non-default `animus-dogfood-smoke` host-shell smoke with deterministic fake adapters. | deterministic fixture checks, fake-backed integration, and packaged headless smoke. |
| Capability-bound multi-role execution | Exact semantic role activation/output capabilities, model-profile composition, separation observations, immutable subject and plan bindings, bounded role receipts, and the separate authoritative-completion gate. | Add a host-owned coordinator that activates roles without ambient grants or context inheritance. | deterministic role fixtures plus a successor packaged smoke that proves candidate/review/verification/judgment separation and completion non-equivalence. |
| Live provider execution | Injected provider boundary, canonical model contracts, provider registry hardening, fixture-backed stream parsers, concrete dispatch plans, and the non-default text-only OpenAI host. | Maintain the exact endpoint and explicit credential boundary. Add new cohorts only through successor Cairns. | optional live-provider smoke plus non-claim entries for omitted providers and unsupported request fields. |
| Built-in or packaged tools | Tool definition metadata, exact request-value binding, argument validation, failure classes, mutation serialization, permission/authorization receipts, explicit executor-kit bindings, the read-only codebase query host, and safe executor receipts. | Maintain the selected non-default read-only executor. Add any mutation, shell, or network pack only through a separate host-owned Cairn. | executor-kit fixtures and the packaged codebase-tool smoke; broader tool suites require separate evidence. |
| Session persistence | Provenance ledger DTOs, neutral replay, host-injected descriptors, pure replay/restore planning, safe receipt redaction, and the accepted non-default durable local host. | Maintain the non-default coding-session composition; never add a global store default. Any live-provider or concrete-tool successor must bind its own authority profile. | focused durable-store rail, coding-session-host rail, child-process restart, fake/file conformance, corruption/lock/sync/leak rejection, and packaged composition evidence. |
| TUI dogfood | Ratatui boundary, UI-neutral host bridge, pure render model, failure isolation, headless/runtime event contracts. | Add optional TUI shell after headless path and concrete adapters. | optional TUI dogfood rung; otherwise TUI remains a non-claim. |
| Daemon/attach | Remote/headless admission, cancellation, session identity, persistence, safe transport receipts. | Add optional daemon host adapter; reusable runtime stays daemon-free. | daemon-specific optional smoke and current-head evidence index. |
| Plugin/MCP runtime | Plugin/tool manifests, MCP mapping, host-owned transport, permission checks, bounded receipts. | Add optional plugin/MCP host adapter; no auto-discovery or auto-trust. | plugin/MCP optional smoke and manifest/runtime-neutral rails. |
| Documentation/examples | Product-surface rejection note, this roadmap, focused verification rails, release evidence index semantics. | Examples must name evidence rung and non-claims. | claim-hygiene rail plus matching focused rails. |
| Release evidence | Release evidence index schema, focused rails, workspace checks, Cairn validation/gates. | Add an evidence assembler only after receipts are collected. | current-head release evidence index; no release tagging claim by default. |

## Successor Cairn ordering

Operational successor Cairns must not claim readiness from implementation code alone. A successor that proposes live providers, built-in tools, persistence, TUI, daemon/attach, plugin/MCP runtime, or release harness behavior must name prerequisite accepted requirement IDs or explicit waivers with replacement requirement IDs.

The order is:

1. reusable runtime and event contracts;
2. injected service boundaries for model, provider, tool, host effects, policy, confirmation, cancellation, and runtime policy;
3. safe receipt and provenance/ledger requirements;
4. policy and permission requirements;
5. concrete host adapters and tool/provider shells;
6. non-default headless/CLI dogfood shell;
7. optional TUI, daemon/attach, plugin/MCP, live-provider, persistence, and release-evidence surfaces.

`cairn/roadmap.ncl` remains the machine-checked sequencing source. Planned future changes may be present there before they are active, but every active change must be classified exactly once and product-surface entries must retain a transitive path through foundational and runtime safety prerequisites.

## Evidence ladder

Readiness claims must name the highest completed rung and must not imply higher rungs:

| Rung | What it proves | What it does not prove |
| --- | --- | --- |
| deterministic fixture checks | Pure validators, claim hygiene, and static boundary fixtures pass. | No executable host shell. |
| fake-backed integration | A deterministic host path runs through fake model/tool services and safe events. | No live provider quality, credentials, network, persistence, TUI, daemon, or plugin behavior. |
| packaged headless smoke | A packaged non-default CLI/headless shell runs the fake-backed prompt path. The separate codebase-tool smoke runs one explicit read-only syntactic query. | No live-provider tool use, mutation, broader product tools, TUI, daemon, plugin, or production claim. |
| durable fake-backed coding session | The non-default coding-session shell commits a metadata-only authority binding and admitted messages to an explicit local store, re-observes fake-host enforcement before dependent fake-provider effects, and passes bounded restart evidence. | No live provider, concrete product tool, TUI dogfood, daemon, plugin, OAuth, global store, filesystem/power-loss guarantee, or production claim. |
| bounded local ACP fixture client | The optional ACP stdio bridge drives the production codec/transport and shared coding-session seams with deterministic positive and adversarial fixtures. | No arbitrary-editor interoperability, remote transport, MCP hosting, plugin runtime, or production claim. |
| optional live-provider smoke | A selected live provider path passed in an explicitly configured host environment. | No general provider parity or default credential discovery. |
| optional TUI dogfood | A selected TUI host path passed bounded dogfood checks. | No daemon, plugin runtime, or production claim. |
| current-head release evidence index | Indexed current-head evidence ties required pure checks and selected optional checks to the recorded head. | No omitted optional surface; omitted modes remain non-claims. |

## Claim-hygiene rule

Docs, evidence indexes, examples, and release notes must not claim Clankers parity, production readiness, live-provider support, daemon behavior, plugin runtime, persistent-session behavior beyond the explicit durable fake-backed rung, or TUI dogfood unless the matching evidence class is indexed. If evidence is absent, the surface must be listed as a non-claim with the next-best validation.

The focused rail is:

```sh
scripts/cairn/check-dogfood-readiness --root .
nix build .#checks.x86_64-linux.dogfood-readiness --no-link -L
```

It validates this roadmap contains the required inventory, matrix, successor-order, evidence-ladder, and claim-hygiene anchors, then runs positive and negative claim-index fixtures under `cairn/dogfood-readiness-fixtures/`.
