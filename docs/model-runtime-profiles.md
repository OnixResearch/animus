# Per-model runtime profiles

r[impl agent_harness_runtime.model_runtime_profiles.verification]

Animus binds each selected provider/model target to one reviewed prompt, skill, memory, and native-tool posture before provider dispatch. The binding is deterministic policy input, not authority: every requested resource is intersected with existing validated objects, active host policy, verified grants, agent/project ceilings, model capabilities, approvals, executor readiness, and named budgets.

## Configuration and identity

`config/model-runtime-profiles.ncl` is the typed source of truth. Runtime code never evaluates Nickel. `scripts/config/export-nickel-configs` exports `config/generated/model-runtime-profiles.json`; `scripts/config/check-nickel-configs` and `scripts/cairn/check-model-runtime-profiles --root .` reject drift.

A profile set declares:

- exact provider/model bindings with an optional exact router role, plus at most one explicitly named default;
- trusted/admitted host or explicitly admitted project provenance;
- prompt-mode and prompt-source references;
- metadata-only, selectable, or required-selected skill references;
- an explicit disabled, read-only, or read-write posture bound to an existing opt-in memory-policy BLAKE3 identity;
- direct tool and toolset references;
- an explicit `disabled`, `optional`, or `capability_required` native tool-calling mode;
- finite named context, tool-call, and parallelism budget policies; and
- policy, model-capability, and canonical BLAKE3 profile identities.

Nickel rejects malformed schemas, duplicate IDs or bindings, dangling budget/toolset references, contradictory disabled-tool profiles, unsafe labels, invalid defaults, and unadmitted project profiles. Rust revalidates the exported DTOs and recomputes each canonical profile identity, so a well-shaped but stale hash still fails closed.

A resumable coding-session may include that admitted model-profile identity as one component of its separate [session runtime authority profile](session-runtime-authority-profile.md). The session profile records only the identity and cannot load, select, authorize, or widen a model profile.

## Resolution

`resolve_model_runtime_profile` uses exactly three specificity tiers:

1. exact provider/model plus router role;
2. exact provider/model without role refinement; and
3. one explicitly named default profile.

Required mode returns typed errors for missing, ambiguous, stale, untrusted, or unadmitted profiles. It never uses wildcard matching, source order, filesystem order, provider defaults, a previous model's profile, or implicit compatibility fallback. `compatibility_disabled` is an explicit host policy and returns no inferred profile.

## Pure composition

The [request-context contribution reducer](request-context-contributions.md) runs below this composition path. It binds one selected contribution plan to the exact immutable model plan and provider request. A fallback or profile change requires full recomposition.

`compose_model_invocation_plan` consumes only in-memory validated values. It reuses:

- `ProviderModelCapabilityMatrix` and `ModelCapabilityNegotiationDecision`;
- `PromptSource`, `PromptAssemblyPolicy`, and `assemble_prompt_sources`;
- validated skill manifests and bodies produced by progressive disclosure;
- an existing validated `OptInMemoryPolicy`; and
- the executor-bound `tool_executor_kit::ToolCatalog` plus explicit readiness observations.

`ModelResourceCeiling` values model independent host-policy, grant, agent-definition, project, and later semantic-role ceilings. A target, prompt mode, prompt source, skill, memory policy, tool, and tool capability is effective only when every supplied ceiling contains it. Profile configuration can narrow that intersection; it cannot grant model, memory, filesystem, process, network, Steel, workflow, tool, or executor authority.

Mandatory host/policy prompt sources remain included even when a profile does not name them. A mandatory or profile-required source that is missing, denied, conflicting, truncated, or over its named context budget blocks composition. Optional resources produce bounded omission diagnostics.

Metadata-only skills never carry full bodies. Selectable and required-selected bodies must already match the trusted manifest and existing invocation, dependency, and budget checks, and their selection must bind the exact target and profile identities. A fallback target cannot inherit another model's selected body or active skill state.

Memory-disabled mode carries no active policy object. Read-only and read-write modes require an exact existing validated `OptInMemoryPolicy`, matching profile and ceiling identities, and independently enabled memory-use controls. Read-write additionally requires existing generation controls; read-only always narrows generation off even when the supplied policy permits it. Missing, disabled, stale, or ceiling-denied policies block composition, and a fallback cannot inherit another target's memory plan.

Disabled tool mode emits no declarations. Optional mode emits only the admitted native intersection. Capability-required mode needs model support and a non-empty admitted intersection. Required tools fail when catalog trust/status, exact capabilities, approval, schema, executor readiness, or budget policy is absent. Prompt-emulated calls are never an alternative.

## Semantic role composition

`animus_runtime::agent_roles::compose_agent_role_model_plan` reuses this composition path and adds one semantic role as another independent ceiling. It also requires separate host admission for the role's output contract, named finite budget, and workspace scopes. A role cannot replace the selected model profile, borrow a prior target's prompts/skills/memory/tools, or convert required resources into grants.

Role-bound native tool calls pass `validate_model_tool_call_binding` first and are then bound to the immutable role plan and effective typed tool capability by `bind_agent_role_tool_action` before executor admission. Role or model fallback returns an explicit recomposition decision; it does not downgrade the role or retain stale role-only resources. See [capability-bound semantic agent roles](agent-role-profiles.md).

## Dispatch, fallback, and returned calls

`bind_model_dispatch_plan` requires both existing provider-neutral dispatch admission and a composed invocation plan in required mode. Router fallback returns `RecomposeForTarget`; the host must resolve and compose the new target from current explicit inputs. [Provider retry](provider-retry-execution.md) may reuse only an exact immutable provider/model/request/profile/prompt/tool/format/reasoning/stream/credential/policy snapshot before semantic stream commitment. Resume and replay remain separate contracts. Any target, profile, prompt, skill, memory, capability, tool catalog, tool plan, policy, or plan identity change requires explicit recomposition.

`validate_model_tool_call_binding` checks every returned native call against the exact advertised tool and all invocation-plan identities before the existing tool executor kit performs final argument, capability, approval, mutation, timeout, output, and executor admission.

## Evidence boundary

`project_model_invocation_receipt` records bounded provider/model labels, router role, specificity, profile and policy identities, prompt/skill/memory/tool plan hashes, memory policy and access metadata, safe skill names and body hashes, tool names and schema identities, omission classes, a redaction marker, and a BLAKE3 receipt identity.

Receipt candidates containing raw prompts, hidden directives, skill bodies, scripts/assets, memory bodies or generated candidates, tool arguments/output, provider payloads, credentials, environment values, private/full paths, endpoints, hidden reasoning, or unbounded traces are rejected.

Passing profile checks prove typed source shape, canonical profile identity, deterministic resolution/composition including existing memory-policy intersection, exercised isolation, and metadata-only receipt projection for the supplied fixtures. They do not prove provider behavior, model quality, tool safety, executor correctness, authorization completeness, verifier soundness, live-host readiness, or whole-system correctness. Authoring tooling retains Nickel evaluation; hosts retain generated-file loading, provider calls, credentials, filesystem/process/network effects, clocks, persistence, executor dispatch, and receipt storage.

## Focused verification

```sh
scripts/config/export-nickel-configs
scripts/config/check-nickel-configs
scripts/cairn/check-model-runtime-profiles --root .
cargo test -p animus-runtime --locked modelprofiles
cargo test -p animus-std --locked --features model-runtime-profiles modelprofiles
nix build path:$PWD#checks.x86_64-linux.model-runtime-profiles --no-link -L
```
