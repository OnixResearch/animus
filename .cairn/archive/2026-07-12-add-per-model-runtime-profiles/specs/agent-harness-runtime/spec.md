# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Typed model runtime profile contract

r[agent_harness_runtime.model_runtime_profiles.contract] Animus MUST define a versioned model runtime profile contract, authored in typed Nickel and revalidated as host-loaded Rust data, that identifies an exact provider/model target or explicit default, optional router-role refinement, prompt-source references, typed skill-resource posture, exact existing memory-policy identity and explicit access posture, tool or toolset references, an explicit native tool-calling mode, named finite context/tool budget-policy references, source scope, trust/admission status, policy identity, and BLAKE3 profile identity.

#### Scenario: Valid profile configuration exports deterministically

r[agent_harness_runtime.model_runtime_profiles.contract.nickel]
- GIVEN a Nickel profile set declares exact model bindings, an explicit default, bounded resource references, an explicit tool-calling mode, named finite budget policies and references, trust metadata, and policy identity
- WHEN Nickel validation, deterministic export, and Rust DTO validation run
- THEN every accepted profile MUST preserve the same typed fields and BLAKE3 content identity
- AND malformed fields, duplicate IDs or bindings, an omitted tool-calling mode, unknown references, missing or duplicate budget-policy identities, unsafe labels, and drifted exports MUST fail before profile resolution.

### Requirement: Deterministic profile resolution

r[agent_harness_runtime.model_runtime_profiles.resolution] Required per-model profile mode MUST resolve profiles through a pure deterministic function using exact provider/model-plus-role, exact provider/model, and one explicitly named default tier in that order, and MUST return typed denials for ambiguous, duplicate, stale, untrusted, or missing bindings without wildcard, filesystem-order, provider-side, or ambient fallback.

#### Scenario: Role refinement wins deterministically

r[agent_harness_runtime.model_runtime_profiles.resolution.role_refinement]
- GIVEN a selected provider/model has both a model-only binding and a binding for its current router role
- WHEN profile resolution runs in required mode
- THEN it MUST select the role-refined profile and record the specificity tier
- AND it MUST NOT merge an unrelated profile selected by source order or model-name pattern.

#### Scenario: Missing required binding fails closed

r[agent_harness_runtime.model_runtime_profiles.resolution.missing]
- GIVEN required mode has no exact binding and no explicit default for the selected target
- WHEN profile resolution runs
- THEN it MUST return a typed missing-profile denial
- AND it MUST NOT reuse the previous model's profile, invent a provider default, or enter compatibility-disabled mode.

### Requirement: Pure model invocation composition

r[agent_harness_runtime.model_runtime_profiles.composition] Animus MUST compose an admitted model invocation plan as a pure deterministic function over the resolved profile, selected model capability matrix, admitted prompt sources, loaded skill resources and selection decisions, an existing validated opt-in memory policy when enabled, trusted tool catalog, active policy and verified grants, executor availability, and explicit budgets before provider or tool effects occur.

#### Scenario: Profile requests remain authority ceilings

r[agent_harness_runtime.model_runtime_profiles.composition.ceilings]
- GIVEN a trusted or project profile requests a skill, prompt source, tool, model behavior, memory scope, filesystem ability, process ability, network ability, Steel ability, or workflow ability outside active policy or verified grants
- WHEN invocation composition runs
- THEN the request MUST be denied or omitted according to its required/optional posture with a bounded diagnostic
- AND the profile MUST NOT expand effective authority, discover a fallback service, or trigger a host effect.

### Requirement: Per-model memory narrows existing policy

r[agent_harness_runtime.model_runtime_profiles.memory] A model runtime profile MUST bind an explicit disabled, read-only, or read-write posture to one exact existing validated opt-in memory-policy identity, MUST intersect that identity with every independent authority ceiling, and MUST NOT create consent, retrieval, generation, storage, persistence, or adapter authority.

#### Scenario: Existing memory policy remains authoritative

r[agent_harness_runtime.model_runtime_profiles.memory.existing_policy]
- GIVEN a profile requests read-only or read-write memory access
- WHEN invocation composition runs
- THEN the host MUST supply an existing validated policy with the exact profile identity, all ceilings MUST admit it, and existing memory-use controls MUST be enabled
- AND read-write MUST additionally require existing generation controls while read-only MUST narrow generation off.

#### Scenario: Missing or stale memory policy fails closed

r[agent_harness_runtime.model_runtime_profiles.memory.denied]
- GIVEN a profile requests memory access but the existing policy is missing, disabled, stale, malformed, or denied by any authority ceiling
- WHEN invocation composition runs
- THEN composition MUST return a typed memory denial before provider dispatch
- AND it MUST NOT discover a memory backend, inherit another target's memory plan, persist a candidate, or widen existing controls.

### Requirement: Per-model skills preserve progressive disclosure

r[agent_harness_runtime.model_runtime_profiles.skills] A model runtime profile MUST limit advertised and selectable skills for its selected model through typed metadata-only, selectable, or required-selected posture, and full skill instructions, references, assets, or scripts MUST enter transient model context only after existing identity, trust, invocation, dependency, and budget checks admit the selected resource.

#### Scenario: Different models receive different admitted skills

r[agent_harness_runtime.model_runtime_profiles.skills.distinct]
- GIVEN two selected models resolve to profiles with different admitted skill references
- WHEN each model invocation plan is composed
- THEN each plan MUST advertise only its own bounded skill summaries and include only its separately admitted selected skill bodies
- AND neither plan MUST contain the other profile's omitted skill body, references, assets, or script material.

#### Scenario: Required skill cannot bypass trust

r[agent_harness_runtime.model_runtime_profiles.skills.required_denied]
- GIVEN a profile marks a skill required but the skill is missing, untrusted, identity-mismatched, dependency-invalid, over budget, or not admitted for invocation
- WHEN invocation composition runs
- THEN composition MUST return a typed required-skill denial
- AND it MUST NOT load the body, execute a script, search ambient skill stores, or silently substitute another skill.

### Requirement: Per-model prompts preserve mandatory policy and budgets

r[agent_harness_runtime.model_runtime_profiles.prompts] Model-specific prompt-source and prompt-mode references MUST pass existing source validation, deterministic precedence, trust/admission, redaction, mandatory-source, and budget rules, and a profile MUST NOT remove, replace, weaken, or reorder below mandatory host or runtime policy sources.

#### Scenario: Model prompt plan is tailored and policy-complete

r[agent_harness_runtime.model_runtime_profiles.prompts.distinct]
- GIVEN two profiles reference different admitted model-specific prompt sources and the host supplies the same mandatory policy sources
- WHEN prompt assembly runs for each selected model
- THEN each invocation plan MUST contain its own admitted model-specific sections plus the mandatory host/policy sections in deterministic order
- AND its ledger MUST identify included, omitted, truncated, conflicting, or denied sources without persisting raw bodies.

#### Scenario: Mandatory prompt suppression fails closed

r[agent_harness_runtime.model_runtime_profiles.prompts.mandatory]
- GIVEN a profile omits, conflicts with, replaces, weakens, or exhausts the budget needed for a mandatory host or policy prompt source
- WHEN prompt composition runs
- THEN composition MUST return a typed mandatory-source or budget denial
- AND it MUST NOT dispatch a reduced prompt, reorder the mandatory source, or hide the conflict from the ledger.

### Requirement: Per-model native tool calling is exact and policy-bound

r[agent_harness_runtime.model_runtime_profiles.tools] Animus MUST derive each model's visible canonical tool declarations as the deterministic intersection of profile tool/toolset references, trusted active host catalog definitions, selected model native tool-call capability, active policy and verified grants, approval posture, matching schema and executor bindings, and named call/parallelism budgets.

#### Scenario: Tool-calling modes shape model-visible declarations

r[agent_harness_runtime.model_runtime_profiles.tools.modes]
- GIVEN one profile disables tool calling, one enables optional native tool calling, and one requires native tool-call capability with at least one eligible declaration
- WHEN invocation plans are composed for compatible selected models
- THEN disabled mode MUST emit no declarations, optional mode MUST emit only its admitted intersection, and capability-required mode MUST emit a non-empty admitted intersection
- AND none of the modes MUST grant a tool, executor, mutation, or capability absent from stronger policy.

#### Scenario: Unsupported or unavailable tool calling fails safely

r[agent_harness_runtime.model_runtime_profiles.tools.unsupported]
- GIVEN a profile requires native tool-call capability or a required tool whose model capability, catalog definition, schema, trust state, grant, approval posture, executor binding, or budget is absent or denied
- WHEN invocation composition runs
- THEN composition MUST return a typed tool-capability or required-tool denial before provider dispatch
- AND it MUST NOT use prompt-emulated tool syntax, provider-specific raw calls, a shell fallback, an ambient plugin, or an undeclared executor.

### Requirement: Dispatch and fallback bind exact invocation-plan identity

r[agent_harness_runtime.model_runtime_profiles.dispatch_binding] Model dispatch, returned tool calls, router fallback, retry, resume, and replay MUST bind the selected provider/model target, router role, profile identity, prompt plan identity, skill plan identity, memory plan identity, capability decision, tool catalog identity, and policy identity, and a change to any bound identity MUST require explicit replanning before further effects.

#### Scenario: Fallback model receives a fresh profile

r[agent_harness_runtime.model_runtime_profiles.dispatch_binding.fallback]
- GIVEN model A fails and the materialized router fallback chain authorizes model B
- WHEN runtime prepares model B dispatch
- THEN it MUST resolve and compose model B's profile from explicit current inputs and require a fresh selection decision for any continued skill chain
- AND it MUST NOT inherit model A's skill selections or active skill-chain state, prompt-source set, memory plan, tool declarations, budgets, pending tool calls, executor bindings, or profile identity.

#### Scenario: Same-target retry pins the immutable plan

r[agent_harness_runtime.model_runtime_profiles.dispatch_binding.retry]
- GIVEN a provider attempt fails with a policy-authorized retriable outcome and the same target remains selected
- WHEN runtime prepares the retry
- THEN it MUST reuse the exact immutable invocation-plan identity, including prompt, skill, memory, tool, capability, policy, and budget bindings, or create an explicitly identified replacement plan
- AND it MUST reject silent reuse when any bound identity has changed.

#### Scenario: Unadvertised or stale tool call is rejected

r[agent_harness_runtime.model_runtime_profiles.dispatch_binding.tool_call]
- GIVEN a model returns a tool call for a tool absent from its invocation plan or bound to a stale or different profile, catalog, target, or policy identity
- WHEN runtime prepares tool executor admission
- THEN it MUST reject the call with a typed plan-binding diagnostic
- AND no tool, plugin, shell, network, filesystem, process, or provider fallback effect may occur.

### Requirement: Model profile receipts are safe evidence

r[agent_harness_runtime.model_runtime_profiles.receipts] Profile-resolution and invocation-plan receipts MUST contain only bounded safe labels or hashes, specificity and trust classes, profile/policy/capability identities, prompt ledger hashes and budget classes, skill labels and content hashes, memory access/policy/plan hashes, tool/schema/catalog hashes, native tool-calling mode, omission or denial classes, redaction markers, and BLAKE3 receipt identity.

#### Scenario: Receipt rejects raw model context and effect material

r[agent_harness_runtime.model_runtime_profiles.receipts.rejects_leaks]
- GIVEN a receipt candidate contains raw prompt, skill, or memory bodies, hidden directives, scripts or assets, tool arguments or output, provider payloads, credentials, environment values, private or full paths, endpoint bodies, hidden reasoning, or unbounded traces
- WHEN receipt validation runs
- THEN validation MUST reject the receipt with a deterministic leak diagnostic
- AND no durable evidence may claim an admitted or completed invocation from that unsafe receipt.

### Requirement: Per-model profile verification covers composition and isolation

r[agent_harness_runtime.model_runtime_profiles.verification] Per-model runtime profile work MUST include focused positive and negative tests for Nickel/Rust contract parity, deterministic resolution, authority ceilings, progressive skills, existing memory-policy intersection, mandatory prompt assembly, native tool projection, exact executor admission, distinct model plans, fallback/retry/resume/replay isolation, stale identities, safe receipts, documentation, component/conformance inventories, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers per-model behavior

r[agent_harness_runtime.model_runtime_profiles.verification.closeout]
- GIVEN maintainers intend to sync and archive the per-model runtime profile change
- WHEN closeout validation runs
- THEN evidence MUST name focused profile, resolver, skill, memory, prompt, tool, dispatch-binding, fallback-isolation, receipt, Nickel, workspace, conformance, facade, Cairn validation, and proposal/design/tasks gate checks
- AND every unavailable or skipped rail MUST record a concrete blocker and the next-best validation without widening implementation or readiness claims.
