# Request-context contributions

Animus uses one pure reducer for request-local prompt context. Existing owners admit source data before the reducer runs. The reducer does not load files, discover skills, call MCP servers, read memory, contact providers, or persist payloads.

## Public boundary

The public API is `animus_runtime::request_context`.

- `ContributionDraft` carries bounded owner-supplied metadata.
- `validate_contribution` creates one canonical `Contribution`.
- `reduce_required_contributions` checks caller-declared mandatory identities.
- `reduce_contributions` applies supersession, deduplication, conflicts, priority, and budget rules.
- `binding_from_model_invocation` binds the reducer below an existing `ModelInvocationPlan`.
- `validate_selected_payloads` checks bodies only at final prompt assembly.
- `advance_contribution_lifetime` expires one exact finite lifetime.
- `SkillBodyService` is an injected host boundary for exact skill-body loading.
- `project_contribution_receipt` emits metadata-only evidence.

The compatibility aliases `bind_skill_provider_request`, `SkillBodyLoadRequest`, and `SkillProviderRequestObservation` preserve the first reviewed public names.

## Producer and authority inventory

| Source class | Existing owner | Admitted input | Typical lifetime | Authority retained by owner |
| --- | --- | --- | --- | --- |
| Prompt | Model-profile prompt composition | Validated prompt section and payload identity | `CurrentRequest` | Prompt policy, trust, required posture, and source loading |
| Skill | Skill manifest and model-profile skill composition | Selected manifest, exact body identity, and safe label | `UntilNextProviderResponse` | Manifest trust, selection, body loading, assets, scripts, and promotion |
| Memory | Existing memory policy and host-loaded result | Policy-admitted memory identity and bounded context | `CurrentRequest` or `CurrentLogicalTurn` | Memory truth, retrieval, generation, storage, and deletion |
| Project context | Context-reference owners and injected resolvers | Admitted reference result and payload identity | `CurrentRequest` | Root authority, file access, path policy, and content loading |
| MCP | Caller-owned MCP admission | Already-admitted instruction or result identity | `CurrentRequest` | Server discovery, transport, trust, credentials, tool calls, and instructions |
| Runtime | Runtime policy and typed host observations | Bounded notices, recovery context, or policy facts | Any explicit finite lifetime | Runtime transition authority, observation, and host effects |

There is no concrete MCP client in this change. The MCP row records the neutral contribution class only.

## Deterministic reduction

The reducer uses this order:

1. Validate contribution metadata and BLAKE3 payload identities.
2. Reject duplicate contribution identities.
3. Check caller-declared mandatory identities.
4. Apply exact supersession.
5. Collapse deduplication groups.
6. Resolve conflict groups.
7. Reserve budget for mandatory contributions.
8. Select advisory contributions by priority and canonical identity.
9. Bind the result to the model target, profile, prompt plan, model-invocation plan, policy, authority decision, and provider request.

Input enumeration order does not select winners. An advisory contribution cannot displace a mandatory contribution. A missing, conflicting, unsafely omitted, or over-budget mandatory contribution fails before provider dispatch.

## Skill logical-turn flow

1. `begin_skill_turn` validates the session, turn, target, profile, manifest, body, selection, and contribution identities.
2. The host calls only its injected `SkillBodyService`.
3. `admit_loaded_skill_body` verifies the exact BLAKE3 body identity.
4. The resulting mandatory contribution has the `UntilNextProviderResponse` lifetime.
5. `bind_skill_provider` admits the first exact provider request or a later same-request retry.
6. `observe_skill_provider_response` accepts only the correlated terminal response.
7. `admit_skill_turn_completion` blocks completion before the terminal response.
8. `expire_unconsumed_skill_turn` permits an explicit pre-dispatch failure path.

Body loading is not successful skill use. A later provider response must consume the exact selected contribution.

## Retention and receipts

`admit_payload_retention` rejects conversation, compaction, memory, quality, and receipt sinks. A payload hash identifies content. It does not grant retention authority.

Receipts contain identities, source and trust classes, posture, priority, budget, deduplication, supersession, conflict, lifetime, sensitivity, omission classes, safe labels, BLAKE3 payload identities, plan identity, claims, and non-claims. They reject prompt bodies, skill bodies, MCP instructions, memory records, project paths, credentials, provider data, and hidden directives.

## Autolith reference boundary

The reviewed reference is Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871`, under the ISC license. Animus adapted these scenarios:

- deterministic priority and conflict selection;
- explicit supersession and deduplication;
- edge-triggered request-local context;
- a selected skill that affects the next provider response.

Animus did not copy or adopt Autolith's prompt store, S-expression configuration, Common Lisp architecture, saved images, Bash supervisor, global mutable process state, or security claims. Animus has no runtime dependency on Autolith.

## Validation

Run:

```sh
cargo test -p animus-runtime request_context --locked
cargo test -p animus-runtime modelprofiles --locked
cargo clippy -p animus-runtime --all-targets --locked -- -D warnings
scripts/cairn/check-request-context-contributions --root .
scripts/cairn/check-runtime-facade-inventory --root .
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-evidence-safety --root .
scripts/cairn/check-wasm-surface --root .
nix run .#cairn -- validate --root .
```

The focused Rust tests contain positive and negative fixture families for all source classes, order changes, mandatory presence, budget failure, supersession, deduplication, conflicts, all three lifetimes, model fallback drift, stale skill bodies, retries, response barriers, retention, and receipt leaks.

## Non-claims

This boundary does not prove prompt correctness, hidden-context confidentiality, memory truth, MCP server trust, provider delivery, model compliance, skill quality, task completion, production readiness, or release eligibility.
