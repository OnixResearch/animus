# Session runtime authority profiles

Animus binds every resumable coding-session lineage to one canonical, metadata-only `SessionRuntimeAuthorityProfile`. The binding makes authority-relevant drift visible across restart without copying policy bodies, secrets, prompts, tool schemas, extension bodies, endpoint or environment values, or physical paths into the durable journal.

Profile equality is continuity evidence only. It is not authorization, sandbox enforcement, capability availability, provider or tool readiness, credential admission, replay equivalence, release eligibility, or whole-session correctness.

## Ownership map

The profile composes safe identities from existing owners; it does not duplicate their policy models.

| Component class | Existing owner | Profile contribution |
| --- | --- | --- |
| `policy_authorization` | caller-owned authorization/Basalt integration and coding-session policy service | Identity of the separately admitted policy/authorization posture. |
| `permission` | permission and command-admission policy | Identity of the admitted permission mode. |
| `sandbox_intent` | sandbox descriptors and optional Sandlock host | Requested sandbox identity, not a capability observation. |
| `model_runtime` | model runtime profiles and provider selection | Identity of the selected admitted runtime profile. |
| `tool_catalog` | tool catalog and executor kit | Identity of the effective catalog. |
| `capability_ceiling` | tool and role capability policy | Identity of the effective ceiling. |
| `hook_plugin_set` | hook/plugin manifest owners | Identity of the admitted set, never extension bodies. |
| `agent_definition_set` | agent-role/profile owners | Identity of the admitted definition set. |
| `context_retrieval` | context/retrieval policy and resolvers | Identity of the admitted policy, never retrieved content or paths. |
| `adapter_cohort` | host adapter catalog | Identity of the selected compatible cohort. |

Durable framing remains owned by `durable_session_store`; coding-session ordering and resume remain owned by `coding_session_host`; concrete probes and effects remain in injected `animus-std` services.

## Pure profile and binding core

`animus_runtime::session_authority_profile` validates exact required-class coverage, rejects duplicate or malformed identities, canonicalizes class order, and computes domain-separated BLAKE3 identities for profiles, bindings, realized-enforcement observations, and receipts. Identity construction does not use clocks, randomness, filesystem order, locale, process state, or ambient host state.

The Nickel source of truth is `config/session-authority-profile.ncl`. It exports a deliberately shuffled metadata-only fixture to `config/generated/session-authority-profile.json`; a Rust test rebuilds the canonical profile and checks the configured identity exactly. Runtime code never evaluates Nickel.

## Durable ordering and resume

The first admitted prompt stages one transaction whose first record is an `AuthorityProfileBinding` and whose second record is the user message. Provider projection excludes the binding body. No provider or tool effect is requested until the synchronized transaction succeeds.

Every accepted profile change appends a successor binding linked to the prior head and prior profile identity. Existing bindings are never rewritten. The facade exposes pure abandon, fork, and rebind planning; the imperative shell must obtain separate authorization and persist the selected successor.

Resume decodes the latest binding before repair or continuation. Exact component and aggregate identity equality may advance to current host checks. Any widening, narrowing, substitution, malformed input, missing component, or legacy unbound history blocks automatic continuation and reports bounded changed-component classes.

## Requested intent versus realized enforcement

`CodingSessionAuthorityService` is an explicit host injection. The std host invokes it afresh on resume and before provider, tool, or summarizer dispatch. The resulting `SessionAuthorityEnforcementObservation` has an identity distinct from the requested profile and must be current, admitted, and bound to that profile.

A host implementation remains responsible for current authorization, capability probes, adapter compatibility, provider and tool readiness, credential admission, and sandbox preflight. The packaged smoke service is deterministic fake evidence only; it is not a production Sandlock or capability probe.

## Safe evidence

Authority events and receipts contain bounded status and changed-component classes plus safe session, branch, head, profile, binding, authorization, and enforcement-observation identities. Coding-session receipts include only the committed profile and binding identities. Sensitive drafts are rejected rather than redacted after serialization.

## ACP continuity boundary

The optional [ACP stdio host](acp-stdio-host.md) copies only the safe profile identity into its session binding and metadata-only receipt. On load it invokes the existing coding-session `resume` path, so exact durable profile continuity and a fresh `CodingSessionAuthorityService` observation are required before repair, continuation, provider, summarizer, transcript, or tool effects. Client permission approval is additive and cannot widen this profile or substitute for current enforcement.

## Verification

Run from the repository root:

```sh
scripts/cairn/check-session-authority-profile --root .
nix build .#checks.x86_64-linux.session-authority-profile --no-link
cargo test -p animus-runtime sessionauthority::tests --locked
cargo test -p animus-runtime sessionhost::tests --locked
cargo test -p animus-std --features coding-session-host coding_session::tests --locked
```

The focused receipt is `target/cairn/focused-rails/session-authority-profile.md`. Its claims are bounded to the checked source, fixtures, fake services, and local durable-store scenarios.
