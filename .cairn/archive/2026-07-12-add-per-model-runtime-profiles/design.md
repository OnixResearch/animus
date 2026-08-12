## Context

Animus has the individual contracts needed for model-specific behavior:

- model capability negotiation describes what a selected provider/model can support;
- prompt assembly orders admitted sources and records budget decisions;
- skill manifests advertise bounded summaries and defer full body loading;
- opt-in memory policies independently gate memory use and generation;
- agent definitions request model, prompt, and tool ceilings without granting authority;
- tool catalogs and the executor kit validate declarations, executors, arguments, capabilities, mutations, output bounds, and receipts; and
- router execution plans select model labels and policy-bound fallback chains.

The missing layer is composition after a concrete model target is selected. Today a host can build that composition, but Animus does not provide one typed profile, pure resolver, or invocation-plan identity that binds the selected model to its effective skills, prompts, memory access, and tools. The new layer must connect existing contracts without absorbing host discovery, provider execution, or tool authority.

## Decisions

### 1. Runtime profiles are separate from model capabilities and agent definitions

**Choice:** Introduce a `ModelRuntimeProfile` concept distinct from provider/model capability metadata and agent-definition manifests.

A capability matrix answers what a model can technically accept. A model runtime profile requests what that model should see for one Animus host. An agent definition describes a task/persona ceiling. The effective invocation plan is the intersection of all three plus host trust, policy, grants, executor availability, and budgets.

**Rationale:** Provider metadata should not accumulate prompt bodies, skill policy, or product tool configuration, and an agent definition must not become a model-specific authority grant.

### 2. Nickel authors the profile schema; hosts own loading

**Choice:** Define a typed Nickel contract for profile sets and validate equivalent Rust DTOs after host loading. Each profile carries a schema identity, profile ID, source scope, trust/admission status, exact provider/model target or explicit default role, optional router-role refinement, prompt-source references, skill references and invocation posture, an exact existing memory-policy identity and access posture, tool/toolset references, an explicit native tool-calling mode, named context/tool budget-policy references, policy identity, and BLAKE3 content identity.

The profile set defines finite named budget policies, or references separately injected host policies by stable identity; profiles do not embed unexplained numeric limits. Missing or duplicate budget-policy identities fail validation. Reusable crates receive loaded typed values only. They do not evaluate Nickel, read profile files, inspect environment variables, discover providers, load skill directories, or enumerate plugins.

**Rationale:** Nickel gives operators a reviewable typed configuration source while preserving the functional-core/imperative-shell boundary.

### 3. Resolution uses fixed specificity and an explicit enablement policy

**Choice:** Add a pure resolver over a selected provider/model target, optional router role, loaded profile bindings, and `ModelProfilePolicy`.

When profile mode is required, resolution order is:

1. exact provider/model plus router role;
2. exact provider/model without a role refinement; and
3. one explicitly named default profile.

Duplicate bindings at any tier, duplicate profile IDs, unknown profile references, stale target or capability identities, unadmitted project profiles, and a missing match without an explicit default return typed denials. There are no wildcard globs, filesystem-order precedence, provider-side defaults, or ambient fallback. Compatibility-disabled mode preserves the current host-composed path and cannot be entered implicitly from required mode.

**Rationale:** Fixed specificity is explainable, deterministic, and safe under multi-model routing.

### 4. Composition is a pure model invocation planning step

**Choice:** Add a pure `compose_model_invocation_plan`-style core over the resolved profile, selected model capability matrix, admitted prompt sources, loaded skill resources and selection decisions, an existing validated opt-in memory policy, trusted host tool catalog, active authority/policy inputs, executor availability, and explicit budgets.

The plan contains the transient assembled model context plus bounded ledgers and identities for the selected target, profile, prompt assembly, skills, effective memory access, tool declarations, capability negotiation, and policy. Provider dispatch occurs only after the plan is admitted. Filesystem loading, Nickel evaluation, provider calls, credential access, executor effects, clocks, persistence, and receipt output remain in thin host shells.

**Rationale:** Every composition decision can be tested with in-memory values before any model or tool effect.

### 5. Per-model memory narrows existing policy

**Choice:** Profiles bind one exact existing opt-in memory-policy BLAKE3 identity and an explicit disabled, read-only, or read-write posture. Disabled mode carries no active policy object. Read-only and read-write require the host to supply the already validated `OptInMemoryPolicy`, every independent authority ceiling to admit its identity, and the existing use controls to be enabled. Read-write additionally requires existing generation controls; read-only narrows generation off even when the supplied policy permits it.

Memory policy identity, effective use/generation bits, and a memory-plan hash bind the invocation, retry, fallback, returned tool calls, and metadata-only receipt. Missing, disabled, stale, or ceiling-denied policy inputs fail before provider dispatch. Profiles never create consent, storage, retrieval, persistence, or generation authority.

**Rationale:** Model-specific memory behavior reuses accepted consent and memory-policy contracts without treating profile selection as authorization or carrying memory state across targets.

### 6. Per-model skills preserve progressive disclosure

**Choice:** A profile filters which trusted `SkillResourceSummary` values may be advertised to the selected model and assigns each reference a typed posture such as metadata-only, selectable, or required-selected. Full skill instructions, references, assets, or scripts enter transient prompt assembly only after existing skill identity, trust, invocation, dependency, and budget checks succeed; no untyped resource category is inferred from a path or body.

A profile is not permission to execute a skill script. Missing required skills deny composition; optional missing or denied skills are omitted with bounded ledger reasons. Skill bodies and active skill-chain state from one model profile are never reused solely because a fallback model shares the same conversation.

**Rationale:** Models can have specialized skills without turning profile selection into code execution or loading every skill body into context.

### 7. Per-model prompts use existing assembly and cannot suppress mandatory policy

**Choice:** Profiles reference host-loaded prompt sources and prompt-mode profiles rather than bypassing `PromptSource` validation. Model-specific sources are assembled through the existing deterministic section, precedence, trust, redaction, and budget rules.

Host and runtime policy may mark sources mandatory. A model profile can add or narrow prompt sources but cannot remove, replace, reorder below, or weaken mandatory host/policy sources. Effective prompt budget is bounded by host policy, profile policy, and the selected model capability budget. A required source that cannot fit or validate denies composition; optional sources follow explicit skip/truncation policy and ledgering.

**Rationale:** Per-model tuning remains visible and bounded without creating a prompt-based policy bypass.

### 8. Per-model tools are an exact policy intersection

**Choice:** Profiles require one explicit provider-neutral native tool-calling mode equivalent to disabled, optional, or capability-required. There is no implicit mode; an explicitly authored compatibility profile uses disabled unless it intentionally opts into native tools. The effective model-visible tool declarations are the deterministic intersection of:

- profile tool and toolset references;
- trusted active host catalog definitions;
- selected model native tool-call capabilities;
- current policy, verified grants, approval posture, and capability ceilings;
- matching schema and executor bindings; and
- finite named tool-call and parallelism budget policies.

Disabled mode emits no declarations and rejects tool calls. Optional mode may emit the non-empty admitted intersection. Capability-required mode denies composition unless native tool calling is supported and at least one requested tool is eligible. It does not claim that a model must produce a tool call. Prompt-emulated or provider-specific raw tool syntax is not accepted as an alternative.

Every returned tool call is revalidated against the exact invocation-plan identity and then passes the existing tool executor kit admission before effects.

**Rationale:** A model sees only tools it can call and the host can safely execute, while profile configuration remains a narrowing layer rather than an authority source.

### 9. Fallback, retry, resume, and replay bind the selected model again

**Choice:** Router fallback to a different model must resolve and compose that target's profile from fresh explicit inputs. It may preserve committed conversation history according to existing ledger policy, but it does not inherit the prior model's skill selection or active skill-chain state, prompt-source set, memory plan, tool declarations, tool budgets, or executor bindings. Continuing the same skill on the fallback target requires that target's profile to admit it and a fresh host selection/binding decision.

A same-target retry after a retriable provider failure reuses the exact immutable invocation plan, including tool configuration and budgets. A profile, policy, capability, prompt-source, skill, or tool-catalog identity change makes that plan stale; runtime rejects silent reuse and requires an explicitly created replacement plan/attempt. Pending tool calls cannot cross to a different target/profile plan.

**Rationale:** Model-specific behavior remains true under the exact transitions most likely to cause accidental privilege or context carry-over.

### 10. Durable evidence is metadata-only

**Choice:** Profile and invocation receipts record safe provider/model labels or hashes, router role, profile ID and BLAKE3 identity, source scope/trust class, prompt ledger hashes and budget classes, advertised/selected skill labels and hashes, effective memory access and policy/plan hashes, tool declaration/schema/catalog hashes, tool-calling mode, capability decision, omission/denial classes, policy identity, and final BLAKE3 receipt identity.

Receipts reject raw prompt, skill, or memory bodies, hidden directives, scripts/assets, tool arguments or output, provider payloads, credentials, environment values, private/full paths, endpoint bodies, hidden reasoning, and unbounded traces.

**Rationale:** Operators can explain which per-model profile was effective without persisting sensitive context.

## Rejected Alternatives

- **Put skills/prompts/tools directly in the provider registry:** this conflates technical provider capability with host behavior and increases credential/config coupling.
- **Let agent definitions own the behavior:** definitions are persona/task ceilings and may be project-supplied; treating them as model authority would violate their trust contract.
- **Select profiles before routing and carry them through fallback:** the profile could describe the wrong model and leak unavailable tools or model-specific instructions.
- **Use wildcard model-name matching:** provider/model names are not a stable authority boundary, and overlapping patterns create hidden precedence.
- **Allow prompt-emulated tool calls for models without native support:** untyped text parsing would bypass canonical tool declarations and exact executor admission.

## Risks / Trade-offs

- Profile composition adds another planning identity and ledger. Keeping it as one pure plan prevents those checks from scattering across provider and tool shells.
- Exact bindings require operators to update profiles when model catalog identities change. This is intentional fail-closed behavior; an explicit default can reduce repetition.
- Different models may receive materially different context, complicating replay comparison. Receipts make that difference explicit rather than implying equivalent runs.
- Required prompt or skill resources can make a model temporarily unavailable under tight context budgets. Optional/required posture must therefore be reviewed per profile.
