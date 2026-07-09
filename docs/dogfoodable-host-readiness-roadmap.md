# Dogfoodable host readiness roadmap

Animus dogfoodability means evidence that a specific non-default host shell can run a bounded prompt path through accepted Animus contracts. It does not mean Clankers parity, production readiness, or default adoption of Clankers product surfaces.

Current status: roadmap defined, claim-hygiene fixtures added, fixture-backed provider stream parsers and model-router execution contracts exist, and no executable dogfood shell is claimed yet. Live providers, daemon/attach, plugin runtime, persistent sessions, TUI dogfood, global state, Matrix/P2P, OAuth, release tagging, and Clankers-compatible behavior remain explicit non-claims until successor Cairns add host-owned implementations and evidence.

## Gap inventory

Classifications are intentionally precise:

- **actionable blocker**: required before the first bounded dogfood host can make a claim;
- **intentional non-claim**: useful surface, but not part of the current dogfood claim;
- **rejected default product surface**: forbidden as reusable Animus default behavior by [Clankers product-surface rejection](clankers-product-surface-rejection.md);
- **optional future host adapter**: allowed only behind an explicit host-owned, non-default successor Cairn.

| Compared surface | Classification | Current blocker or reason | Successor/evidence gate |
| --- | --- | --- | --- |
| Runnable CLI/headless entrypoint | actionable blocker | No maintained non-default shell proves a prompt can run end to end. | Successor host-shell Cairn with fake-backed smoke and safe receipt evidence. |
| Provider execution | actionable blocker | Reusable crates have injected-service contracts, model-router execution envelopes, provider registry metadata, and fixture-backed stream parsers, but no live-provider host shell claim is indexed. | Provider adapter Cairn after `agent_harness_runtime.clankers_injected_services.provider_boundary`, provider registry contracts, concrete dispatch gates, and safe provider receipts; live-provider smoke is optional evidence. |
| Tool execution | actionable blocker | Tool DTOs and metadata exist, but default product tools are intentionally absent. | Tool host/executor Cairn after `agent_harness_runtime.pi_tool_contracts.definition_metadata` and fail-closed tool dispatch evidence. |
| Session persistence | actionable blocker | Neutral ledger/provenance contracts exist, but durable host storage is not claimed. | Host persistence Cairn after provenance ledger requirements and safe receipt redaction rails. |
| Receipts and evidence | actionable blocker | Safe receipt DTOs exist; dogfood receipts need a bounded shell-specific index. | Fake-backed integration receipt first, then current-head release evidence index if packaged evidence is claimed. |
| Documentation and examples | actionable blocker | Users need examples that show what is runnable and what remains a non-claim. | Example Cairn must name the highest evidence rung and run claim-hygiene validation. |
| Readiness harness | actionable blocker | No single rail yet ties dogfood readiness claims to indexed evidence. | `scripts/cairn/check-dogfood-readiness --root .` gates fixture-level claim hygiene. |
| TUI | optional future host adapter and rejected default product surface | Ratatui host contracts exist, but terminal dogfood is not default readiness. | Optional TUI Cairn after headless shell, concrete adapters, and TUI-specific evidence. |
| Daemon/attach | optional future host adapter and rejected default product surface | Long-lived daemon ownership is outside reusable crates. | Optional daemon Cairn with admission, persistence, cancellation, and attach receipts. |
| Plugin/MCP runtime | optional future host adapter and rejected default product surface | Manifests and MCP mapping are contracts; process/network transports remain host-owned. | Plugin/MCP Cairn after manifest, transport, permission, and receipt prerequisites. |
| Provider router/OAuth | rejected default product surface | Account flows and provider discovery are host product behavior. | Optional provider-shell Cairn only; no reusable default dependency. |
| Matrix/P2P coordination | rejected default product surface | Network coordination is not required for dogfoodability. | Separate host adapter, if ever, with remote admission and safe transport receipts. |
| Global session store | rejected default product surface | Host-global `.clankers`-style state is incompatible with reusable defaults. | Host persistence Cairn must choose an explicit store descriptor and safe receipt model. |
| Self-evolution apply/rollback | rejected default product surface | Runtime self-mutation is not required for a bounded prompt path. | Separate policy-heavy Cairn; not part of first dogfood. |
| Release tagging | rejected default product surface | Tagging workflow ownership is outside Animus runtime readiness. | Release automation Cairn only after evidence index semantics are accepted. |

## First non-default host-shell target

The first dogfood target is a non-default headless/CLI host shell. It is owned by the host shell, not by reusable core crates. It uses deterministic fake adapters before live providers and proves that the request lifecycle can run without ambient credentials, provider discovery, network access, process execution, daemon ownership, global session storage, plugin loading, router/OAuth flows, Matrix/P2P coordination, terminal UI ownership, self-evolution, or release tagging.

The target must remain feature-gated or separately packaged so `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, `animus-runtime`, and default `animus-std` contracts do not require product behavior.

## Minimum dogfood prompt path

A minimum dogfood smoke covers this path:

1. Parse typed prompt input into a headless run request with objective, identity, policy profile, output contract, and noninteractive behavior.
2. Admit the request through accepted policy/capability boundaries before any side effect.
3. Dispatch a model request through an injected deterministic fake model adapter.
4. Admit tool metadata from a safe catalog without shipping product tools as defaults.
5. Emit ordered runtime events for prompt assembly, model dispatch, optional tool admission, completion status, and final output selection.
6. Produce safe receipt metadata with status class, event counts, adapter labels, redaction markers, and BLAKE3 identities where content identity matters.
7. Exit without reading credentials, opening the network, spawning a process, starting a daemon, writing a global store, or persisting a session by ambient default.

## Operational readiness matrix

| Surface | Minimum prerequisites | Successor Cairn expectation | Evidence required before claim |
| --- | --- | --- | --- |
| Fake-backed headless/CLI | `agent_harness_runtime.claude_code_headless.request`, `agent_harness_runtime.claude_code_headless.events`, `agent_harness_runtime.clankers_injected_services.contracts`, `agent_harness_runtime.clankers_safe_receipts.dto` | Add a non-default host-shell smoke with deterministic fake adapters. | deterministic fixture checks and fake-backed integration. |
| Live provider execution | Injected provider boundary, canonical model contracts, provider registry hardening, fixture-backed stream parsers, model-router execution envelopes, permission policy, safe provider receipts. | Add a concrete provider host adapter without ambient credentials, ambient model discovery, provider-side fallback, or network fallback. | optional live-provider smoke plus non-claim entries for omitted providers. |
| Built-in or packaged tools | Tool definition metadata, argument validation, failure classes, mutation serialization, permission/authorization receipts. | Add host-owned tool executors or capability packs; reusable crates expose contracts only. | deterministic tool fixtures, fake executor integration, then packaged headless smoke. |
| Session persistence | Provenance ledger DTOs, neutral replay, safe receipt redaction, host-owned storage descriptors. | Add explicit storage adapter; no global session store default. | fake persistence fixture, restore/replay checks, and safe receipt leak rejection. |
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
| packaged headless smoke | A packaged non-default CLI/headless shell runs the fake-backed prompt path. | No live-provider, TUI, daemon, plugin, or production claim. |
| optional live-provider smoke | A selected live provider path passed in an explicitly configured host environment. | No general provider parity or default credential discovery. |
| optional TUI dogfood | A selected TUI host path passed bounded dogfood checks. | No daemon, plugin runtime, or production claim. |
| current-head release evidence index | Indexed current-head evidence ties required pure checks and selected optional checks to the recorded head. | No omitted optional surface; omitted modes remain non-claims. |

## Claim-hygiene rule

Docs, evidence indexes, examples, and release notes must not claim Clankers parity, production readiness, live-provider support, daemon behavior, plugin runtime, persistent-session behavior, or TUI dogfood unless the matching evidence class is indexed. If evidence is absent, the surface must be listed as a non-claim with the next-best validation.

The focused rail is:

```sh
scripts/cairn/check-dogfood-readiness --root .
nix build .#checks.x86_64-linux.dogfood-readiness --no-link -L
```

It validates this roadmap contains the required inventory, matrix, successor-order, evidence-ladder, and claim-hygiene anchors, then runs positive and negative claim-index fixtures under `cairn/dogfood-readiness-fixtures/`.
