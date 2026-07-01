# Agent Harness Runtime Specification

## Purpose

Defines the accepted baseline for Animus as a modular Rust agent-harness framework with explicit crate barriers, typed adapter contracts, a Steel decision fabric, and standard-library host shells.

## Requirements

### Requirement: Crate boundary baseline

r[agent_harness_runtime.baseline.crate_boundaries] The workspace MUST preserve explicit crate boundaries between pure core state, implementation-free adapter contracts, optional Preserves boundary serialization, Steel turn decisions, standard-library host orchestration, optional terminal UI hosting, and the root compatibility facade.

#### Scenario: Embedders choose the smallest surface

r[agent_harness_runtime.baseline.crate_boundaries.smallest_dependency]
- GIVEN an embedder needs only deterministic state logic, adapter DTOs, serialization, Steel scripting, std shell orchestration, or TUI hosting
- WHEN they select workspace crates
- THEN each reusable crate exposes only its owned boundary
- AND concrete host, provider, filesystem, process, network, and UI behavior stays outside unrelated reusable crates.

### Requirement: Functional core and imperative shell split

r[agent_harness_runtime.baseline.functional_core] Non-trivial Animus behavior MUST be decomposed into pure deterministic core logic over explicit inputs and thin imperative shells for filesystem, process, environment, network, terminal, credential, clock, and persistence effects.

#### Scenario: Core logic is testable without standing up the world

r[agent_harness_runtime.baseline.functional_core.testable]
- GIVEN runtime, policy, adapter, prompt, provenance, or serialization logic contains branch decisions
- WHEN maintainers add or change that behavior
- THEN the branch decisions are represented as deterministic functions over in-memory values
- AND tests can exercise positive and negative cases without real providers, shells, terminals, credentials, or storage backends.

### Requirement: Adapter-first integration boundary

r[agent_harness_runtime.baseline.adapters] Model, tool, memory, ledger, queue, policy, confirmation, provider, workspace, and host integrations MUST cross typed adapter or service contracts rather than being constructed by reusable runtime core code.

#### Scenario: Missing host services fail closed

r[agent_harness_runtime.baseline.adapters.missing_service]
- GIVEN reusable runtime logic requires a host capability
- WHEN the corresponding adapter or service is absent, denied, unavailable, malformed, or cancelled
- THEN runtime returns a typed failure before claiming completed effects
- AND it does not discover ambient config, read credentials, spawn commands, or contact providers as a fallback.

### Requirement: Runtime dispatch uses injected services

r[agent_harness_runtime.clankers_injected_services.contracts] Animus runtime dispatch MUST consume injected model/provider, tool, host-effect, capability, cancellation, confirmation, and runtime-policy services through typed contracts rather than constructing concrete clients or reading ambient host state.

#### Scenario: Service contracts are explicit

r[agent_harness_runtime.clankers_injected_services.contracts.explicit]
- GIVEN runtime code needs model, provider, tool, host-effect, confirmation, cancellation, or policy behavior
- WHEN the boundary is reviewed
- THEN the behavior is represented by an injected trait, adapter, or typed service handle
- AND reusable runtime code does not depend on concrete provider clients, credential stores, shell tools, plugin runtimes, or host persistence.

### Requirement: Missing or denied services fail closed

r[agent_harness_runtime.clankers_injected_services.fail_closed] Missing, unavailable, denied, cancelled, timed-out, malformed-output, and completed service outcomes MUST be typed and MUST prevent false success receipts or later side effects when the outcome is not completed.

#### Scenario: Non-completed services block effects

r[agent_harness_runtime.clankers_injected_services.fail_closed.blocks]
- GIVEN a runtime action requires an injected service
- WHEN the service is absent, unavailable, denied, cancelled, timed out, or returns malformed output
- THEN runtime returns the corresponding typed outcome
- AND no receipt or provenance fact claims the effect completed.

### Requirement: Confirmation denies by default

r[agent_harness_runtime.clankers_injected_services.confirmation] Protected actions that require confirmation MUST deny by default when no broker is injected or when the broker reports unavailable, timed-out, or cancelled.

#### Scenario: Missing confirmation broker prevents protected action

r[agent_harness_runtime.clankers_injected_services.confirmation.default_deny]
- GIVEN a protected action requires human or host confirmation
- WHEN no confirmation broker is available
- THEN runtime records a denied decision
- AND the protected side effect is not executed.

### Requirement: Provider boundary forbids ambient clients

r[agent_harness_runtime.clankers_injected_services.provider_boundary] Provider dispatch tests or rails MUST prove reusable runtime code does not construct provider clients, read credentials, discover provider config, or open network connections directly.

#### Scenario: Provider dispatch goes through the injected service

r[agent_harness_runtime.clankers_injected_services.provider_boundary.injected]
- GIVEN a model/provider request is dispatched
- WHEN focused boundary tests run
- THEN the runtime calls only the injected service with a validated adapter-neutral request
- AND ambient credential lookup, provider discovery, network opening, and concrete client construction are absent from reusable runtime code.

### Requirement: Tool and effect boundaries fail before unsafe execution

r[agent_harness_runtime.clankers_injected_services.tool_boundary] Tool/effect dispatch tests MUST cover missing executor, unavailable service, capability denial, cancellation, timeout, and malformed output.

#### Scenario: Tool denial stops the turn safely

r[agent_harness_runtime.clankers_injected_services.tool_boundary.denied]
- GIVEN a loop decision requests a tool or effect denied by capability policy
- WHEN dispatch evaluates the decision
- THEN a typed capability-denied outcome is returned
- AND later model, provider, or tool calls for that denied effect are not executed.

### Requirement: Effect receipts are redacted

r[agent_harness_runtime.clankers_injected_services.receipts] Service and effect receipts MUST record safe outcome classes and MUST omit raw provider bodies, credentials, environment values, full paths, raw tool arguments, and raw tool output.

#### Scenario: Injected-service closeout is evidenced

r[agent_harness_runtime.clankers_injected_services.closeout]
- GIVEN the injected service change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes provider-boundary tests, tool/effect negative tests, receipt redaction tests, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Typed agent definition manifests

r[agent_harness_runtime.clankers_agent_definitions.manifest] Animus agent definitions MUST be represented as typed manifests with name, description, model preference, prompt fragments, capability ceilings, toolset limits, source scope, trust status, and manifest identity before activation.

#### Scenario: Valid user manifest activates as typed data

r[agent_harness_runtime.clankers_agent_definitions.manifest.valid]
- GIVEN a user-scoped agent definition satisfies the typed manifest contract
- WHEN activation validates the definition
- THEN Animus produces a typed activation candidate with safe manifest identity
- AND raw manifest parsing does not grant tool, model, memory, or filesystem authority.

### Requirement: Deterministic discovery and precedence

r[agent_harness_runtime.clankers_agent_definitions.discovery] Agent-definition discovery MUST be deterministic over loaded sources and MUST reject duplicate definition names at the same precedence instead of selecting by filesystem order.

#### Scenario: Duplicate project definitions fail closed

r[agent_harness_runtime.clankers_agent_definitions.discovery.duplicate]
- GIVEN two project-scoped sources define the same agent name
- WHEN discovery resolves available definitions
- THEN discovery returns a duplicate-definition diagnostic
- AND no arbitrary source is selected.

### Requirement: Project definitions require admission

r[agent_harness_runtime.clankers_agent_definitions.project_trust] Project or workspace agent definitions MUST be untrusted by default and MUST require host confirmation or policy admission before they influence prompt assembly, tool access, model selection, memory, or workflow behavior.

#### Scenario: Unadmitted project agent cannot influence prompt

r[agent_harness_runtime.clankers_agent_definitions.project_trust.denied]
- GIVEN a project-scoped agent definition is discovered
- AND no host admission decision trusts that manifest identity
- WHEN a session requests that agent
- THEN Animus denies activation or activates a safe no-influence failure state
- AND the prompt and tool/model permissions are unchanged.

### Requirement: Manifest capabilities are ceilings

r[agent_harness_runtime.clankers_agent_definitions.capability_ceilings] Agent-definition toolsets, model preferences, memory scopes, context-reference scopes, and workflow abilities MUST be interpreted as ceilings or requests and MUST NOT expand effective authority beyond active capability policy and verified grants.

#### Scenario: Manifest escalation rejected

r[agent_harness_runtime.clankers_agent_definitions.capability_ceilings.escalation]
- GIVEN an agent manifest requests a tool, model role, network context reference, or workflow ability that active policy does not authorize
- WHEN activation evaluates capability ceilings
- THEN the unauthorized ability is rejected or omitted according to policy
- AND the receipt records the denied escalation without applying it.

### Requirement: Safe activation receipts

r[agent_harness_runtime.clankers_agent_definitions.receipts] Agent-definition discovery and activation receipts MUST record safe metadata such as definition name, source scope, trust/admission status, capability ceilings, selected model role label, manifest hash, policy hash, and redaction markers, and MUST NOT include raw prompt bodies, full paths, credentials, provider payloads, or hidden memory bodies.

#### Scenario: Activation receipt leak rejected

r[agent_harness_runtime.clankers_agent_definitions.receipts.rejects_leaks]
- GIVEN an activation receipt fixture includes raw prompt text, a full project path, credentials, provider JSON, or hidden memory body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Agent definition trust verification

r[agent_harness_runtime.clankers_agent_definitions.verification] The adaptation MUST include focused positive and negative tests for typed manifests, deterministic discovery, duplicate names, project admission, capability ceiling enforcement, malformed manifests, compatibility-import boundaries, and receipt redaction.

#### Scenario: Closeout evidence covers agent-definition fixtures

r[agent_harness_runtime.clankers_agent_definitions.verification.closeout]
- GIVEN maintainers intend to archive the agent definition trust adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest, discovery, trust, capability, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Bounded Clash adaptation scope

r[agent_harness_runtime.clash_ucan.scope] Animus MUST treat Clash as a bounded reference for selected safety-harness patterns, including capability taxonomy, decision explanations, sandbox descriptors, and scenario tests, and MUST NOT adopt Clash's Starlark policy language, full policy engine, brush shell, agent plugin machinery, editor/LSP surfaces, installer flows, audit schema, or precedence-as-authority model as part of this adaptation.

#### Scenario: Scope review rejects wholesale adoption

r[agent_harness_runtime.clash_ucan.scope.review]
- GIVEN a proposed change imports Clash's Starlark DSL, brush shell, plugin infrastructure, or audit schema into Animus runtime crates
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until only the approved safety-harness patterns remain
- AND the rejection names the Animus-owned alternative boundary.

### Requirement: UCAN-first capability requests

r[agent_harness_runtime.clash_ucan.capability_request] Guarded runtime actions MUST be represented as typed capability requests with contract, resource, and ability fields before policy or dispatch decisions are trusted.

#### Scenario: Runtime action is normalized before authorization

r[agent_harness_runtime.clash_ucan.capability_request.normalized]
- GIVEN a runtime decision requests a model call, tool invocation, filesystem access, process execution, network access, or Steel evaluation
- WHEN the runtime prepares to authorize the action
- THEN it constructs a typed capability request naming the selected contract, resource, and ability
- AND malformed, missing, or empty request fields fail before adapter or host dispatch.

### Requirement: Nickel contracts and UCAN authority gate local allows

r[agent_harness_runtime.clash_ucan.nickel_contracts] Animus capability policy MUST be authored as Nickel-reviewed contracts or fixtures that define stable resource/ability names for guarded model, tool, filesystem, exec, network, and Steel actions.

#### Scenario: Nickel contract export is the policy source

r[agent_harness_runtime.clash_ucan.nickel_contracts.export]
- GIVEN a maintained capability policy contract source
- WHEN the policy export or drift check runs
- THEN generated runtime policy data matches the reviewed Nickel source
- AND unknown abilities, malformed resource prefixes, unsupported action classes, or missing contracts fail before runtime use.

r[agent_harness_runtime.clash_ucan.ucan_authority] A local policy `allow` MUST NOT authorize a guarded action unless the Nickel contract permits the requested resource and ability and the supplied UCAN grant set authorizes the same resource and ability.

#### Scenario: Local allow without UCAN grant denies

r[agent_harness_runtime.clash_ucan.ucan_authority.missing_grant]
- GIVEN local policy contains an allow caveat for a requested tool action
- AND the request lacks a UCAN grant authorizing the same resource and ability
- WHEN authorization is evaluated
- THEN the action is denied with a typed missing-authority reason
- AND no model, provider, tool, process, network, filesystem, or Steel host dispatch occurs.

### Requirement: Local policy layers are caveats

r[agent_harness_runtime.clash_ucan.local_caveats] User, project, session, and harness policy layers MUST be interpreted as local caveats that can narrow, deny, or prompt for a guarded action, and MUST NOT expand effective authority unless the session/request carries a verified UCAN grant for the expanded resource and ability.

#### Scenario: Session override cannot bypass authority

r[agent_harness_runtime.clash_ucan.local_caveats.session_override]
- GIVEN a session policy layer asks to allow a broader filesystem, network, exec, tool, model, or Steel action than lower layers allow
- WHEN no verified UCAN grant authorizes the broader resource and ability
- THEN the broader action is denied despite the session allow
- AND the receipt records local caveat status separately from UCAN authority status.

#### Scenario: Human approval attaches bounded authority

r[agent_harness_runtime.clash_ucan.local_caveats.ask_grant]
- GIVEN a local caveat returns ask for a guarded action
- WHEN the user approval path produces a bounded session UCAN grant for the requested resource and ability
- THEN the runtime may reevaluate the same capability request with that grant
- AND the resulting receipt identifies the session grant without recording raw prompt or tool input data.

### Requirement: Sandbox descriptors are execution constraints

r[agent_harness_runtime.clash_ucan.sandbox_constraints] Sandbox descriptors MAY be attached to authorized actions as execution constraints, but MUST NOT be treated as authorization proof, and runtime dispatch MUST fail closed when a required sandbox descriptor is malformed, denied by policy, or cannot be enforced by the selected shell/platform adapter.

#### Scenario: Required sandbox is unavailable

r[agent_harness_runtime.clash_ucan.sandbox_constraints.unavailable]
- GIVEN a capability request is authorized by Nickel contracts and UCAN grants
- AND the selected local caveat requires a sandbox descriptor
- WHEN the shell/platform adapter reports that the descriptor is malformed or unenforceable
- THEN runtime dispatch denies the action before host execution
- AND the receipt records the sandbox issue without claiming that UCAN authorization failed.

### Requirement: Safe authorization receipts

r[agent_harness_runtime.clash_ucan.receipts] Authorization receipts MUST carry safe metadata only, including status, issue code, contract id, ability label, redacted resource identifier or BLAKE3 resource hash, policy hash, checked-capability evidence, local caveat results, sandbox descriptor hash when present, and BLAKE3 receipt hash, and MUST NOT include raw prompts, provider payloads, credentials, full paths, raw tool bodies, raw model outputs, or unbounded outputs.

#### Scenario: Authorized receipt carries authority evidence

r[agent_harness_runtime.clash_ucan.receipts.authorized]
- GIVEN a guarded action is allowed after Nickel contract checks, UCAN grant checks, local caveat checks, and required sandbox checks
- WHEN the authorization receipt is emitted
- THEN it records the allowed status, contract id, ability, checked UCAN grants, policy hash, caveat summary, optional sandbox descriptor hash, and receipt hash
- AND sensitive resource, prompt, payload, and output material is omitted or represented only by approved hashes/redaction markers.

#### Scenario: Receipt validation rejects leaks

r[agent_harness_runtime.clash_ucan.receipts.rejects_leaks]
- GIVEN a receipt fixture contains a raw prompt, credential-looking field, absolute path dump, provider payload, raw tool body, raw model output, or unbounded output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the receipt cannot be used as review or release evidence.

### Requirement: Focused verification for UCAN-aligned adaptation

r[agent_harness_runtime.clash_ucan.verification] The adaptation MUST include focused positive and negative tests for capability request normalization, Nickel contract validation, UCAN authority checks, local caveat narrowing, session grant behavior, sandbox descriptor enforcement, and receipt redaction.

#### Scenario: Positive and negative fixtures gate closeout

r[agent_harness_runtime.clash_ucan.verification.fixtures]
- GIVEN the UCAN-aligned Clash pattern adaptation is ready for review
- WHEN focused verification runs
- THEN positive fixtures show authorized model/tool actions, local caveat narrowing, bounded session grant authorization, sandbox-constrained dispatch, and redacted receipts
- AND negative fixtures show missing UCAN grants, contract/resource/ability mismatches, local policy denials, malformed or unenforceable sandbox descriptors, and receipt leak attempts fail closed.

r[agent_harness_runtime.clash_ucan.closeout] Closeout evidence MUST include successful focused capability/caveat/sandbox/receipt tests, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates before this adaptation is archived.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.clash_ucan.closeout.evidence]
- GIVEN maintainers intend to archive the UCAN-aligned Clash pattern adaptation
- WHEN they collect closeout evidence
- THEN the evidence names every successful focused rail, workspace check, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Permission modes are bounded UX caveats

r[agent_harness_runtime.claude_code_permissions.scope] Animus MUST treat Claude Code permission modes as a bounded UX reference for allow, ask, and deny outcomes, and MUST NOT treat local permission allows as authority beyond Nickel contract checks and verified UCAN grants.

#### Scenario: Local allow lacks authority

r[agent_harness_runtime.claude_code_permissions.scope.local_allow_denied]
- GIVEN local permission policy says to allow a guarded filesystem, exec, network, model, Steel, or tool action
- AND the requested action lacks matching contract and UCAN authority
- WHEN permission resolution runs
- THEN the action is denied before dispatch
- AND the receipt separates missing authority from local permission status.

### Requirement: Typed permission policy descriptors

r[agent_harness_runtime.claude_code_permissions.policy] Permission policy descriptors MUST map guarded action classes and resource labels to typed allow, ask, or deny outcomes with source identity, trust scope, precedence, and prompt-summary policy.

#### Scenario: Malformed permission policy fails closed

r[agent_harness_runtime.claude_code_permissions.policy.invalid]
- GIVEN a permission descriptor has an unknown action class, missing resource label, unsupported outcome, or missing source identity
- WHEN permission policy validation runs
- THEN validation returns typed diagnostics
- AND the malformed descriptor cannot authorize or prompt for an action.

### Requirement: Pure permission resolution

r[agent_harness_runtime.claude_code_permissions.resolution] Permission resolution MUST be a pure deterministic decision over in-memory action, authority, policy, and prompt-decision inputs that returns execute, prompt, or deny with typed reasons.

#### Scenario: Ask mode pauses before execution

r[agent_harness_runtime.claude_code_permissions.resolution.ask]
- GIVEN authority checks pass and local policy maps the action to ask
- WHEN permission resolution runs without a matching prompt-decision token
- THEN the resolver returns a prompt-required decision
- AND no model, tool, shell, filesystem, or network dispatch occurs.

### Requirement: Host-mediated prompt decisions

r[agent_harness_runtime.claude_code_permissions.prompt] Ask decisions MUST be completed only through an injected host prompt service that receives redacted action summaries and returns scoped prompt-decision tokens before dispatch can continue.

#### Scenario: Stale prompt token is rejected

r[agent_harness_runtime.claude_code_permissions.prompt.stale]
- GIVEN a prompt-decision token was issued for a different action identity, policy hash, session, or resource label
- WHEN permission resolution evaluates the token
- THEN the token is rejected as stale or mismatched
- AND the guarded action remains unexecuted.

### Requirement: Safe permission receipts

r[agent_harness_runtime.claude_code_permissions.receipts] Permission receipts MUST record safe metadata for action class, redacted resource identity or BLAKE3 resource hash, authority status, local outcome, prompt status, policy hash, and decision reasons, and MUST NOT include raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs.

#### Scenario: Denied action emits redacted receipt

r[agent_harness_runtime.claude_code_permissions.receipts.denied]
- GIVEN permission resolution denies a guarded action
- WHEN the runtime emits a permission receipt
- THEN the receipt records denial reason, action class, policy hash, and redacted resource metadata
- AND sensitive request material is omitted or represented only by approved hashes and redaction markers.

### Requirement: Safe receipt DTOs

r[agent_harness_runtime.clankers_safe_receipts.dto] Animus runtime and provenance receipts MUST expose safe metadata only, including schema, status, issue code, labels, bounded counts, BLAKE3 hashes, effect or mutation kind, backend/interchange labels, and explicit redaction markers.

#### Scenario: Positive receipts expose only safe evidence

r[agent_harness_runtime.clankers_safe_receipts.positive]
- GIVEN model, tool, authorization, runtime block, final response, adapter error, or provenance observations contain raw sensitive inputs and outputs
- WHEN safe receipts are constructed
- THEN receipts expose only safe labels, bounded counts, status, issue class, redaction markers, and BLAKE3 hashes
- AND the raw sensitive material is omitted.

### Requirement: Receipt leak rejection

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks] Receipt validation MUST reject or redact raw prompt text, provider JSON, credential-looking values, absolute path dumps, raw tool input, raw tool output, raw model output, and unbounded output before evidence is accepted.

#### Scenario: Leak fixtures fail closed

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks.fixture]
- GIVEN a receipt fixture includes raw prompt text, provider JSON, token-like values, absolute paths, raw tool/model output, or an oversized body
- WHEN receipt validation runs
- THEN validation returns a typed redaction error or redacted output
- AND the unsafe fixture is not accepted as review or release evidence.

### Requirement: BLAKE3 canonical artifact envelopes

r[agent_harness_runtime.clankers_artifact_identity.envelope] Animus-owned artifact identity MUST use BLAKE3 over versioned canonical envelopes containing artifact kind, envelope version, redaction class, normalized payload, and sorted/deduplicated dependency hashes.

#### Scenario: Artifact identity is deterministic

r[agent_harness_runtime.clankers_artifact_identity.deterministic]
- GIVEN equivalent artifact envelopes with dependencies provided in different orders and with duplicates
- WHEN artifact identity is computed
- THEN dependencies are sorted and deduplicated before hashing
- AND the resulting BLAKE3 hash is stable and round-trips through display/parse validation.

### Requirement: Redaction classes govern review output

r[agent_harness_runtime.clankers_artifact_identity.redaction] Artifact redaction classes MUST determine whether payload bodies may appear in receipts, CLI, or review output.

#### Scenario: Non-public artifacts hide payloads

r[agent_harness_runtime.clankers_artifact_identity.redaction.display]
- GIVEN an artifact is metadata-only, redacted-payload, or secret
- WHEN a receipt or review output is rendered
- THEN payload bodies are omitted or replaced with explicit redaction markers
- AND only public artifacts may expose payload bodies by default.

### Requirement: Focused receipt and artifact rails

r[agent_harness_runtime.clankers_safe_receipts.rails] Animus MUST provide focused verification rails for safe receipt validation, leak rejection, artifact hash determinism, and redaction-class display behavior before claiming the Clankers receipt/artifact pattern adopted.

#### Scenario: Receipt and artifact closeout is evidenced

r[agent_harness_runtime.clankers_safe_receipts.closeout]
- GIVEN the safe receipt and artifact identity change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused receipt/artifact rails, positive and negative tests, dependency checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Animus-owned neutral ledger DTOs

r[agent_harness_runtime.clankers_neutral_ledger.dto] Animus MUST define its own neutral session-ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, unsupported entries, replay metadata, and typed replay errors without depending on Clankers session, daemon, TUI, provider, Automerge, or concrete storage types.

#### Scenario: DTOs stay storage-neutral

r[agent_harness_runtime.clankers_neutral_ledger.dto.storage_neutral]
- GIVEN reusable Animus ledger DTOs are reviewed
- WHEN dependency and field boundaries are inspected
- THEN the DTOs do not require concrete storage, filesystem paths, wall-clock construction, daemon events, TUI types, provider-native payloads, or Clankers transcript IDs
- AND host persistence can wrap them without changing pure replay semantics.

### Requirement: Pure deterministic replay

r[agent_harness_runtime.clankers_neutral_ledger.replay] Ledger replay MUST be a pure deterministic function over in-memory entries that returns model-visible context plus safe metadata counts or a typed error.

#### Scenario: Supported entries replay in order

r[agent_harness_runtime.clankers_neutral_ledger.replay_positive]
- GIVEN an in-memory ledger containing user, assistant, tool, summary, usage, and safe receipt entries
- WHEN replay runs
- THEN model-visible user, assistant, tool, and summary context is returned in deterministic order
- AND usage and receipt entries contribute safe metadata counts without leaking raw storage details.

### Requirement: Unsupported entries fail closed

r[agent_harness_runtime.clankers_neutral_ledger.unsupported] Replay MUST fail closed on unsupported or adapter-owned ledger entries rather than silently dropping or exposing them.

#### Scenario: Unsupported payloads do not leak

r[agent_harness_runtime.clankers_neutral_ledger.unsupported.redacted]
- GIVEN replay encounters an unsupported or adapter-owned entry
- WHEN replay is required before a model request
- THEN replay returns a typed unsupported-entry error
- AND the error includes only a safe reason class, not the raw unsupported payload.

### Requirement: Resume requires an injected ledger service

r[agent_harness_runtime.clankers_neutral_ledger.missing_service] Runtime paths that require session resume MUST consume an injected ledger service and MUST fail before adapter dispatch when the service or requested session is unavailable.

#### Scenario: Missing ledger service blocks dispatch

r[agent_harness_runtime.clankers_neutral_ledger.missing_service.blocks]
- GIVEN a runtime request requires resume semantics
- WHEN no ledger service is injected or the requested session is missing
- THEN runtime returns a typed unavailable-service or missing-session failure
- AND no model or tool adapter call is made.

### Requirement: Pure branch query helpers when parent links exist

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries] If Animus ledger entries include parent/head relationships, branch query helpers MUST be pure and MUST detect duplicate IDs, missing parents, cycles, active branch ancestry, and abandoned/dead-end attempts.

#### Scenario: Malformed graphs fail without panic

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries.invalid]
- GIVEN a ledger graph has duplicate IDs, a missing parent, a cycle, or reversed-order assumptions
- WHEN query preparation runs
- THEN it returns a typed graph validation error
- AND no panic, infinite traversal, or silent ancestry corruption occurs.

#### Scenario: Ledger closeout is evidenced

r[agent_harness_runtime.clankers_neutral_ledger.closeout]
- GIVEN the neutral ledger change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused replay tests, unsupported-entry negative tests, missing-service tests, dependency-isolation checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Candidate inventory

r[agent_harness_runtime.trellis_verified_core.candidate_inventory] Animus MUST inventory pure safety-critical decision cores that are candidates for Trellis-style verification, including policy/effect gates, ledger graph validation, branch replay, receipt redaction, schema compatibility, compaction cut points, and roadmap dependency checks when those areas are in scope.

#### Scenario: Only pure cores are candidates

r[agent_harness_runtime.trellis_verified_core.candidate_inventory.pure_only]
- GIVEN maintainers nominate a verification candidate
- WHEN the candidate inventory is reviewed
- THEN the candidate identifies explicit inputs and outputs
- AND filesystem, process, provider, network, credential, terminal, clock, and ambient global effects are excluded from the verified core.

### Requirement: Proof obligations

r[agent_harness_runtime.trellis_verified_core.obligations] Each selected verified-core candidate MUST declare invariants, preconditions, postconditions, rejected-state classes, and equivalence expectations before proof or model code is treated as release evidence.

#### Scenario: Safety property is explicit

r[agent_harness_runtime.trellis_verified_core.obligations.explicit]
- GIVEN a candidate such as graph validation, effect authorization, redaction, or schema compatibility is selected
- WHEN maintainers inspect its verification plan
- THEN the plan states what cannot happen, what must happen, and which invalid inputs are rejected
- AND the proof/model artifact links back to requirement IDs.

### Requirement: Dependency isolation

r[agent_harness_runtime.trellis_verified_core.dependency_isolation] Trellis or verified-helper dependencies MUST be optional, test-only, or isolated in dedicated verified modules until a stable verified-core dependency boundary is accepted.

#### Scenario: Proof dependency does not leak into default runtime

r[agent_harness_runtime.trellis_verified_core.dependency_isolation.default_clean]
- GIVEN default Animus crates are built without verification features
- WHEN dependency checks run
- THEN default reusable runtime/core crates do not require Trellis or proof-tool dependencies
- AND verification-enabled builds remain explicit.

### Requirement: Model or verified logic

r[agent_harness_runtime.trellis_verified_core.model_logic] Animus SHOULD provide model or verified-core implementations for selected pure decision boundaries where proof-oriented hardening materially improves safety claims.

#### Scenario: Model logic handles deterministic input

r[agent_harness_runtime.trellis_verified_core.model_logic.deterministic]
- GIVEN a selected pure decision boundary has modeled inputs
- WHEN model or verified logic evaluates those inputs
- THEN it returns deterministic outputs or typed rejection classes
- AND it does not perform I/O, mutation of external state, logging, or adapter dispatch.

### Requirement: Equivalence tests

r[agent_harness_runtime.trellis_verified_core.equivalence] When model or verified logic and production Rust logic both exist for a decision boundary, Animus MUST include equivalence tests for valid and invalid inputs.

#### Scenario: Divergence is reported

r[agent_harness_runtime.trellis_verified_core.equivalence.divergence]
- GIVEN model/verified logic and production Rust evaluate the same fixture
- WHEN their accepted/rejected status or normalized outputs differ
- THEN the test fails with a diagnostic naming the fixture and boundary
- AND production release evidence cannot claim verified coverage for that boundary.

### Requirement: Negative invariant fixtures

r[agent_harness_runtime.trellis_verified_core.negative_fixtures] Verified-core work MUST include negative fixtures for malformed graphs, authorization bypass attempts, unsafe redaction inputs, unsupported compatibility cases, and invariant-violating counterexamples relevant to the selected candidates.

#### Scenario: Counterexample remains durable

r[agent_harness_runtime.trellis_verified_core.negative_fixtures.counterexample]
- GIVEN a rejected state could break safety claims if accepted
- WHEN negative fixtures run
- THEN the state is rejected with a typed issue code
- AND the fixture can be replayed without live host services.

### Requirement: Verified-core closeout evidence

r[agent_harness_runtime.trellis_verified_core.closeout] Trellis verified-core work MUST document proof scope and run verification/model checks, equivalence tests, negative fixtures, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Verification evidence is reviewable

r[agent_harness_runtime.trellis_verified_core.closeout.evidence]
- GIVEN verified-core work is ready to archive
- WHEN reviewers inspect evidence
- THEN proof scope, model/equivalence checks, negative fixtures, dependency isolation, workspace checks, and Cairn gates are named with status
- AND unsupported proof tooling is recorded as unavailable rather than silently skipped.

### Requirement: Optional dataspace adapter contract

r[agent_harness_runtime.syndicate_dataspace.adapter_contract] Animus MAY provide optional dataspace adapter contracts for publishing bounded runtime assertions, subscribing to safe query patterns, and receiving host orchestration events, but reusable runtime/core crates MUST NOT depend on Syndicate crates or live dataspace transports by default.

#### Scenario: Runtime uses injected dataspace service

r[agent_harness_runtime.syndicate_dataspace.adapter_contract.injected]
- GIVEN runtime wants to publish or observe dataspace state
- WHEN a dataspace backend is configured
- THEN runtime communicates through an injected typed adapter contract
- AND no default runtime/core crate constructs a Syndicate backend or network transport.

### Requirement: Redacted assertion projection

r[agent_harness_runtime.syndicate_dataspace.assertion_projection] Dataspace projections MUST map runtime events, tool states, background work, session/provenance summaries, and agent/subagent lifecycle states into bounded redacted assertion shapes.

#### Scenario: Raw sensitive material is not asserted

r[agent_harness_runtime.syndicate_dataspace.assertion_projection.redacted]
- GIVEN an event or state contains prompt text, provider payloads, credentials, full paths, file bodies, tool input/output, model output, or unbounded logs
- WHEN it is projected to a dataspace assertion
- THEN the assertion includes only approved redaction markers, hashes, counts, stable IDs, and safe status classes
- AND raw sensitive material is omitted by default.

### Requirement: Explicit scoping and lifecycle

r[agent_harness_runtime.syndicate_dataspace.scoping] Dataspace adapter DTOs MUST represent facet, session, tenant, workspace, branch, agent/subagent, cancellation, lease/expiry, stale assertion, and ownership-transfer semantics explicitly before being used for concurrent orchestration.

#### Scenario: Scoped cancellation removes assertions

r[agent_harness_runtime.syndicate_dataspace.scoping.cancellation]
- GIVEN a scoped agent, subagent, tool, or background task is cancelled
- WHEN the dataspace adapter processes the cancellation
- THEN it emits or updates bounded cancellation assertions for the affected scope
- AND unrelated tenant, workspace, branch, or session assertions are not modified.

### Requirement: Fake dataspace backend

r[agent_harness_runtime.syndicate_dataspace.fake_backend] Animus MUST provide deterministic fake-dataspace fixtures before live dataspace transports are treated as release evidence.

#### Scenario: Fake backend exercises ordering and stale cleanup

r[agent_harness_runtime.syndicate_dataspace.fake_backend.deterministic]
- GIVEN publish, subscribe, expiry, cancellation, and malformed-subscription fixtures
- WHEN fake-dataspace tests run
- THEN ordering and cleanup outcomes are deterministic
- AND no live network or daemon is required.

### Requirement: Dataspace tests

r[agent_harness_runtime.syndicate_dataspace.tests] Dataspace adapter work MUST include positive and negative tests for bounded projections, scoped publish/subscribe, cancellation, stale assertions, malformed subscriptions, denied queries, dependency isolation, and safe failure evidence.

#### Scenario: Malformed subscription fails safely

r[agent_harness_runtime.syndicate_dataspace.tests.malformed_subscription]
- GIVEN a subscription requests raw private payloads, crosses tenant/workspace scope, or uses an unsupported query shape
- WHEN the dataspace adapter validates it
- THEN validation fails with a typed issue code
- AND no raw private material is returned.

### Requirement: Dependency isolation

r[agent_harness_runtime.syndicate_dataspace.dependency_isolation] Animus MUST check that Syndicate crates, daemon clients, and live transport dependencies remain out of default reusable runtime/core dependency graphs unless a future accepted change makes them default.

#### Scenario: Syndicate dependency leaks into core

r[agent_harness_runtime.syndicate_dataspace.dependency_isolation.rejects_leak]
- GIVEN a Syndicate or live transport dependency is added to a default reusable runtime/core crate
- WHEN dependency isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to use the optional dataspace adapter boundary.

### Requirement: Dataspace closeout evidence

r[agent_harness_runtime.syndicate_dataspace.closeout] Syndicate dataspace adapter work MUST document optional live integration configuration and run fake-dataspace tests, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Dataspace evidence is reviewable

r[agent_harness_runtime.syndicate_dataspace.closeout.evidence]
- GIVEN dataspace adapter work is ready to archive
- WHEN reviewers inspect evidence
- THEN fake-dataspace tests, dependency isolation, workspace checks, and Cairn gates are named with status
- AND live integration is either explicitly configured or marked unavailable with a blocker.

### Requirement: Steel decision fabric baseline

r[agent_harness_runtime.baseline.steel_fabric] Animus SHOULD use Steel Scheme as the default scriptable turn-decision fabric while Rust boundary code MUST revalidate every selected model, tool, memory, response, or host action before dispatch.

#### Scenario: Script decisions are revalidated

r[agent_harness_runtime.baseline.steel_fabric.revalidated]
- GIVEN a Steel script returns a loop decision
- WHEN Rust runtime code receives that decision
- THEN the decision is decoded through typed boundary DTOs
- AND malformed payloads, unsupported actions, unauthorized effects, or invalid text fail closed before adapter dispatch.

### Requirement: Nickel-owned repository configuration

r[agent_harness_runtime.baseline.nickel_config] Repo-owned tool and policy configuration MUST use Nickel as the source of truth when Animus owns the config shape, while generated tool-native artifacts are kept fresh for consumers that require JSON, TOML, YAML, or other native formats.

#### Scenario: Generated config drift is detectable

r[agent_harness_runtime.baseline.nickel_config.drift]
- GIVEN maintainers change repo-owned Nickel configuration
- WHEN generated native config files are checked
- THEN drift is reported deterministically
- AND stale generated config is not treated as authoritative evidence.

### Requirement: Verification safety rails

r[agent_harness_runtime.baseline.verification] The workspace MUST maintain focused positive and negative tests, formatting, linting, Tiger Style, generated-config drift checks, feature checks, and lifecycle validation evidence for behavior that crosses public API, adapter, policy, serialization, or host boundaries.

#### Scenario: Boundary changes include reviewable evidence

r[agent_harness_runtime.baseline.verification.evidence]
- GIVEN a boundary-affecting change is ready for review
- WHEN maintainers inspect its evidence
- THEN the evidence names the focused checks that cover the changed boundary
- AND unavailable checks are recorded with a blocker and next best validation path.

### Requirement: Focused rail convention

r[agent_harness_runtime.clankers_focused_rails.convention] Animus MUST define a focused verification rail convention where each rail validates one boundary, uses deterministic fixtures, emits safe versioned evidence, and avoids live credentials or host-specific state by default.

#### Scenario: Rails have bounded evidence

r[agent_harness_runtime.clankers_focused_rails.convention.evidence]
- GIVEN a focused rail validates an Animus boundary
- WHEN the rail writes evidence
- THEN evidence includes schema, status, fixture names, hashed artifacts, and safe metadata
- AND it omits raw prompts, credentials, provider payloads, full paths, raw tool output, and live host state.

### Requirement: Lifecycle documentation rail

r[agent_harness_runtime.clankers_focused_rails.lifecycle] Animus MUST provide a lifecycle-doc marker rail that checks boundary names, owner markers, forbidden dependency sections, and validation command references.

#### Scenario: Lifecycle rail catches drift

r[agent_harness_runtime.clankers_focused_rails.lifecycle.drift]
- GIVEN the lifecycle document loses a required boundary, owner, forbidden dependency section, or validation marker
- WHEN the lifecycle rail runs
- THEN it fails with an actionable diagnostic
- AND it does not require provider credentials or live services.

### Requirement: Steel runtime policy rail

r[agent_harness_runtime.clankers_focused_rails.steel_policy] Animus MUST provide a focused Steel runtime policy rail that covers Nickel-authored policy export drift, generated-policy activation, current Steel activation, Rust adapter-boundary revalidation, invalid Steel program rejection before adapter dispatch, and an explicit policy-drift hook when a canonical policy source is supplied.

#### Scenario: Steel rail covers positive and negative activation

r[agent_harness_runtime.clankers_focused_rails.steel_policy.fixtures]
- GIVEN current positive and negative Steel runtime fixtures exist
- WHEN the Steel runtime policy rail runs
- THEN valid Steel programs activate safely through Rust DTO validation
- AND invalid Steel programs fail before model or tool adapter dispatch
- AND policy-drift checking is either executed against an explicit canonical source or reported as skipped with a next-best validation path.

### Requirement: Nickel-authored Steel runtime policy

r[agent_harness_runtime.clankers_steel_policy.nickel] Animus Steel runtime profiles MUST be authored in Nickel with typed contracts for profile identity, script bindings, BLAKE3 script hashes, runtime budgets, allowed host actions, fallback mode, and receipt redaction.

#### Scenario: Policy contract accepts valid profiles

r[agent_harness_runtime.clankers_steel_policy.nickel.valid]
- GIVEN a maintained Steel runtime profile under `policy/steel-runtime/`
- WHEN Nickel validation runs
- THEN valid profile identity, script binding, hash, budget, host-action, fallback, and receipt-redaction fields pass
- AND the policy remains reviewable before runtime use.

### Requirement: Checked export and invalid fixtures

r[agent_harness_runtime.clankers_steel_policy.export] Rust runtime tests MUST consume checked policy data or typed fixtures and MUST NOT evaluate Nickel during turn execution.

#### Scenario: Policy export drift fails

r[agent_harness_runtime.clankers_steel_policy.export.drift]
- GIVEN generated policy data exists for Rust consumption
- WHEN the drift check runs
- THEN it fails if generated data differs from Nickel source
- AND it reports a deterministic update path.

#### Scenario: Invalid policies fail before activation

r[agent_harness_runtime.clankers_steel_policy.invalid_fixtures]
- GIVEN a policy fixture has a missing script, bad name, non-positive budget, unknown host action, hash mismatch, path escape, or unsupported fallback mode
- WHEN policy validation runs
- THEN validation returns a typed policy error
- AND Steel is not loaded or evaluated.

### Requirement: Policy enforcement before Steel evaluation

r[agent_harness_runtime.clankers_steel_policy.enforced] Animus MUST validate selected profile, script hash, runtime budget, action allowlist, fallback mode, and receipt redaction before Steel loop evaluation can influence model or tool dispatch.

#### Scenario: Policy denial blocks adapter calls

r[agent_harness_runtime.clankers_steel_policy.enforced.blocks]
- GIVEN a runtime turn selects a Steel script that fails policy activation
- WHEN the runtime prepares the turn
- THEN it returns a typed policy-denied failure
- AND no model or tool adapter call is made.

### Requirement: Redacted policy receipts and rails

r[agent_harness_runtime.clankers_steel_policy.receipts] Steel policy activation receipts MUST expose safe status, profile labels, script hash, policy hash, denied reason class, fallback decision, and receipt hash only.

#### Scenario: Receipts omit raw sensitive material

r[agent_harness_runtime.clankers_steel_policy.receipts.redacted]
- GIVEN activation observes prompt text, script source, provider payloads, credentials, or tool bodies
- WHEN a receipt is rendered
- THEN those values are omitted or represented by redaction markers and BLAKE3 hashes
- AND the receipt remains usable as review evidence.

#### Scenario: Focused rails gate closeout

r[agent_harness_runtime.clankers_steel_policy.rails]
- GIVEN maintainers claim Steel runtime policy is adopted
- WHEN focused policy drift and activation rails run
- THEN they cover valid policy, invalid fixtures, enforcement-before-evaluation, and receipt redaction
- AND closeout evidence names the successful rails or explicit blockers.

#### Scenario: Steel policy closeout is evidenced

r[agent_harness_runtime.clankers_steel_policy.closeout]
- GIVEN the Steel policy change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused policy rails, Steel tests, Nickel/config drift checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Receipt, artifact, ledger, and service rails

r[agent_harness_runtime.clankers_focused_rails.receipts_artifacts] Animus MUST provide focused rails for safe receipt validation, leak rejection, BLAKE3 artifact determinism, and redaction-class display behavior.

r[agent_harness_runtime.clankers_focused_rails.ledger] Animus MUST provide focused rails for neutral ledger replay, unsupported-entry failure, safe metadata counts, and missing ledger service behavior.

r[agent_harness_runtime.clankers_focused_rails.injected_services] Animus MUST provide focused rails or tests for injected provider/tool service boundaries, absence of ambient provider behavior, and fail-closed missing/denied/cancelled/malformed outcomes.

#### Scenario: Boundary rails cover negative cases

r[agent_harness_runtime.clankers_focused_rails.boundary_negative]
- GIVEN receipt, artifact, ledger, or injected-service boundary fixtures include unsafe or unavailable cases
- WHEN focused rails run
- THEN each unsafe or unavailable case returns a typed failure or redacted result
- AND no rail accepts leaked sensitive material or false success evidence.

### Requirement: Stable rails wire into Nix checks

r[agent_harness_runtime.clankers_focused_rails.nix_wiring] Stable credential-free focused rails SHOULD be wired into named Nix checks or the default flake check once deterministic; rails not wired into Nix MUST document the blocker.

#### Scenario: Closeout separates evidence from non-claims

r[agent_harness_runtime.clankers_focused_rails.closeout]
- GIVEN a Clankers-pattern adoption is ready for archive
- WHEN closeout evidence is prepared
- THEN it names successful focused rails, workspace tests, clippy, Nickel/config drift checks when applicable, Cairn validation, and Cairn gates
- AND it explicitly identifies skipped rails and non-claims such as live providers, TUI dogfood, daemon tests, VM checks, or production readiness when not covered.

### Requirement: Roadmap taxonomy

r[agent_harness_runtime.roadmap_sequence.taxonomy] Animus MUST classify active Cairn changes by roadmap role, including foundational lifecycle, core-runtime contract, host-adapter, optional-reference, product-surface, and evaluation-only roles.

#### Scenario: Active changes have roadmap roles

r[agent_harness_runtime.roadmap_sequence.taxonomy.classified]
- GIVEN the Cairn tree contains active changes
- WHEN the roadmap classification is checked
- THEN each active change has exactly one primary role
- AND optional secondary relationships are represented as explicit prerequisite or related-change links.

### Requirement: Dependency order

r[agent_harness_runtime.roadmap_sequence.dependency_order] The roadmap MUST define prerequisite tiers that land accepted baseline specs and local validation wiring before runtime behavior, reusable runtime contracts before concrete host adapters, and core safety contracts before product surfaces.

#### Scenario: Dependent change waits for prerequisite contracts

r[agent_harness_runtime.roadmap_sequence.dependency_order.blocks_dependents]
- GIVEN a change depends on runtime loop, injected services, receipts, ledger, event, policy, or host-adapter contracts
- WHEN the prerequisite requirement IDs are not accepted, not in the same archive batch, and not explicitly waived
- THEN roadmap validation reports the dependency as blocked
- AND the dependent change is not marked archive-ready.

### Requirement: Overlap resolution

r[agent_harness_runtime.roadmap_sequence.overlap_resolution] The roadmap MUST assign ownership for overlapping ledger, receipt, provider, hook, manifest, catalog, and tool-contract concerns so duplicate changes converge on shared requirement IDs instead of creating incompatible contracts.

#### Scenario: Overlapping changes share owners

r[agent_harness_runtime.roadmap_sequence.overlap_resolution.shared_owner]
- GIVEN two active changes describe similar ledger, receipt, provider, hook, manifest, catalog, or tool behavior
- WHEN maintainers classify the changes
- THEN one owning concept and prerequisite path is recorded
- AND the non-owning change links to that owner or narrows itself to evaluation-only scope.

### Requirement: Roadmap validation

r[agent_harness_runtime.roadmap_sequence.validation] Animus SHOULD provide a deterministic roadmap fixture or static check that detects unknown change IDs, unknown prerequisite requirement IDs, dependency cycles, unclassified active changes, and product-surface changes that bypass foundational safety contracts.

#### Scenario: Invalid roadmap fails closed

r[agent_harness_runtime.roadmap_sequence.validation.negative]
- GIVEN roadmap fixtures include an unknown change ID, unknown prerequisite ID, dependency cycle, unclassified change, or unsafe product-surface shortcut
- WHEN roadmap validation runs
- THEN validation fails with deterministic issue codes
- AND the diagnostics do not depend on filesystem ordering or ambient state.

### Requirement: Roadmap closeout evidence

r[agent_harness_runtime.roadmap_sequence.closeout] Roadmap sequencing work MUST produce reviewable evidence that lifecycle validation, roadmap validation, and Cairn gates pass before the change is archived.

#### Scenario: Sequencing evidence is reviewable

r[agent_harness_runtime.roadmap_sequence.closeout.evidence]
- GIVEN the roadmap sequencing change is ready to archive
- WHEN reviewers inspect the evidence
- THEN the evidence names successful lifecycle validation, focused roadmap checks, and Cairn proposal/design/tasks gates
- AND any unavailable check has a blocker and a next best validation path.

### Requirement: Generated policy artifact

r[agent_harness_runtime.cairn_policy.generated_artifact] Animus MUST provide repo-local Cairn generated policy data at the path expected by default Cairn runtime validation, or an equivalent repo-owned wrapper that resolves the policy without hidden session-specific absolute paths.

#### Scenario: Default validation finds policy

r[agent_harness_runtime.cairn_policy.generated_artifact.default_path]
- GIVEN a developer runs Cairn validation from the Animus repository root
- WHEN no explicit external policy path is supplied
- THEN validation loads the repo-owned policy artifact or wrapper-resolved policy
- AND it does not depend on a private checkout path.

### Requirement: Policy refresh documentation

r[agent_harness_runtime.cairn_policy.refresh_docs] Animus MUST document the policy source, refresh command, check command, and expected default validation command for Cairn lifecycle artifacts.

#### Scenario: Maintainer refreshes policy intentionally

r[agent_harness_runtime.cairn_policy.refresh_docs.reproducible]
- GIVEN the canonical Cairn policy source changes
- WHEN a maintainer refreshes Animus policy wiring
- THEN the documentation names the source identity and commands used
- AND the generated artifact can be reviewed as an intentional lifecycle dependency update.

### Requirement: Policy drift check

r[agent_harness_runtime.cairn_policy.drift_check] Animus SHOULD provide a deterministic policy drift check or documented fixture that compares the checked-in generated policy to the canonical exported policy when the source is available.

#### Scenario: Stale policy is detected

r[agent_harness_runtime.cairn_policy.drift_check.stale]
- GIVEN the checked-in generated policy differs from the canonical exported policy
- WHEN the drift check runs with the source available
- THEN the check fails with a deterministic diagnostic
- AND it reports the source identity without embedding secrets or local-only full paths in durable evidence.

### Requirement: Default Cairn validation

r[agent_harness_runtime.cairn_policy.default_validation] Animus lifecycle validation MUST be runnable through the default Cairn validation command from the repository root without requiring a developer to remember an external policy argument.

#### Scenario: Validation is local and reproducible

r[agent_harness_runtime.cairn_policy.default_validation.local]
- GIVEN the repository contains lifecycle artifacts and policy wiring
- WHEN `cairn validate --root .` or the pinned equivalent runs
- THEN validation succeeds or fails only on lifecycle/policy content
- AND missing external checkout paths are not reported as validation blockers.

### Requirement: Policy evidence

r[agent_harness_runtime.cairn_policy.evidence] Cairn policy validation evidence SHOULD record policy source identity, policy hash when available, validation command, and validation result.

#### Scenario: Reviewers can audit policy used for validation

r[agent_harness_runtime.cairn_policy.evidence.reviewable]
- GIVEN lifecycle validation evidence is attached to a change
- WHEN reviewers inspect it
- THEN the evidence identifies which policy artifact was used
- AND it includes enough stable identity to reproduce or challenge the validation result.

### Requirement: Policy wiring closeout

r[agent_harness_runtime.cairn_policy.closeout] Policy wiring work MUST run default validation, explicit-policy validation when available, and Cairn proposal/design/tasks gates before archive.

#### Scenario: Policy wiring closeout is reviewable

r[agent_harness_runtime.cairn_policy.closeout.evidence]
- GIVEN the policy wiring change is ready to archive
- WHEN reviewers inspect closeout evidence
- THEN default validation, explicit-policy validation, and relevant Cairn gates are named with their status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Lifecycle layout

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout] Animus MUST treat `cairn/specs/` as the accepted requirements baseline, `cairn/changes/` as active change deltas, and `cairn/archive/` as completed lifecycle history once changes are accepted and archived.

#### Scenario: Accepted spec is source of current requirements

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout.accepted_baseline]
- GIVEN a requirement has been accepted and synced
- WHEN maintainers inspect current lifecycle requirements
- THEN the requirement appears under `cairn/specs/`
- AND the completed change package is archived rather than left as active planning state.

### Requirement: Change threshold guidance

r[agent_harness_runtime.cairn_dogfood.change_threshold] Animus MUST document when work requires a Cairn change package, including public behavior, compatibility semantics, mutation behavior, architecture, receipt/schema contracts, release/evidence contracts, roadmap scope, and safety-boundary changes.

#### Scenario: Small safe edits can stay direct

r[agent_harness_runtime.cairn_dogfood.change_threshold.direct_safe_edit]
- GIVEN a change is limited to typo repair, fixture hardening, or narrow documentation cleanup
- WHEN it does not affect public behavior, architecture, compatibility, mutation, schema, receipt, or roadmap scope
- THEN maintainers may apply it directly
- AND ordinary repository checks still provide evidence.

### Requirement: Gate workflow

r[agent_harness_runtime.cairn_dogfood.gate_workflow] Animus MUST define a lifecycle workflow for proposal, design, tasks, validation, sync dry-run, archive dry-run, and executed archive closeout using Cairn commands or pinned equivalents.

#### Scenario: Change cannot archive without gates

r[agent_harness_runtime.cairn_dogfood.gate_workflow.blocks_incomplete]
- GIVEN an active Cairn change lacks passing proposal, design, tasks, validation, or sync/archive dry-run evidence
- WHEN maintainers attempt closeout
- THEN the change is not considered archive-ready
- AND blockers or waivers are recorded explicitly.

### Requirement: Bounded evidence contract

r[agent_harness_runtime.cairn_dogfood.evidence_contract] Cairn lifecycle evidence SHOULD record command identity, status, issue codes, requirement/change IDs, policy source identity, policy hash when available, output artifact hashes, and unavailable-check blockers while keeping evidence bounded and reviewable.

#### Scenario: Policy identity is visible in evidence

r[agent_harness_runtime.cairn_dogfood.evidence_contract.policy]
- GIVEN Cairn validation or gates run under a policy artifact
- WHEN evidence is recorded
- THEN the evidence identifies the policy source or artifact and hash when available
- AND reviewers can tell which policy accepted the lifecycle content.

### Requirement: Safe evidence summaries

r[agent_harness_runtime.cairn_dogfood.safe_evidence] Durable Cairn evidence summaries MUST omit raw prompts, credentials, provider payloads, full private paths, raw private file bodies, and unbounded command output by default.

#### Scenario: Evidence summary redacts private material

r[agent_harness_runtime.cairn_dogfood.safe_evidence.redacted]
- GIVEN validation, gate, sync, archive, release-readiness, or workspace-check output includes private or unbounded material
- WHEN durable evidence is summarized
- THEN it includes bounded status, counts, issue codes, hashes, and safe relative labels
- AND raw sensitive material is omitted or replaced by approved redaction markers.

### Requirement: Cairn dogfood closeout

r[agent_harness_runtime.cairn_dogfood.closeout] Cairn dogfood work MUST run validation, proposal/design/tasks gates, and sync/archive dry runs for the change itself before archive.

#### Scenario: Dogfood evidence is reviewable

r[agent_harness_runtime.cairn_dogfood.closeout.evidence]
- GIVEN Cairn dogfood work is ready to archive
- WHEN reviewers inspect evidence
- THEN validation, gates, sync dry-run, archive dry-run, and evidence-safety checks are named with status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Clankers product surfaces are rejected as defaults

r[agent_harness_runtime.clankers_product_rejection.docs] Animus MUST document that it adapts bounded Clankers patterns while rejecting Clankers product surfaces as default reusable Animus behavior.

#### Scenario: Rejected surfaces are explicit

r[agent_harness_runtime.clankers_product_rejection.docs.explicit]
- GIVEN maintainers review Clankers references in Animus
- WHEN they inspect the architecture note
- THEN it explicitly rejects default adoption of Clankers TUI, daemon/attach protocol, Matrix/P2P, global `.clankers` state, provider router/OAuth, plugin subprocess runtime, Wasm/Extism loading, Automerge desktop storage, self-evolution apply/rollback, live dogfood harnesses, product tools, and release tagging workflow
- AND it distinguishes those surfaces from adopted boundary patterns.

### Requirement: Reusable crates remain product-surface free

r[agent_harness_runtime.clankers_product_rejection.dependency_isolation] Reusable Animus crates and default std-shell contracts MUST NOT require Clankers product crates, daemon/TUI concepts, global state directories, Matrix/P2P, plugin runtimes, Automerge storage, OAuth/provider router, self-evolution workflows, product tools, or dogfood harnesses.

#### Scenario: Dependency isolation rail rejects product leakage

r[agent_harness_runtime.clankers_product_rejection.dependency_isolation.rail]
- GIVEN reusable Animus crate dependencies and source markers are checked
- WHEN a rejected Clankers product surface appears as a runtime dependency or assumption
- THEN the rail fails with an actionable diagnostic
- AND bounded-reference documentation mentions remain allowed only in documented reference sections.

### Requirement: Future exceptions require explicit Cairn scope

r[agent_harness_runtime.clankers_product_rejection.future_exception] Any future change that adopts a previously rejected Clankers product surface MUST create a focused Cairn change that names host ownership, injected service boundaries, safety policy, redacted receipts, non-default feature gating, and focused validation rails.

#### Scenario: Future exception is gated

r[agent_harness_runtime.clankers_product_rejection.future_exception.gated]
- GIVEN a maintainer wants to add a daemon, TUI, plugin runtime, provider router, Matrix/P2P, global state, or self-evolution surface
- WHEN the proposal is reviewed
- THEN it includes a focused Cairn change with host ownership, injected services, safety policy, redacted receipts, feature gating, and validation rails
- AND the surface remains non-default unless the change explicitly justifies default behavior.

### Requirement: Marker rail guards non-adoption

r[agent_harness_runtime.clankers_product_rejection.marker_rail] Animus MUST provide a focused marker or dependency rail that allows bounded-reference docs while rejecting product-surface dependencies or runtime assumptions in reusable crates.

#### Scenario: Product rejection closeout is evidenced

r[agent_harness_runtime.clankers_product_rejection.closeout]
- GIVEN the product-surface rejection change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes docs checks, dependency/marker rail results, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded tool catalog evaluation

r[agent_harness_runtime.clankers_tool_catalog.scope] Animus MUST evaluate Clankers tool-catalog and capability-pack patterns as bounded references and MUST define any adopted catalog schema in Animus-owned terms.

#### Scenario: Catalog adoption rejects product tools

r[agent_harness_runtime.clankers_tool_catalog.scope.rejects_product]
- GIVEN maintainers evaluate Clankers tool catalogs
- WHEN the adoption decision is documented
- THEN Animus distinguishes descriptor/capability patterns from Clankers product tools and plugin runtimes
- AND it does not import Clankers built-in tools as Animus runtime dependencies.

### Requirement: Optional catalog descriptor schema

r[agent_harness_runtime.clankers_tool_catalog.schema] An Animus tool catalog MAY define descriptors for tool name, description, runtime kind, capability classes, side-effect level, approval policy, input/output schema references, and redaction policy.

#### Scenario: Descriptor schema is explicit

r[agent_harness_runtime.clankers_tool_catalog.schema.explicit]
- GIVEN a tool descriptor is maintained by a host or repo
- WHEN catalog validation reads it
- THEN the descriptor names capabilities, side-effect level, approval behavior, schema references, and redaction behavior
- AND missing required fields fail validation.

### Requirement: Catalog validation fails unsafe descriptors

r[agent_harness_runtime.clankers_tool_catalog.validation] Catalog validation MUST reject duplicate names, built-in collisions, unknown runtime kinds, invalid schemas, dangerous capabilities without approval, and secret-adjacent capabilities without redaction.

#### Scenario: Unsafe catalog fixtures fail

r[agent_harness_runtime.clankers_tool_catalog.fixtures]
- GIVEN catalog fixtures include duplicate names, invalid schemas, unknown runtime kinds, unsafe capabilities, missing approval, or missing redaction
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors still normalize deterministically.

### Requirement: Catalog parsing is runtime-neutral

r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral] Catalog parsing and validation MUST NOT execute product tools, start plugins, load providers, read secrets, inherit environment, or open network connections.

#### Scenario: Runtime-neutral rail catches side effects

r[agent_harness_runtime.clankers_tool_catalog.runtime_neutral.rail]
- GIVEN a catalog manifest is parsed for validation
- WHEN the runtime-neutrality rail runs
- THEN it proves validation is data-only
- AND no tool execution, plugin launch, provider startup, secret read, environment inheritance, or network access is required.

#### Scenario: Tool catalog closeout is evidenced

r[agent_harness_runtime.clankers_tool_catalog.closeout]
- GIVEN the tool catalog evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes adoption-scope docs, manifest validation fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Wasm-supported surface

r[agent_harness_runtime.wasm_compile.surface] Animus MUST define a Wasm-supported crate and feature surface for deterministic embedders, initially targeting `wasm32-unknown-unknown` and including only crates that do not require ambient std shell services.

#### Scenario: Embedders select a portable surface

r[agent_harness_runtime.wasm_compile.surface.portable]
- GIVEN an embedder wants Animus logic in a Wasm module
- WHEN they select the documented Wasm-supported crate and feature profile
- THEN the selected surface includes deterministic reusable contracts such as core state and adapter DTOs
- AND it excludes std shell orchestration, terminal UI, provider clients, native persistence, process execution, and filesystem authority unless a later target-specific contract accepts them.

### Requirement: Root facade supports a no-std Wasm profile

r[agent_harness_runtime.wasm_compile.root_facade] The root `animus` facade MUST be able to expose a no-std-compatible profile for the Wasm-supported surface without requiring `animus-std`, terminal, process, filesystem, provider, or native persistence dependencies.

#### Scenario: Root no-std profile avoids shell dependencies

r[agent_harness_runtime.wasm_compile.root_facade.no_shell]
- GIVEN the root facade is built for the documented Wasm target without std shell features
- WHEN dependency resolution and compile checks run
- THEN shell-only crates are not required by that build
- AND std-backed convenience APIs remain available only under documented host-shell features or host-target builds.

### Requirement: Wasm compile rail

r[agent_harness_runtime.wasm_compile.rail] Animus MUST provide a credential-free Wasm compile rail that checks the documented supported surface against the accepted Wasm target and reports deterministic evidence for review.

#### Scenario: Compile rail catches portability drift

r[agent_harness_runtime.wasm_compile.rail.drift]
- GIVEN a change introduces an incompatible dependency or target assumption into the Wasm-supported surface
- WHEN the Wasm compile rail runs
- THEN the rail fails before review claims Wasm support
- AND the diagnostic identifies the crate or feature profile that no longer compiles.

### Requirement: Shell isolation for Wasm support

r[agent_harness_runtime.wasm_compile.shell_isolation] Animus MUST keep std shell, terminal, Steel runtime, provider, native persistence, process, filesystem, network, credential, clock, and host execution dependencies outside the Wasm-supported surface unless guarded by target or feature gates that the Wasm rail does not enable.

#### Scenario: Host behavior remains explicit

r[agent_harness_runtime.wasm_compile.shell_isolation.explicit]
- GIVEN a host wants process, filesystem, provider, terminal, browser, WASI, plugin, or persistence behavior
- WHEN it composes Animus with host services
- THEN that behavior crosses explicit adapter or shell contracts
- AND Wasm compile support alone does not imply those host behaviors are present or authorized.

### Requirement: Optional bridge participation is explicit

r[agent_harness_runtime.wasm_compile.optional_bridges] Optional bridge crates such as Preserves MAY join the Wasm rail only after their target compatibility, dependency behavior, and adapter-DTO revalidation path are documented and tested.

#### Scenario: Optional bridge support is not assumed

r[agent_harness_runtime.wasm_compile.optional_bridges.evidence]
- GIVEN an optional bridge is considered for Wasm support
- WHEN closeout evidence is prepared
- THEN it either includes successful Wasm compile evidence and positive/negative boundary tests for that bridge
- OR records the blocker and excludes the bridge from the supported Wasm surface.

### Requirement: Wasm closeout evidence

r[agent_harness_runtime.wasm_compile.closeout] Boundary-affecting Wasm support changes MUST include reviewable evidence for the focused Wasm rail, relevant crate tests, workspace checks, Cairn validation, and Cairn gates, with unavailable checks recorded alongside blockers and next-best validation.

#### Scenario: Reviewers can distinguish support from non-claims

r[agent_harness_runtime.wasm_compile.closeout.non_claims]
- GIVEN a Wasm support change is ready to archive
- WHEN reviewers inspect its evidence
- THEN the evidence names the exact crate and feature surface that compiles for Wasm
- AND it explicitly lists excluded runtime claims such as browser APIs, WASI host behavior, plugin loading, providers, terminal UI, process execution, filesystem access, credentials, and native persistence.

### Requirement: Animus lifecycle map

r[agent_harness_runtime.clankers_lifecycle_doc.map] Animus MUST document the runtime request lifecycle from typed runtime input through deterministic core state, Steel loop decisions, adapter dispatch, event/report projection, safe receipt construction, and host ledger or persistence seams.

#### Scenario: Golden path is reviewable

r[agent_harness_runtime.clankers_lifecycle_doc.map.reviewable]
- GIVEN a maintainer needs to change runtime request behavior
- WHEN they inspect `docs/request-lifecycle.md`
- THEN the document describes the ordered Animus runtime path
- AND it distinguishes reusable core/adapters/Steel behavior from std-shell or host-owned behavior.

### Requirement: Boundary owners and source-of-truth types

r[agent_harness_runtime.clankers_lifecycle_doc.owners] Each lifecycle boundary MUST name the owning crate and source-of-truth type or trait for the state transition.

#### Scenario: Owner names prevent duplicate state machines

r[agent_harness_runtime.clankers_lifecycle_doc.owners.no_duplicate_state]
- GIVEN a change touches runtime input, Steel decisions, model/tool dispatch, reports, events, receipts, or ledger replay
- WHEN reviewers inspect the lifecycle document
- THEN the owning crate and source-of-truth type or trait are named
- AND the change does not introduce an undocumented duplicate owner for the same transition.

### Requirement: Forbidden dependency rules

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies] The lifecycle document MUST identify forbidden dependencies and side effects for reusable core, adapter, Steel, std-shell, receipt, and future ledger boundaries.

#### Scenario: Forbidden dependencies stay visible

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies.visible]
- GIVEN a reusable Animus crate boundary is reviewed
- WHEN the lifecycle document is consulted
- THEN it identifies which boundaries must not read environment, filesystem, credentials, provider config, network, clocks, or concrete host persistence
- AND it points to the validation rail that checks those constraints when one exists.

### Requirement: Lifecycle links and marker rail

r[agent_harness_runtime.clankers_lifecycle_doc.links] Animus README or architecture docs MUST link to the lifecycle document so maintainers can find it before changing runtime behavior.

#### Scenario: Focused rail guards lifecycle drift

r[agent_harness_runtime.clankers_lifecycle_doc.rail]
- GIVEN the lifecycle document is part of the maintained architecture contract
- WHEN the focused marker rail runs
- THEN it fails if required boundary names, owner markers, forbidden dependency sections, or validation command references are missing
- AND it emits reviewable evidence without requiring live providers or host credentials.

#### Scenario: Lifecycle closeout is evidenced

r[agent_harness_runtime.clankers_lifecycle_doc.closeout]
- GIVEN the lifecycle change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused lifecycle rail, relevant docs checks, Cairn validation, and Cairn proposal/design/tasks gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded runtime facade inventory adoption

r[agent_harness_runtime.clankers_runtime_facade_inventory.scope] Animus MUST adopt Clankers' runtime-facade inventory pattern only as a bounded architecture and verification rail, not as a dependency on Clankers runtime crates or product surfaces.

#### Scenario: Inventory adoption rejects product surfaces

r[agent_harness_runtime.clankers_runtime_facade_inventory.scope.rejects_product]
- GIVEN maintainers adapt Clankers runtime-facade inventory ideas
- WHEN the adoption scope is documented
- THEN Animus distinguishes export-classification discipline from Clankers daemon, TUI, provider router, plugin runtime, auth flow, process manager, desktop storage, and product shell behavior
- AND reusable Animus crates do not import Clankers runtime crates for the inventory.

### Requirement: Public export inventory schema

r[agent_harness_runtime.clankers_runtime_facade_inventory.schema] Animus MUST define an inventory record schema for runtime-facing public exports that records export path, owning crate, source module or marker, class, side-effect allowance, stability intent, review note, and safe evidence metadata.

#### Scenario: Export records name owners and authority class

r[agent_harness_runtime.clankers_runtime_facade_inventory.schema.owner_class]
- GIVEN a runtime-facing public export is included in the inventory
- WHEN reviewers inspect the record
- THEN the record names the owning crate and source location or marker
- AND it classifies the export as a reusable contract, host-injection surface, or shell adapter.

### Requirement: Export classes preserve reusable boundaries

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification] Runtime-facing public exports MUST be classified as reusable contracts, host-injection surfaces, or shell adapters, and each class MUST define forbidden side effects.

#### Scenario: Reusable contract remains no-ambient

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification.reusable_no_ambient]
- GIVEN an export is classified as a reusable contract
- WHEN the inventory rail checks it
- THEN the export does not require ambient filesystem, environment, credentials, clocks, randomness, network, provider discovery, TUI state, daemon frames, plugin launch, process execution, or concrete persistence
- AND any needed authority is represented through a host-injection surface or shell adapter instead.

#### Scenario: Host injection fails closed by default

r[agent_harness_runtime.clankers_runtime_facade_inventory.classification.fail_closed]
- GIVEN an export is classified as a host-injection surface
- WHEN the default service or adapter is absent, unavailable, denied, cancelled, or unsupported
- THEN it fails closed with typed status or denial metadata
- AND it does not silently create a shell adapter, read ambient config, discover providers, or contact backends.

### Requirement: Focused inventory rail

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail] Animus MUST provide a focused static rail or equivalent deterministic test that fails on unclassified runtime-facing public exports, missing owner metadata, class and side-effect contradictions, and unsafe evidence fields.

r[agent_harness_runtime.clankers_runtime_facade_inventory.positive] The inventory rail MUST accept fixtures or real exports for pure DTOs, validation helpers, adapter traits, and deny-by-default host service seams.

r[agent_harness_runtime.clankers_runtime_facade_inventory.negative] The inventory rail MUST reject fixtures or real exports that classify ambient filesystem, environment, credential, clock, network, provider, TUI, daemon, plugin, process, or persistence behavior as reusable contracts.

#### Scenario: Positive and negative fixtures guard classification

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail.fixtures]
- GIVEN inventory fixtures or real exports include reusable contracts, host-injection seams, and unsafe shell-authority examples
- WHEN the focused inventory rail runs
- THEN valid reusable and host-injection surfaces are accepted
- AND unsafe ambient-authority examples are rejected as reusable contracts.

#### Scenario: Unclassified exports fail review

r[agent_harness_runtime.clankers_runtime_facade_inventory.rail.unclassified]
- GIVEN a runtime-facing crate adds a public export without inventory classification
- WHEN the focused inventory rail runs
- THEN it fails with an actionable diagnostic naming the missing classification
- AND it does not require live providers, host credentials, plugin runtimes, daemon sessions, or TUI state.

### Requirement: Safe inventory evidence and docs

r[agent_harness_runtime.clankers_runtime_facade_inventory.evidence] Inventory evidence MUST include schema/status, checked crates, class counts, fixture names, safe diagnostics, and content hashes when needed, and MUST omit raw prompts, provider payloads, credentials, environment values, full host paths, and tool output.

r[agent_harness_runtime.clankers_runtime_facade_inventory.docs] Animus README or architecture docs MUST link to an inventory summary that tells embedders which surfaces are default-safe reusable contracts, host-owned injection points, or shell-only adapters.

#### Scenario: Closeout evidence separates claims from non-claims

r[agent_harness_runtime.clankers_runtime_facade_inventory.closeout]
- GIVEN the runtime facade inventory change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused inventory rail, positive and negative fixtures, dependency checks, docs links, Cairn validation, and Cairn gates
- AND skipped checks or non-claims such as live providers, TUI dogfood, daemon tests, plugin runtime tests, VM checks, or production readiness are identified explicitly.

### Requirement: Plugin manifests are bounded descriptor references

r[agent_harness_runtime.clankers_plugin_manifest.scope] Animus MUST treat Clankers plugin manifests as bounded descriptor references and MUST NOT adopt plugin execution, stdio supervision, Wasm loading, Zellij, MCP, or sandbox enforcement in this change.

#### Scenario: Descriptor scope rejects runtime adoption

r[agent_harness_runtime.clankers_plugin_manifest.scope.rejects_runtime]
- GIVEN maintainers evaluate Clankers plugin manifests
- WHEN the adoption scope is reviewed
- THEN it documents descriptor fields separately from runtime execution
- AND it does not add plugin process launch, Wasm loading, MCP/Zellij integration, or sandbox enforcement as default Animus behavior.

### Requirement: Optional manifest schema

r[agent_harness_runtime.clankers_plugin_manifest.schema] Animus MAY define plugin descriptor fields for identity, runtime kind, declared tools, capabilities, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction.

#### Scenario: Manifest descriptors declare authority needs

r[agent_harness_runtime.clankers_plugin_manifest.schema.authority]
- GIVEN a plugin descriptor is parsed
- WHEN validation inspects it
- THEN declared tools, capabilities, runtime kind, launch policy metadata, sandbox requirements, environment allowlist names, writable roots, network policy, and redaction are explicit
- AND missing required descriptor fields fail validation.

### Requirement: Manifest validation rejects unsafe declarations

r[agent_harness_runtime.clankers_plugin_manifest.validation] Manifest validation MUST reject unknown runtime kinds, missing required fields, duplicate or built-in tool collisions, path escapes, absolute writable roots, environment value leakage, and unsafe network/sandbox combinations.

#### Scenario: Invalid manifest fixtures fail

r[agent_harness_runtime.clankers_plugin_manifest.fixtures]
- GIVEN manifest fixtures include path escapes, absolute writable roots, env values, missing commands, unknown runtime kinds, tool collisions, or unsafe network/sandbox combinations
- WHEN validation runs
- THEN each invalid fixture fails with an actionable diagnostic
- AND safe descriptors normalize deterministically.

### Requirement: Manifest parsing is runtime-neutral

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral] Manifest parsing MUST NOT launch stdio processes, load Wasm, inherit environment values, read secrets, connect to network, or execute tools.

#### Scenario: Runtime-neutrality rail catches plugin side effects

r[agent_harness_runtime.clankers_plugin_manifest.runtime_neutral.rail]
- GIVEN plugin descriptor parsing is validated
- WHEN the runtime-neutrality rail runs
- THEN it proves parsing is data-only
- AND no plugin execution, Wasm load, secret read, environment inheritance, or network access is required.

#### Scenario: Plugin manifest closeout is evidenced

r[agent_harness_runtime.clankers_plugin_manifest.closeout]
- GIVEN the plugin manifest boundary evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, manifest fixtures, runtime-neutrality checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Release evidence index is a bounded future pattern

r[agent_harness_runtime.clankers_evidence_index.scope] Animus MUST treat Clankers release evidence indexing as a bounded future pattern and MUST NOT adopt the full Clankers harness, dogfood tests, live model checks, VM gates, or release tagging workflow in this evaluation change.

#### Scenario: Evidence index scope avoids harness adoption

r[agent_harness_runtime.clankers_evidence_index.scope.bounded]
- GIVEN maintainers evaluate Clankers release evidence
- WHEN the adoption note is reviewed
- THEN it documents evidence-index concepts separately from Clankers' full harness and dogfood suites
- AND it keeps live model checks, VM gates, TUI/daemon dogfood, and release tagging out of scope.

### Requirement: Evidence index schema composes receipts

r[agent_harness_runtime.clankers_evidence_index.schema] A future Animus evidence index SHOULD compose focused rail receipts, workspace tests, clippy, flake checks, Nickel/config drift checks, Cairn validation, and Cairn gates without rerunning missing modes by default.

#### Scenario: Index composes selected evidence

r[agent_harness_runtime.clankers_evidence_index.schema.compose]
- GIVEN multiple local receipts and check summaries exist
- WHEN the evidence index is generated
- THEN it selects and summarizes existing evidence by mode
- AND it does not imply missing modes ran.

### Requirement: Current-head evidence is explicit

r[agent_harness_runtime.clankers_evidence_index.current_head] An evidence index that makes current-head claims MUST record commit, branch, dirty status, receipt/head match, stale receipt status, and missing modes, with VCS collection kept in the imperative shell.

#### Scenario: Stale or dirty evidence is not current-head proof

r[agent_harness_runtime.clankers_evidence_index.current_head.stale]
- GIVEN selected receipts do not match the current head or the worktree is dirty
- WHEN the evidence index is reviewed
- THEN those receipts are labeled stale or dirty
- AND they are not treated as current-head proof.

### Requirement: Non-claims are explicit

r[agent_harness_runtime.clankers_evidence_index.non_claims] Evidence indexes MUST separate pure credential-free checks from optional host-dependent checks and MUST state non-claims for live providers, TUI/daemon dogfood, VM checks, remote services, plugin runtime, and production readiness unless those checks explicitly ran.

#### Scenario: Overclaim fixtures fail

r[agent_harness_runtime.clankers_evidence_index.fixtures]
- GIVEN fixture evidence omits live provider, daemon/TUI, VM, plugin, or production-readiness checks
- WHEN index validation runs
- THEN claims for those omitted surfaces are rejected or labeled as non-claims
- AND pure evidence remains usable for the contracts it actually covers.

#### Scenario: Evidence index closeout is documented

r[agent_harness_runtime.clankers_evidence_index.closeout]
- GIVEN the evidence index evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, schema fixtures, current-head fixtures, non-claim fixtures, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Runtime crate boundary

r[agent_harness_runtime.loop.contract] The workspace MUST provide an `animus-runtime` crate that owns reusable agent-loop orchestration without depending on concrete provider, tool, filesystem, process, network, UI, or Syndicate implementations.

#### Scenario: Runtime crate is embeddable

r[agent_harness_runtime.loop.contract.embeddable]
- GIVEN an embedder depends on `animus-runtime`, `animus-adapters`, and a chosen loop fabric
- WHEN they construct a runtime with model and tool adapters
- THEN they can execute the agent loop without pulling in concrete provider or tool crates
- AND the runtime public API exposes typed inputs, configuration, events, reports, and errors.

### Requirement: Pure runtime loop core

r[agent_harness_runtime.loop.core] Runtime decision handling MUST be decomposed into a pure deterministic core over explicit turn state, decisions, adapter outcomes, and configuration, plus a thin shell that calls fabrics and adapters.

#### Scenario: Runtime core is directly testable

r[agent_harness_runtime.loop.core.pure]
- GIVEN a modeled turn state, runtime configuration, and next decision
- WHEN the runtime core evaluates the transition
- THEN it returns typed next state, events, and completion status deterministically
- AND it does not read files, inspect environment, spawn processes, use clocks, log, mutate global state, or call adapters.

### Requirement: Steel default loop fabric

r[agent_harness_runtime.steel_fabric.default] The runtime SHOULD provide a default constructor or helper that uses `animus-steel` as the scriptable agent-loop fabric while preserving a swappable fabric boundary.

#### Scenario: Steel decision drives runtime turn

r[agent_harness_runtime.steel_fabric.default.turn]
- GIVEN a Steel program defines a valid `(animus-decide)` function
- WHEN the runtime evaluates a turn through the Steel fabric
- THEN the returned decision is validated by Rust boundary DTOs before dispatch
- AND invalid Steel load, evaluation, shape, or payload errors stop the turn with typed runtime failure.

### Requirement: Adapter decision dispatch

r[agent_harness_runtime.adapter_dispatch] Runtime dispatch MUST route validated model and tool decisions through injected `ModelAdapter` and `ToolAdapter` implementations and MUST NOT construct concrete provider or tool clients internally.

#### Scenario: Model decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.model]
- GIVEN the loop fabric returns a validated model request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected model adapter with the validated request
- AND records a typed model-requested and model-responded event or a typed adapter-error event.

#### Scenario: Tool decision uses injected adapter

r[agent_harness_runtime.adapter_dispatch.tool]
- GIVEN the loop fabric returns a validated tool request decision
- WHEN runtime dispatches the turn
- THEN it calls only the injected tool adapter with the validated request
- AND records a typed tool-requested and tool-responded event or a typed adapter-error event.

### Requirement: Runtime stop and max-turn behavior

r[agent_harness_runtime.loop.max_turns] Runtime configuration MUST include an explicit max-turn limit and MUST halt with typed status when the fabric returns stop, final response, adapter failure, invalid decision, or max-turn exhaustion.

#### Scenario: Max turns fail closed

r[agent_harness_runtime.loop.max_turns.exhausted]
- GIVEN runtime configuration allows a finite number of turns
- WHEN the loop reaches that limit without final response or stop
- THEN runtime halts with a max-turns-exceeded status
- AND no additional model or tool adapter calls occur.

### Requirement: Runtime positive tests

r[agent_harness_runtime.tests.positive] Runtime work MUST include positive tests for one-turn model calls, tool calls, final responses, explicit stop behavior, and multi-turn propagation of previous model/tool outputs into the next fabric turn.

#### Scenario: Multi-turn state is propagated

r[agent_harness_runtime.tests.positive.multi_turn]
- GIVEN a model adapter response or tool adapter output is produced in one turn
- WHEN the next turn is evaluated
- THEN the loop fabric receives the previous output through typed turn input
- AND the final report records the ordered event sequence.

### Requirement: Runtime negative tests

r[agent_harness_runtime.tests.negative] Runtime work MUST include negative tests for invalid Steel decisions, malformed inputs, model adapter failure, tool adapter failure, max-turn exhaustion, missing final response, and stop-without-response behavior.

#### Scenario: Adapter failure stops safely

r[agent_harness_runtime.tests.negative.adapter_failure]
- GIVEN an injected model or tool adapter returns an error
- WHEN runtime dispatch handles that adapter call
- THEN runtime records a typed adapter-error event
- AND halts without invoking later adapters or fabric turns.

### Requirement: Preserves runtime schema

r[agent_harness_runtime.preserves_schema.contract] Runtime boundary messages SHOULD have Preserves schema coverage for stable event, request, response, final-report, and error payloads before those payloads are treated as external wire contracts.

#### Scenario: Schema validates runtime payloads

r[agent_harness_runtime.preserves_schema.contract.round_trip]
- GIVEN a runtime event or final report is encoded for a boundary
- WHEN Preserves schema validation and round-trip tests run
- THEN valid payloads decode into equivalent typed Rust values
- AND malformed, wrong-kind, missing-field, and invalid-text payloads fail closed.

### Requirement: Preserves schema coverage tiers

r[agent_harness_runtime.preserves_boundary.coverage_tiers] Animus MUST classify Preserves payload coverage as internal DTO round trip, stable external wire contract, compatibility import/export adapter, or review-only fixture before publishing schema compatibility expectations.

#### Scenario: Boundary declares compatibility level

r[agent_harness_runtime.preserves_boundary.coverage_tiers.declared]
- GIVEN a payload family crosses an adapter, runtime, session, provenance, policy, dataset, or external protocol boundary
- WHEN maintainers add Preserves coverage
- THEN the payload family declares its coverage tier
- AND stable external wire contracts have stricter versioning and fixture requirements than internal DTO round trips.

### Requirement: Versioned Preserves schema identities

r[agent_harness_runtime.preserves_boundary.versioning] Stable Preserves boundary payloads MUST carry or bind a schema family and version identity with explicit additive, breaking, deprecated, and unsupported-version behavior.

#### Scenario: Unsupported version fails closed

r[agent_harness_runtime.preserves_boundary.versioning.unsupported]
- GIVEN a Preserves payload has an unsupported schema version
- WHEN boundary decoding runs
- THEN decoding returns a typed unsupported-version error
- AND the runtime does not reinterpret the payload as a different family.

### Requirement: Preserves positive round-trip fixtures

r[agent_harness_runtime.preserves_boundary.round_trip] Stable Preserves boundary work MUST include positive text and binary round-trip fixtures for valid adapter messages, runtime events, final reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows when those families are in scope.

#### Scenario: Valid payload round trips

r[agent_harness_runtime.preserves_boundary.round_trip.valid]
- GIVEN a valid boundary payload fixture
- WHEN it is encoded and decoded through supported Preserves forms
- THEN the decoded typed Rust value is equivalent to the source value
- AND canonical fixture output is deterministic.

### Requirement: Preserves malformed payload rejection

r[agent_harness_runtime.preserves_boundary.malformed] Stable Preserves boundary work MUST include negative fixtures for wrong kind, missing required fields, invalid text, unsupported versions, unbounded payloads, unsafe raw material, and compatibility leak attempts.

#### Scenario: Malformed payload is rejected safely

r[agent_harness_runtime.preserves_boundary.malformed.rejects]
- GIVEN a malformed, wrong-kind, wrong-version, unbounded, or unsafe Preserves payload
- WHEN boundary decoding runs
- THEN decoding fails with a typed boundary error
- AND diagnostics omit raw private payload bodies.

### Requirement: Preserves unknown and future fields

r[agent_harness_runtime.preserves_boundary.unknown_fields] Animus MAY preserve unknown or future fields only when they are bounded, structurally safe metadata; otherwise boundary code MUST fail with typed compatibility errors.

#### Scenario: Unsafe unknown field is rejected

r[agent_harness_runtime.preserves_boundary.unknown_fields.rejects_unsafe]
- GIVEN a payload includes an unknown field carrying raw prompt, credential, provider JSON, full path, file body, tool input/output, or unbounded content
- WHEN compatibility handling evaluates the field
- THEN the field is rejected or redacted according to policy
- AND it is not silently preserved into model-visible context, receipts, or review output.

### Requirement: Preserves core isolation

r[agent_harness_runtime.preserves_boundary.core_isolation] Core business logic MUST consume typed Rust values after Preserves boundary validation and MUST NOT make policy, runtime, or provenance decisions by inspecting raw Preserves syntax trees directly.

#### Scenario: Raw AST stays at boundary

r[agent_harness_runtime.preserves_boundary.core_isolation.no_ast_logic]
- GIVEN a behavior decision depends on adapter, runtime, policy, session, provenance, or dataset fields
- WHEN maintainers inspect the implementation
- THEN raw Preserves parsing and validation is isolated to boundary modules
- AND the decision logic receives typed validated values.

### Requirement: Preserves boundary closeout evidence

r[agent_harness_runtime.preserves_boundary.closeout] Preserves schema work MUST run positive round-trip fixtures, negative malformed fixtures, unknown-field tests, core-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Schema evidence is reviewable

r[agent_harness_runtime.preserves_boundary.closeout.evidence]
- GIVEN Preserves boundary work is ready to archive
- WHEN reviewers inspect evidence
- THEN fixture, compatibility, isolation, workspace, and Cairn gate checks are named with status
- AND unavailable checks include a blocker and next best validation path.

### Requirement: Runtime docs and validation

r[agent_harness_runtime.docs.validation] Runtime work MUST update architecture docs and safety contracts to describe `animus-runtime`, Steel loop-fabric use, adapter dispatch boundaries, and validation commands.

#### Scenario: Runtime closeout is reviewable

r[agent_harness_runtime.validation]
- GIVEN runtime work is ready to archive
- WHEN reviewers inspect the change evidence
- THEN successful logs show focused crate tests, positive and negative runtime tests, Preserves schema tests when added, workspace check/clippy/tests, pre-commit, Octet/Tigerstyle, flake checks, Cairn proposal/design/tasks gates, and Cairn validation.

### Requirement: Semantic agent status DTOs

r[agent_harness_runtime.agent_status_authority.semantic_state] Animus MUST define semantic agent status DTOs for `idle`, `working`, `blocked`, `done`, and `unknown` with source identity, agent label, optional native session reference, bounded message text, presentation metadata, sequence, and optional TTL fields.

#### Scenario: Lifecycle report is validated

r[agent_harness_runtime.agent_status_authority.semantic_state.validation]
- GIVEN a trusted host integration reports an agent label, source identifier, semantic state, target, and sequence
- WHEN Animus validates the status report
- THEN the report is accepted as a typed lifecycle observation
- AND malformed states, malformed source identifiers, missing targets, and unsupported session references are rejected with typed diagnostics.

### Requirement: Authority precedence is deterministic

r[agent_harness_runtime.agent_status_authority.authority_precedence] Status selection MUST choose one semantic authority for a target at a time, and a current trusted lifecycle authority MUST suppress heuristic fallback observations for that target.

#### Scenario: Lifecycle authority wins over heuristic fallback

r[agent_harness_runtime.agent_status_authority.authority_precedence.lifecycle_wins]
- GIVEN a current trusted lifecycle report marks an agent `working`
- AND a host heuristic observation would otherwise classify the same target as `idle`
- WHEN status selection runs
- THEN the selected semantic state is `working`
- AND the heuristic observation is recorded only as non-authoritative fallback evidence.

### Requirement: Presentation metadata is visual-only

r[agent_harness_runtime.agent_status_authority.presentation_only] Titles, display agent names, compact activity text, custom status labels, and state label overrides MUST be presentation-only metadata and MUST NOT change semantic lifecycle state, waits, notifications, rollups, or policy decisions.

#### Scenario: Custom label cannot escalate state

r[agent_harness_runtime.agent_status_authority.presentation_only.no_semantic_escalation]
- GIVEN a metadata report sets a custom visual label that says `needs input`
- AND the selected semantic state remains `working`
- WHEN waits, notifications, rollups, and policy decisions consume agent status
- THEN they use `working`
- AND the visual label is available only to presentation hosts.

### Requirement: Explainable status decisions are bounded

r[agent_harness_runtime.agent_status_authority.explain] Animus SHOULD expose bounded status-decision explanations that identify the selected authority, rejected candidates, freshness class, fallback reason, safe hashes, and redacted evidence summaries without embedding raw prompts, terminal snapshots, provider payloads, credentials, or unbounded detector output.

#### Scenario: Explanation rejects raw evidence leaks

r[agent_harness_runtime.agent_status_authority.explain.redacted]
- GIVEN a status explanation fixture contains raw terminal text, prompt text, provider JSON, a credential-looking value, or an unbounded detector transcript
- WHEN explanation validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the explanation is not accepted as durable evidence.

### Requirement: Detection is host-owned

r[agent_harness_runtime.agent_status_authority.host_owned_detection] Reusable Animus runtime crates MUST NOT inspect processes, read terminals, parse screen buffers, or load detector manifests; they MUST consume typed host observations supplied through adapter contracts.

#### Scenario: Pure status selection has no ambient I/O

r[agent_harness_runtime.agent_status_authority.host_owned_detection.no_ambient_io]
- GIVEN status selection receives typed lifecycle and heuristic observations in memory
- WHEN it selects the current semantic state
- THEN the selection result is deterministic over those inputs
- AND the pure status core does not read terminals, processes, files, clocks, environment variables, or network state.

### Requirement: Status authority verification

r[agent_harness_runtime.agent_status_authority.verification] The adaptation MUST include focused positive and negative tests for report validation, authority precedence, stale sequence and TTL handling, presentation-only metadata, bounded explanations, leak rejection, host-owned detection, and pure status selection.

#### Scenario: Closeout evidence covers status authority rails

r[agent_harness_runtime.agent_status_authority.verification.closeout]
- GIVEN maintainers intend to archive the agent status authority adaptation
- WHEN closeout validation runs
- THEN evidence names focused status DTO, authority precedence, presentation metadata, explanation redaction, host-owned detection, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.

### Requirement: Adapter families

r[agent_harness_runtime.std_runtime_adapters.adapter_families] Animus MUST define concrete std-shell adapter families for model providers, filesystem tools, process execution, HTTP/network helpers, and deterministic fixture backends before those effects are exposed through runtime workflows.

#### Scenario: Concrete effects stay in shell crates

r[agent_harness_runtime.std_runtime_adapters.adapter_families.shell_owned]
- GIVEN a concrete adapter opens a provider connection, reads or writes host files, starts a process, accesses HTTP/network resources, reads credentials, or depends on platform APIs
- WHEN maintainers inspect crate dependencies
- THEN that adapter is owned by `animus-std` or an optional host crate
- AND reusable runtime/core crates consume only injected traits and DTOs.

### Requirement: Effect gates

r[agent_harness_runtime.std_runtime_adapters.effect_gates] Concrete adapters MUST check accepted provider registry, tool contract, policy, sandbox, receipt, and event lifecycle contracts before performing host effects, and MUST fail closed when a required contract or approval is absent.

#### Scenario: Adapter refuses ungated effect

r[agent_harness_runtime.std_runtime_adapters.effect_gates.rejects_ungated]
- GIVEN an adapter request lacks required policy authorization, sandbox compatibility, tool schema validation, provider capability support, or host approval
- WHEN the adapter receives the request
- THEN it returns a typed denial before performing the effect
- AND it emits only safe status evidence.

### Requirement: Shell-owned provider registry

r[agent_harness_runtime.pi_provider_registry.shell_registry] Animus MUST keep provider and model registry discovery, config loading, credential lookup, and concrete client construction in `animus-std` or host shells rather than reusable runtime core crates.

#### Scenario: Runtime receives injected model service

r[agent_harness_runtime.pi_provider_registry.shell_registry.injected]
- GIVEN the shell selects a provider/model from registry config
- WHEN runtime dispatches a model call
- THEN runtime calls only the injected model service with a validated request
- AND it does not read provider config, inspect credentials, or construct a network client.

### Requirement: Typed provider and model metadata

r[agent_harness_runtime.pi_provider_registry.metadata] Provider/model registry entries MUST expose typed provider IDs, model IDs, API kind, endpoint identity, input modalities, context budget, output budget, reasoning support, allowed thinking levels, and optional cost metadata.

#### Scenario: Capability metadata clamps thinking level

r[agent_harness_runtime.pi_provider_registry.metadata.thinking]
- GIVEN a selected model does not support a requested thinking level
- WHEN the shell resolves runtime model settings
- THEN the setting is clamped or rejected according to policy
- AND the runtime receives a valid selected model service and effective capability metadata.

### Requirement: Nickel-authored owned registry config

r[agent_harness_runtime.pi_provider_registry.nickel_config] Animus-owned provider registry configuration MUST be authored in Nickel with typed contracts and exported or checked before shell runtime consumption.

#### Scenario: Invalid provider config fails before use

r[agent_harness_runtime.pi_provider_registry.nickel_config.invalid]
- GIVEN a provider config has an invalid provider ID, unsupported API kind, non-positive budget, or missing required endpoint identity
- WHEN the config export or check runs
- THEN validation fails before the shell can select that provider
- AND no runtime model service is constructed from the invalid entry.

### Requirement: Redacted credential resolution

r[agent_harness_runtime.pi_provider_registry.credentials] Credential resolution MUST be an imperative-shell service that returns usable provider credentials or typed redacted failures without exposing raw tokens, environment values, OAuth secrets, or credential file contents.

#### Scenario: Missing credential reports safe diagnostic

r[agent_harness_runtime.pi_provider_registry.credentials.missing]
- GIVEN a selected provider requires a credential that is absent
- WHEN the shell resolves the provider service
- THEN resolution fails with a typed missing-credential diagnostic
- AND the diagnostic names only the credential class or source label, not any secret value.

### Requirement: Dynamic provider validation

r[agent_harness_runtime.pi_provider_registry.dynamic_validation] Host-provided or dynamically registered provider entries MUST pass the same provider/model metadata validation as file-backed registry entries before they can be selected.

#### Scenario: Dynamic invalid model is rejected

r[agent_harness_runtime.pi_provider_registry.dynamic_validation.reject]
- GIVEN a host registers a provider entry at runtime with malformed model metadata
- WHEN the shell validates the dynamic entry
- THEN the entry is rejected with typed diagnostics
- AND active runtime model services remain unchanged.

### Requirement: Provider parity remains a future bounded pattern

r[agent_harness_runtime.clankers_provider_parity.scope] Animus MUST treat Clankers provider/router contract parity as a future bounded pattern and MUST NOT add provider clients, router daemons, OAuth, credential lookup, network transport, or live provider checks in this evaluation change.

#### Scenario: Provider parity scope is explicit

r[agent_harness_runtime.clankers_provider_parity.scope.future]
- GIVEN maintainers evaluate Clankers provider parity patterns
- WHEN the adoption note is reviewed
- THEN it documents parity criteria for future provider abstractions
- AND it keeps concrete provider clients, router daemons, OAuth, credentials, network transport, and live checks out of scope.

### Requirement: Shared provider request fields are defined when needed

r[agent_harness_runtime.clankers_provider_parity.request_fields] If Animus introduces multiple provider or router request DTOs, it MUST define shared request-field expectations and parity checks for fields such as model, messages, system prompt, output limits, temperature, tools, metadata, cache policy, and extra parameters as applicable.

#### Scenario: Shared fields stay aligned

r[agent_harness_runtime.clankers_provider_parity.request_fields.aligned]
- GIVEN multiple provider request DTOs coexist
- WHEN parity tests run
- THEN shared fields serialize and project consistently
- AND missing cross-cutting metadata is detected before provider dispatch.

### Requirement: Constructor inventory catches request drift

r[agent_harness_runtime.clankers_provider_parity.constructor_inventory] Concrete provider request structs with multiple construction sites SHOULD have a constructor-inventory rail that requires each real constructor to set all required shared fields.

#### Scenario: Constructor inventory fails on omitted fields

r[agent_harness_runtime.clankers_provider_parity.constructor_inventory.omission]
- GIVEN a new provider request constructor omits a required shared field
- WHEN the constructor-inventory rail runs
- THEN it fails with an actionable diagnostic
- AND maintainers update either the constructor or the accepted inventory count.

### Requirement: Provider-native message conversion

r[agent_harness_runtime.clankers_provider_parity.message_conversion] Provider routing adapters MUST use provider-native message conversion fixtures when provider-specific message shapes are required and MUST NOT rely on lossy generic JSON conversion.

#### Scenario: Native conversion preserves content

r[agent_harness_runtime.clankers_provider_parity.message_conversion.fixture]
- GIVEN a provider backend requires a native message shape
- WHEN conversion tests compare against an explicit fixture
- THEN user, assistant, tool, summary, and metadata content required by the backend are preserved
- AND lossy generic conversion that drops content fails.

### Requirement: Streaming and provider-prefix failures are tested

r[agent_harness_runtime.clankers_provider_parity.streaming] Future streaming provider claims MUST include parser-entrypoint tests that feed raw stream bytes through the real parser.

r[agent_harness_runtime.clankers_provider_parity.fail_closed_prefix] Future provider-qualified model IDs MUST fail closed for known-but-unavailable providers rather than falling back silently.

#### Scenario: Provider parity closeout is evidenced

r[agent_harness_runtime.clankers_provider_parity.closeout]
- GIVEN provider parity evaluation is ready to archive
- WHEN closeout validation runs
- THEN evidence includes scope docs, request-field criteria, constructor-inventory pattern, message-conversion fixtures, streaming parser-entrypoint criteria, fail-closed prefix criteria, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Deterministic fixture adapters

r[agent_harness_runtime.std_runtime_adapters.fixtures] Animus MUST provide deterministic fixture adapters for default tests of model, filesystem, process, network, clock, and credential behavior without live providers, real host mutation, or ambient secrets.

#### Scenario: Default tests avoid live effects

r[agent_harness_runtime.std_runtime_adapters.fixtures.no_live_effects]
- GIVEN default workspace checks run on a developer or CI machine
- WHEN concrete adapter tests execute
- THEN they use in-memory or fixture-backed adapters
- AND they do not require network access, credentials, process side effects, or host filesystem mutation beyond test-owned temporary state.

### Requirement: Positive adapter tests

r[agent_harness_runtime.std_runtime_adapters.tests_positive] Concrete adapter work MUST include positive tests for successful injected model, tool, filesystem, process, and HTTP helper flows using deterministic fixtures.

#### Scenario: Fixture-backed adapters produce runtime events

r[agent_harness_runtime.std_runtime_adapters.tests_positive.events]
- GIVEN fixture-backed adapters are injected into runtime or shell orchestration
- WHEN a supported request is dispatched
- THEN the adapter returns a typed success
- AND runtime events and receipts record safe identities, status classes, counts, and hashes.

### Requirement: Negative adapter tests

r[agent_harness_runtime.std_runtime_adapters.tests_negative] Concrete adapter work MUST include negative tests for missing credentials, denied capabilities, sandbox mismatch, malformed outputs, timeout or cancellation, unsupported provider/tool features, and receipt leak attempts.

#### Scenario: Adapter failures do not leak sensitive payloads

r[agent_harness_runtime.std_runtime_adapters.tests_negative.safe_failures]
- GIVEN an adapter fails because credentials are missing, capability is denied, sandbox cannot be enforced, output is malformed, or operation times out
- WHEN failure evidence is emitted
- THEN the evidence contains typed issue codes and safe metadata
- AND raw credentials, prompts, provider payloads, full paths, private file bodies, and unbounded command output are omitted.

### Requirement: Dependency isolation

r[agent_harness_runtime.std_runtime_adapters.dependency_isolation] Animus MUST provide dependency-isolation checks proving reusable runtime/core crates do not depend on concrete provider, filesystem, process, network, credential, or platform-enforcement crates.

#### Scenario: Concrete dependency enters reusable crate

r[agent_harness_runtime.std_runtime_adapters.dependency_isolation.rejects_leak]
- GIVEN a concrete adapter dependency is added to a reusable runtime/core crate
- WHEN dependency isolation checks run
- THEN the check fails with an actionable diagnostic
- AND maintainers are directed to move the dependency behind a shell adapter.

### Requirement: Adapter closeout evidence

r[agent_harness_runtime.std_runtime_adapters.closeout] Concrete adapter work MUST document opt-in live smoke-test configuration and run deterministic adapter tests, dependency-isolation checks, workspace checks, and Cairn gates before archive.

#### Scenario: Adapter evidence is reviewable

r[agent_harness_runtime.std_runtime_adapters.closeout.evidence]
- GIVEN concrete adapter work is ready to archive
- WHEN reviewers inspect the evidence
- THEN deterministic tests and dependency-isolation checks are named with status
- AND live smoke tests are either explicitly run with safe configuration or marked unavailable with a blocker.

### Requirement: Typed runtime event stream

r[agent_harness_runtime.pi_event_lifecycle.typed_events] Animus MUST expose a typed runtime event stream for stable agent lifecycle transitions including runtime start, context replay, turn start, loop decision, model dispatch, tool dispatch, tool result, final response, cancellation, compaction, and shutdown.

#### Scenario: Events preserve turn order

r[agent_harness_runtime.pi_event_lifecycle.typed_events.order]
- GIVEN a runtime turn that calls a model and then a tool before producing a final response
- WHEN events are collected from the run
- THEN the events appear in deterministic transition order
- AND each event contains typed correlation IDs sufficient to associate decisions, dispatches, and results.

### Requirement: Host-injected middleware hooks

r[agent_harness_runtime.pi_event_lifecycle.middleware] Animus MUST support host-injected middleware hooks for observe-only, mutate, block, and replace-style boundary decisions without runtime code discovering plugins or loading code from disk.

#### Scenario: Middleware blocks a tool call

r[agent_harness_runtime.pi_event_lifecycle.middleware.block_tool]
- GIVEN a host injects a tool-call middleware that denies a requested capability
- WHEN the loop decision requests that tool
- THEN runtime emits a typed tool-denied event and report status
- AND the tool adapter is not called.

### Requirement: Deterministic hook chaining

r[agent_harness_runtime.pi_event_lifecycle.hook_order] Middleware hooks that can modify a boundary payload MUST run in configured deterministic order, and each later hook MUST observe the accepted output of earlier hooks.

#### Scenario: Hook mutation order is reproducible

r[agent_harness_runtime.pi_event_lifecycle.hook_order.chained]
- GIVEN two middleware hooks modify safe metadata on a model dispatch request
- WHEN runtime prepares the dispatch
- THEN the second hook observes the first hook's accepted mutation
- AND repeated runs with the same hook order produce identical event and report metadata.

### Requirement: Hook failures fail safe at guarded boundaries

r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks] Errors returned by middleware guarding model dispatch, tool dispatch, compaction, replay, or session replacement MUST fail safe with typed runtime failures rather than allowing the guarded action to continue.

#### Scenario: Tool guard error denies execution

r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks.tool]
- GIVEN a middleware hook errors while evaluating a tool-call boundary
- WHEN runtime handles the hook error
- THEN runtime emits a typed hook-failure event and marks the tool call denied or aborted
- AND it does not execute the tool adapter.

### Requirement: Event stream is mode-neutral

r[agent_harness_runtime.pi_event_lifecycle.mode_neutral] Runtime events MUST be reusable by direct SDK-style embedding, headless protocol streaming, receipts, and future UI shells without embedding TUI-specific component state in the core event DTOs.

#### Scenario: Headless consumer receives same lifecycle facts

r[agent_harness_runtime.pi_event_lifecycle.mode_neutral.headless]
- GIVEN a headless host subscribes to runtime events
- WHEN a run completes with model and tool activity
- THEN the host receives the same stable lifecycle facts as an in-process SDK subscriber
- AND no event requires terminal UI state to interpret.

### Requirement: Typed model-router recipes

r[agent_harness_runtime.vllm_semantic_router_recipes.contracts] Animus model-router recipes MUST be typed contracts for single-route, confidence escalation, ratings ensemble, ReMoM-style quorum synthesis, fusion judge/finalizer, and bounded role workflow route families, including allowed roles, output contract, budget, concurrency, quorum, synthesis, repair, and fallback fields.

#### Scenario: Malformed recipe rejected

r[agent_harness_runtime.vllm_semantic_router_recipes.contracts.invalid]
- GIVEN a router recipe has duplicate roles, unknown model roles, a missing output contract, or an unsupported route family
- WHEN recipe validation runs
- THEN validation rejects the recipe before provider dispatch.

### Requirement: Pure request-signal projection

r[agent_harness_runtime.vllm_semantic_router_recipes.signals] Request-signal projection MUST be a pure deterministic function over bounded metadata for task shape, risk, contract pressure, context pressure, latency band, cost band, privacy/locality, and fallback eligibility.

#### Scenario: Signal projection omits unsafe payloads

r[agent_harness_runtime.vllm_semantic_router_recipes.signals.safe]
- GIVEN request metadata is available with sensitive prompt bodies, provider payloads, credentials, or unbounded text omitted
- WHEN signal projection runs
- THEN projection returns bounded labels and score buckets
- AND it does not require or emit raw prompt bodies, provider payloads, credentials, or unbounded text.

### Requirement: Deterministic recipe selection

r[agent_harness_runtime.vllm_semantic_router_recipes.selection] Recipe selection MUST be deterministic over projected signals, validated recipe policy, budget state, allowed backends, and safe model catalog projections, returning a selected recipe plan or typed denial before any provider call.

#### Scenario: Hard task selects collaborative recipe

r[agent_harness_runtime.vllm_semantic_router_recipes.selection.valid]
- GIVEN projected signals indicate high reasoning variance, strict output contract pressure, and eligible model roles within budget
- WHEN recipe selection runs
- THEN the selected plan uses the configured quorum synthesis or fusion recipe
- AND no provider request is sent during selection.

### Requirement: Bounded execution plans

r[agent_harness_runtime.vllm_semantic_router_recipes.bounds] Collaborative router execution plans MUST declare hard limits for max candidates, max parallelism, min-success quorum, timeout class, cancellation behavior, synthesis attempts, repair attempts, and fallback policy before host execution begins.

#### Scenario: Unbounded fan-out denied

r[agent_harness_runtime.vllm_semantic_router_recipes.bounds.invalid]
- GIVEN a recipe plan omits max parallelism, has zero quorum, requires more successes than candidates, allows unlimited repair, or falls back to a disallowed model
- WHEN execution-plan validation runs
- THEN validation returns a typed denial
- AND no model/provider service is called.

### Requirement: Output contracts gate collaborative success

r[agent_harness_runtime.vllm_semantic_router_recipes.output_contract] Collaborative routes MUST validate the final response contract after synthesis, finalization, repair, or fallback before reporting success to callers or automation.

#### Scenario: Malformed final output fails closed

r[agent_harness_runtime.vllm_semantic_router_recipes.output_contract.invalid]
- GIVEN worker responses produce evidence but synthesis or repair returns malformed final output
- WHEN response validation runs
- THEN the route returns a typed malformed-output failure or a policy-declared best-valid-evidence fallback
- AND success is not claimed for invalid output.

### Requirement: Safe router-recipe receipts

r[agent_harness_runtime.vllm_semantic_router_recipes.receipts] Router-recipe receipts MUST record safe metadata such as recipe ID, signal buckets, selected model labels, bounded counts, quorum class, fallback class, policy hash, duration buckets, redaction markers, and BLAKE3 identity, and MUST NOT include raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, or unbounded traces.

#### Scenario: Router receipt leak rejected

r[agent_harness_runtime.vllm_semantic_router_recipes.receipts.rejects_leaks]
- GIVEN a router-recipe receipt fixture contains raw prompt text, provider JSON, worker output, judge rationale, hidden reasoning text, credentials, full paths, or an unbounded trace
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Router-recipe verification

r[agent_harness_runtime.vllm_semantic_router_recipes.verification] The adaptation MUST include focused positive and negative tests for recipe validation, signal projection, deterministic selection, execution bounds, output-contract validation, fallback behavior, and receipt redaction before closeout.

#### Scenario: Closeout evidence covers router recipes

r[agent_harness_runtime.vllm_semantic_router_recipes.verification.closeout]
- GIVEN maintainers intend to archive the router-recipe adaptation
- WHEN closeout validation runs
- THEN evidence names focused recipe, signal, selection, bounds, output, receipt, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: Typed policy/profile descriptors

r[agent_harness_runtime.hermes_policy_profiles.descriptor] Animus MUST define typed policy/profile descriptors for host-supplied operating guidance with policy kind, source scope, trust status, priority, goal text, success criteria, constraints, validation rules, stop rules, provenance, and optional prompt snippets.

#### Scenario: Valid policy profile composes into prompt input

r[agent_harness_runtime.hermes_policy_profiles.descriptor.valid]
- GIVEN a trusted host supplies a policy profile with goal, success criteria, constraints, validation rules, stop rules, provenance, and priority
- WHEN prompt assembly prepares policy context
- THEN the profile contributes an attributed bounded prompt segment
- AND the descriptor remains available as structured metadata for runtime/report evidence.

### Requirement: Deterministic policy precedence and conflict diagnostics

r[agent_harness_runtime.hermes_policy_profiles.precedence] Animus MUST compose multiple policy descriptors in deterministic precedence order and report conflicts between side-effect limits, validation requirements, or stop rules.

#### Scenario: Global safety rule defeats project override

r[agent_harness_runtime.hermes_policy_profiles.precedence.safety_wins]
- GIVEN a trusted global policy forbids a destructive action without explicit approval and a project-local policy attempts to allow that action implicitly
- WHEN policy composition runs
- THEN the global safety constraint remains active
- AND diagnostics identify the overridden project-local rule without embedding unsafe project prompt text.

### Requirement: Policy prompts are outcome-first and bounded

r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene] Policy-derived prompt text MUST prioritize goal, success criteria, constraints, required output shape, validation, and stop rules, and MUST be bounded to prevent context bloat.

#### Scenario: Process-heavy policy is rejected or warned

r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene.process_heavy]
- GIVEN a policy descriptor contains a long process script with no observable success criteria or stop rule
- WHEN policy validation runs
- THEN validation rejects it or emits a prompt-hygiene diagnostic according to policy severity
- AND prompt assembly does not silently include unbounded process-heavy text.

### Requirement: Project-local policy is trust-gated

r[agent_harness_runtime.hermes_policy_profiles.project_trust] Project-local policy descriptors MUST NOT influence prompts, tool activation, command expansion, or runtime guardrails until host trust permits them.

#### Scenario: Untrusted project policy is omitted

r[agent_harness_runtime.hermes_policy_profiles.project_trust.omit]
- GIVEN a project-local policy descriptor is marked untrusted
- WHEN policy context is assembled
- THEN the descriptor is omitted or marked unavailable according to host policy
- AND its constraints and prompt snippets do not override trusted global policy.

### Requirement: Policy descriptors do not replace enforcement

r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary] Animus MUST treat policy descriptors as guidance and configuration input, not as the sole enforcement mechanism for capability denial, read-only mode, mutation serialization, or credential redaction.

#### Scenario: Read-only policy also has runtime guard

r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary.readonly]
- GIVEN policy context says the session is read-only
- WHEN a tool call requests filesystem mutation
- THEN runtime enforcement denies or blocks the mutation through a typed capability boundary
- AND the denial does not rely only on model obedience to prompt text.

### Requirement: Policy profile adaptation verification

r[agent_harness_runtime.hermes_policy_profiles.verification] The policy profile adaptation MUST include focused tests for valid composition, conflict precedence, prompt hygiene, trust gating, enforcement-boundary integration, prompt budget bounds, stale model guidance rejection, and safe diagnostics.

#### Scenario: Closeout evidence covers policy composition

r[agent_harness_runtime.hermes_policy_profiles.verification.closeout]
- GIVEN maintainers intend to archive the Hermes policy profile adaptation
- WHEN closeout validation runs
- THEN evidence names focused policy composition tests, prompt-hygiene tests, trust-boundary tests, enforcement-boundary tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.

### Requirement: MCP Apps evaluation matrix is explicit

r[agent_harness_runtime.goose_mcp_app_surfaces.evaluation_matrix] Animus MUST evaluate Goose MCP Apps patterns with an explicit matrix covering UI resource identity, MIME/profile tags, CSP domains, permission requests, host-context sync, callbacks, sizing, lifecycle bounds, and adoption risk.

#### Scenario: Unsafe UI metadata is classified

r[agent_harness_runtime.goose_mcp_app_surfaces.evaluation_matrix.fixtures]
- GIVEN evaluation fixtures include safe UI metadata and unsafe broad-domain, broad-permission, unbounded-callback, or ambient-host-control descriptors
- WHEN the MCP Apps evaluation matrix is applied
- THEN each fixture is classified with an adoption decision and risk reason
- AND no fixture claims runtime UI execution behavior.

### Requirement: UI metadata descriptors remain data-only

r[agent_harness_runtime.goose_mcp_app_surfaces.metadata_descriptor] Animus MAY define a data-only UI metadata descriptor sketch for future tool-result UI resources, but reusable validation MUST NOT render HTML, run JavaScript, open remote resources, request browser permissions, or send chat callbacks.

#### Scenario: Descriptor validation has no UI execution

r[agent_harness_runtime.goose_mcp_app_surfaces.metadata_descriptor.no_execution]
- GIVEN a metadata descriptor includes a UI resource URI, MIME/profile tag, CSP domains, permissions, and callback names
- WHEN reusable validation runs
- THEN validation inspects only descriptor data
- AND HTML rendering, JavaScript execution, remote resource loading, browser permission requests, and chat callbacks are absent.

### Requirement: Future MCP App surfaces are deny-by-default

r[agent_harness_runtime.goose_mcp_app_surfaces.security_constraints] Any future MCP App-style surface SHOULD default to no remote domains, no nested frames, no device permissions, no clipboard write, no direct model-visible message injection, and bounded host-context callbacks unless explicitly admitted by policy.

#### Scenario: Broad UI permission request is denied in evaluation

r[agent_harness_runtime.goose_mcp_app_surfaces.security_constraints.denied]
- GIVEN a candidate UI metadata descriptor requests wildcard resource domains, camera access, clipboard write, and direct chat-message callbacks without policy admission
- WHEN evaluation applies the security constraints
- THEN the descriptor is classified as rejected for future default behavior
- AND the evaluation records the required host-policy gates before reconsideration.

### Requirement: MCP Apps evaluation closeout is evidenced

r[agent_harness_runtime.goose_mcp_app_surfaces.verification] MCP Apps surface evaluation MUST include the adoption matrix, positive and negative metadata fixtures, runtime-neutrality checks, security-constraint documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: MCP Apps evaluation evidence is complete

r[agent_harness_runtime.goose_mcp_app_surfaces.verification.closeout]
- GIVEN the Goose MCP Apps surface evaluation change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the evaluation matrix, safe and unsafe fixtures, runtime-neutrality checks, security-constraint docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Security review inputs are bounded and redacted

r[agent_harness_runtime.goose_security_review_stack.review_input] Animus MUST define bounded security review inputs for original task labels, recent context summaries, tool call descriptor, action class, active policy labels, and redacted evidence identities.

#### Scenario: Review input omits sensitive raw material

r[agent_harness_runtime.goose_security_review_stack.review_input.redacted]
- GIVEN a tool call includes raw arguments, paths, credentials, provider text, or prompt content
- WHEN security review input is constructed
- THEN the input exposes only safe labels, bounded summaries, and BLAKE3 evidence hashes
- AND raw prompts, credentials, full paths, and unbounded tool arguments are omitted.

### Requirement: Deterministic pattern findings are available

r[agent_harness_runtime.goose_security_review_stack.pattern_findings] Animus MUST provide deterministic scanner findings for prompt injection, destructive actions, exfiltration, privilege escalation, obfuscated remote execution, and out-of-scope mutation attempts.

#### Scenario: Dangerous payloads produce findings

r[agent_harness_runtime.goose_security_review_stack.pattern_findings.detects]
- GIVEN a requested tool call attempts hidden instruction following, credential access, unknown-domain upload, recursive deletion, privilege escalation, or download-and-execute behavior
- WHEN deterministic security scanning runs
- THEN scanning returns one or more findings with category, confidence, finding ID, and safe explanation
- AND the finding evidence does not include the raw secret or full sensitive payload.

### Requirement: Classifier and adversary reviews are advisory

r[agent_harness_runtime.goose_security_review_stack.advisory_review] Optional classifier and adversary reviewer outputs MUST be modeled as advisory allow, warn, block, unavailable, timeout, or malformed outcomes that cannot grant missing authority or weaken active policy.

#### Scenario: Reviewer allow cannot bypass authority

r[agent_harness_runtime.goose_security_review_stack.advisory_review.no_authority]
- GIVEN active capability or sandbox policy denies a tool call
- WHEN an adversary reviewer recommends allowing the call
- THEN permission resolution preserves the denial
- AND no provider, tool, host-effect, or confirmation bypass is executed.

### Requirement: Security review failure behavior is policy controlled

r[agent_harness_runtime.goose_security_review_stack.fail_behavior] Security review outages, timeouts, and malformed results MUST resolve through explicit policy-controlled fail behavior by action risk class.

#### Scenario: High-risk review outage fails closed

r[agent_harness_runtime.goose_security_review_stack.fail_behavior.closed]
- GIVEN policy marks network exfiltration or destructive filesystem mutation as high risk
- WHEN the configured adversary reviewer times out before evaluating that action
- THEN runtime records a review-unavailable denial or required confirmation according to policy
- AND the risky action is not silently allowed by default.

### Requirement: Findings bind through permission and receipt policy

r[agent_harness_runtime.goose_security_review_stack.policy_binding] Security findings MUST flow through permission resolution, confirmation prompts, sandbox descriptors, and safe receipts before any reviewed effect is dispatched.

#### Scenario: Blocked finding stops dispatch

r[agent_harness_runtime.goose_security_review_stack.policy_binding.blocks]
- GIVEN a deterministic scanner or admitted advisory reviewer produces a blocking finding for a tool call
- WHEN runtime evaluates the tool call
- THEN dispatch stops before provider, tool, or host-effect execution
- AND the denial receipt contains only safe finding metadata and redacted evidence identities.

### Requirement: Security review closeout is evidenced

r[agent_harness_runtime.goose_security_review_stack.verification] Security review stack adoption MUST include positive benign-review tests, negative prompt-injection and dangerous-action fixtures, advisory-authority bypass tests, fail-behavior tests, receipt redaction tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Security evidence is complete

- GIVEN the Goose security review stack change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused scanner tests, advisory-review tests, fail-behavior tests, policy-binding tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Typed recipe workflow manifests

r[agent_harness_runtime.goose_recipe_workflows.manifest] Animus MUST define typed recipe/workflow manifests for identity, instructions, initial prompt, activities, parameters, model profile, capability ceilings, toolset references, validation gates, retry policy, subrecipes, response schema, and provenance.

#### Scenario: Valid recipe manifest loads deterministically

r[agent_harness_runtime.goose_recipe_workflows.manifest.valid]
- GIVEN a recipe manifest declares instructions, prompt, parameters, allowed toolsets, validation gates, response schema, and provenance
- WHEN pure manifest validation runs
- THEN validation returns a deterministic accepted manifest DTO
- AND no provider, filesystem, process, network, credential, or tool adapter is invoked.

### Requirement: Nickel is the authoritative recipe source

r[agent_harness_runtime.goose_recipe_workflows.nickel_source] Animus recipe authoring MUST use typed Nickel contracts as source of truth, and YAML or JSON recipe files MUST be derived interchange artifacts unless an external adapter explicitly imports them under validation.

#### Scenario: Generated recipe drift is rejected

r[agent_harness_runtime.goose_recipe_workflows.nickel_source.drift]
- GIVEN a YAML export differs from the Nickel recipe source for the same recipe identity
- WHEN generated-config drift validation runs
- THEN validation reports recipe drift
- AND runtime does not prefer the edited export over the checked Nickel source.

### Requirement: Parameter binding is pure and declared

r[agent_harness_runtime.goose_recipe_workflows.parameters] Recipe parameter binding MUST validate declared template variables, safe built-in variables, optional defaults, select options, and file-parameter admission before prompt assembly.

#### Scenario: Unsafe parameter template fails closed

r[agent_harness_runtime.goose_recipe_workflows.parameters.invalid]
- GIVEN a recipe template references an undeclared variable, omits an optional default, includes an invalid select option, or assigns a default to a file parameter
- WHEN parameter binding runs
- THEN binding returns a typed validation error
- AND no recipe prompt, activity, or subrecipe input is assembled from the invalid template.

### Requirement: Structured automation output is validated

r[agent_harness_runtime.goose_recipe_workflows.structured_output] Recipes that declare structured output MUST validate final responses against the declared schema before reporting automation success.

#### Scenario: Malformed final output blocks success

r[agent_harness_runtime.goose_recipe_workflows.structured_output.invalid]
- GIVEN a headless recipe declares a required JSON output field
- WHEN the final assistant output omits that field or uses the wrong type
- THEN output validation records a structured-output failure
- AND automation does not consume the malformed output as a successful result.

### Requirement: Retry policy remains host-mediated

r[agent_harness_runtime.goose_recipe_workflows.retry_policy] Recipe retry checks and cleanup actions MUST be represented as host-effect requests with capability labels, sandbox requirements, timeouts, redaction, and receipts rather than direct reusable-core shell execution.

#### Scenario: Retry cannot bypass policy denial

r[agent_harness_runtime.goose_recipe_workflows.retry_policy.bypass]
- GIVEN a recipe retry check requests a shell command denied by active capability or sandbox policy
- WHEN the retry planner evaluates the check
- THEN the retry attempt records a denied host-effect request
- AND reusable core does not execute the command or report validation success.

### Requirement: Recipe trust and provenance are explicit

r[agent_harness_runtime.goose_recipe_workflows.trust] Imported, changed, first-run, or project-provided recipes MUST have explicit trust and provenance receipts before they can influence prompt assembly, tool availability, model selection, memory use, or workflow permissions.

#### Scenario: Untrusted recipe has no influence

r[agent_harness_runtime.goose_recipe_workflows.trust.denied]
- GIVEN a project recipe has not been admitted by host policy or user confirmation
- WHEN runtime prepares a session using that recipe
- THEN the recipe is rejected or activated in a no-influence failure state
- AND prompt, tools, model, memory, and workflow permissions remain unchanged.

### Requirement: Recipe workflow closeout is evidenced

r[agent_harness_runtime.goose_recipe_workflows.verification] Recipe workflow adoption MUST include positive and negative tests for manifest validation, Nickel source authority, parameter binding, structured output, retry policy, trust receipts, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Recipe workflow evidence is complete

- GIVEN the Goose recipe workflow change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused recipe workflow tests, Nickel drift checks, structured-output failures, retry-bypass failures, trust-denial failures, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.

### Requirement: Bounded BranchFS adaptation scope

r[agent_harness_runtime.branchfs_workspace.scope] Animus MUST treat BranchFS as a bounded reference for speculative workspace patterns, including copy-on-write branch creation, nested parentage, leaf-only commit and abort discipline, commit-to-parent promotion, zero-cost discard intent, virtual branch addressing, and mmap or cache invalidation awareness, and MUST NOT vendor BranchFS, require FUSE or macFUSE for all users, copy the BranchFS CLI or storage layout as the Animus contract, or expose BranchFS-specific ioctls, tombstones, daemon paths, or mount paths from `animus-core`.

#### Scenario: Scope review rejects backend coupling

r[agent_harness_runtime.branchfs_workspace.scope.review]
- GIVEN a proposed implementation imports BranchFS into reusable core crates, shells out to `branchfs` from pure logic, stores BranchFS mount paths in core DTOs, or requires FUSE for non-workspace runtime paths
- WHEN maintainers compare it with the bounded adaptation scope
- THEN the change is rejected or split until BranchFS behavior remains adapter-local reference evidence
- AND portable Animus workspace descriptors remain independent of BranchFS implementation details.

### Requirement: VCS-neutral speculative workspace descriptors

r[agent_harness_runtime.branchfs_workspace.descriptor] Animus speculative workspace descriptors MUST be typed portable runtime intent covering source identity, workspace identity, parent workspace identity, writable roots, ignored or local-file inclusion policy, commit target, abort policy, conflict policy, cleanup policy, and required adapter capability classes.

#### Scenario: Descriptor captures workspace intent without FUSE details

r[agent_harness_runtime.branchfs_workspace.descriptor.intent]
- GIVEN a runtime action requests an isolated speculative workspace
- WHEN Animus validates the workspace descriptor
- THEN the descriptor names portable workspace intent rather than BranchFS CLI flags, FUSE mount paths, daemon socket paths, storage directories, ioctls, or tombstone files
- AND missing, duplicate, malformed, or unknown required fields fail validation before any workspace backend is invoked.

### Requirement: Pure workspace planning

r[agent_harness_runtime.branchfs_workspace.planning] Workspace descriptor parsing, normalization, branch graph validation, conflict policy validation, unsupported-capability diagnostics, and receipt-safe projection MUST be pure deterministic logic over typed inputs and MUST NOT read the filesystem, inspect the environment, spawn processes, consult clocks, probe kernels, inspect VCS state, start daemons, mount filesystems, or mutate host state.

#### Scenario: Invalid branch graph fails before platform setup

r[agent_harness_runtime.branchfs_workspace.planning.invalid_graph]
- GIVEN a workspace descriptor graph contains a parent cycle, duplicate workspace identity, missing parent identity, non-leaf commit request, or incompatible commit and abort policy
- WHEN pure workspace planning runs
- THEN it returns typed diagnostics naming the invalid graph or policy condition
- AND no FUSE operation, VCS operation, process executor, filesystem operation, daemon request, or platform probe is invoked.

### Requirement: Workspace adapter capability model

r[agent_harness_runtime.branchfs_workspace.adapter] Animus workspace adapters SHOULD expose an explicit capability model for translating normalized workspace plans into enforceable backend behavior, including labels for BranchFS-like copy-on-write branches, Jujutsu or Git worktrees, temporary snapshot directories, nested workspaces, commit-to-parent support, abort support, conflict detection, cleanup guarantees, and unsupported platform states.

#### Scenario: Backend reports unsupported nested commit

r[agent_harness_runtime.branchfs_workspace.adapter.unsupported]
- GIVEN a normalized workspace plan requires nested workspaces and commit-to-parent behavior
- AND the selected adapter can create a temporary snapshot but cannot represent nested parentage or commit-to-parent semantics
- WHEN adapter capability matching runs
- THEN it reports unsupported capability diagnostics for the missing nested and commit behavior
- AND the runtime does not silently downgrade the plan to a weaker workspace mode.

### Requirement: Required workspace isolation fails closed

r[agent_harness_runtime.branchfs_workspace.fail_closed] Runtime dispatch MUST fail closed before model, tool, or process execution when required workspace isolation is malformed, denied by policy, unsupported by the selected adapter, unavailable on the current platform, or only partially enforceable, and MUST NOT treat workspace isolation as authorization proof.

#### Scenario: Required workspace does not fall back to host checkout

r[agent_harness_runtime.branchfs_workspace.fail_closed.no_host_fallback]
- GIVEN a capability request is authorized by its normal contract and grant checks
- AND its local caveat requires speculative workspace isolation
- AND the selected adapter cannot start, mount, snapshot, clean up, or enforce the requested writable-root and commit/abort semantics
- WHEN dispatch evaluates the action
- THEN dispatch is denied before host execution with a typed workspace-unavailable or partial-enforcement issue
- AND the receipt records workspace isolation failure separately from authorization success.

### Requirement: Explicit commit and abort outcomes

r[agent_harness_runtime.branchfs_workspace.commit_abort] Workspace adapters MUST report explicit outcomes for created, committed, aborted, conflict, unavailable, failed-cleanup, and ambiguous states, and nested branch backends MUST preserve leaf-scoped commit and abort behavior so commit promotes only to the immediate parent and abort discards only the selected leaf workspace while preserving parent and sibling workspaces.

#### Scenario: Leaf commit promotes without affecting siblings

r[agent_harness_runtime.branchfs_workspace.commit_abort.leaf_commit]
- GIVEN a parent workspace has two child workspaces and one child is selected for commit
- WHEN the adapter commits the selected leaf workspace
- THEN the committed changes are promoted only to the selected parent target
- AND sibling workspaces remain isolated unless conflict policy explicitly blocks or invalidates them.

#### Scenario: Abort discards selected leaf only

r[agent_harness_runtime.branchfs_workspace.commit_abort.leaf_abort]
- GIVEN a nested workspace tree has a leaf workspace with uncommitted changes
- WHEN the adapter aborts that leaf workspace
- THEN only the selected leaf workspace is discarded
- AND the parent workspace, sibling workspaces, and base source identity remain available or are reported with explicit adapter diagnostics.

### Requirement: Redacted workspace receipts

r[agent_harness_runtime.branchfs_workspace.receipts] Workspace receipts MUST carry safe metadata only, including workspace status, issue code, normalized descriptor hash, source identity hash, adapter capability labels, parent and workspace identity labels, commit or abort outcome, conflict status, cleanup status, bounded manifest counts or BLAKE3 hashes, and BLAKE3 receipt hash, and MUST NOT include raw prompts, credentials, absolute path dumps, raw file contents, raw process output, raw branch manifests, raw network payloads, or unbounded event streams.

#### Scenario: Successful commit receipt is reviewable

r[agent_harness_runtime.branchfs_workspace.receipts.success]
- GIVEN an isolated workspace is created, used, committed to its parent, and cleaned up successfully
- WHEN the workspace receipt is emitted
- THEN it records descriptor, source, manifest, commit, cleanup, and receipt hashes plus bounded adapter capability labels and outcome status
- AND sensitive path, prompt, credential, file-content, process-output, branch-manifest, and payload material is omitted or represented only by approved hashes and redaction markers.

#### Scenario: Receipt validation rejects workspace leaks

r[agent_harness_runtime.branchfs_workspace.receipts.rejects_leaks]
- GIVEN a workspace receipt fixture contains raw prompts, credential-looking values, absolute path dumps, raw file contents, raw process output, raw branch manifests, raw network payloads, or an unbounded event stream
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the receipt cannot be used as review or release evidence.

### Requirement: Focused verification for speculative workspaces

r[agent_harness_runtime.branchfs_workspace.verification] The BranchFS-inspired adaptation MUST include focused positive and negative tests for descriptor normalization, branch graph validation, adapter capability translation, fail-closed dispatch, commit and abort outcomes, cleanup handling, and receipt redaction.

#### Scenario: Positive and negative workspace fixtures gate closeout

r[agent_harness_runtime.branchfs_workspace.verification.fixtures]
- GIVEN the BranchFS-inspired speculative workspace adaptation is ready for review
- WHEN focused verification runs
- THEN positive fixtures show valid descriptor normalization, nested workspace planning, successful create, commit, abort, cleanup, and redacted receipts
- AND negative fixtures show malformed descriptors, invalid branch graphs, unsupported FUSE or backend environments, partial enforcement, commit conflicts, failed cleanup, ambiguous state, and receipt leak attempts fail closed.

r[agent_harness_runtime.branchfs_workspace.closeout] Closeout evidence MUST include successful focused workspace descriptor, adapter, commit/abort, cleanup, receipt, workspace checks, Cairn validation, and Cairn proposal/design/tasks gates before this adaptation is archived.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.branchfs_workspace.closeout.evidence]
- GIVEN maintainers intend to archive the BranchFS-inspired speculative workspace adaptation
- WHEN they collect closeout evidence
- THEN the evidence names every successful focused rail, workspace check, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.
