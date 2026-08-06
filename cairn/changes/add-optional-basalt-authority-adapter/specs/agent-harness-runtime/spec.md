## ADDED Requirements

### Requirement: Basalt integration preserves authority ownership

r[agent_harness_runtime.basalt_authority_adapter.boundary] Animus MUST keep agent definitions, semantic roles, guarded-action construction, capability intersections, dispatch, and completion authority separate from Basalt contract enforcement and UCAN principal, delegation, proof, caveat, time, revocation, and replay semantics. Agent-specific policy MUST NOT move into Basalt.

#### Scenario: Each layer keeps its authority boundary

r[agent_harness_runtime.basalt_authority_adapter.boundary.ownership]
- GIVEN a host authorizes an action for a named Animus agent through the Basalt adapter
- WHEN maintainers inspect the request, verification, decision, and dispatch path
- THEN Animus MUST own the agent, role, task, local ceiling, dispatch, and completion identities while Basalt and UCAN retain their generic policy and token semantics
- AND no layer MUST claim ambient trust, global authority, sandbox confinement, effect correctness, agent safety, or completion from the other layer's allowed decision.

### Requirement: Host authority providers use an implementation-neutral port

r[agent_harness_runtime.basalt_authority_adapter.port] Animus MUST define implementation-neutral host-authority request, agent-binding, decision, issue, and receipt-projection contracts for guarded actions. Generic public contracts MUST NOT expose Basalt or UCAN concrete types.

#### Scenario: Another authority provider remains usable

r[agent_harness_runtime.basalt_authority_adapter.port.replaceable]
- GIVEN a host does not select the Basalt adapter and supplies another implementation of the authority port
- WHEN a guarded action reaches authorization
- THEN Animus MUST pass only generic validated request data to that provider
- AND the provider MUST be usable without resolving, compiling, constructing, or importing Basalt or its vendored UCAN boundary.

#### Scenario: Malformed generic request fails before provider use

r[agent_harness_runtime.basalt_authority_adapter.port.malformed]
- GIVEN a host-authority request has an empty, unsafe, duplicate, wildcard, unsupported, or inconsistent action, contract, resource, ability, identity, digest, or bound
- WHEN generic request validation runs
- THEN Animus MUST return a stable typed denial before calling any authority provider or effect adapter.

### Requirement: Agent authority binds DID and immutable definition identity

r[agent_harness_runtime.basalt_authority_adapter.agent_binding] A host-admitted agent authority binding MUST bind one UCAN audience DID to the exact agent-definition BLAKE3 identity, binding-policy identity, and freshness identity. A friendly agent name, display name, role label, prompt, or manifest text MUST NOT establish or widen authority.

#### Scenario: Named agent uses its admitted DID binding

r[agent_harness_runtime.basalt_authority_adapter.agent_binding.valid]
- GIVEN a selected agent name resolves to an admitted definition whose BLAKE3 identity is bound to one UCAN audience DID under current policy
- WHEN the agent requests a guarded action
- THEN the authority request MUST carry the exact definition, DID, binding-policy, and freshness identities
- AND holder verification MUST establish the bounded invocation separately from name resolution.

#### Scenario: Name-only authority is rejected

r[agent_harness_runtime.basalt_authority_adapter.agent_binding.name_only]
- GIVEN an agent name or display label matches a trusted definition but no current DID and definition binding exists
- WHEN the agent requests a guarded action
- THEN authorization MUST fail with a missing-binding denial
- AND the name, prompt, role label, local allow, or prior receipt MUST NOT substitute for the binding.

#### Scenario: Binding drift fails closed

r[agent_harness_runtime.basalt_authority_adapter.agent_binding.drift]
- GIVEN the DID, definition identity, subject, task, actor, role plan, session, policy, or freshness identity differs from the admitted binding
- WHEN authorization or replay evaluates the request
- THEN the adapter MUST reject the request as stale or mismatched
- AND it MUST NOT reuse a binding from another agent, definition, task, role, session, or prior authorization.

### Requirement: Basalt authority support is optional and isolated

r[agent_harness_runtime.basalt_authority_adapter.isolation] The Basalt authority adapter MUST be provided through a non-default feature and a separate optional standard-library host crate. Default, `no_std`, Wasm, core, adapter, runtime, Steel, TUI, and unrelated host builds MUST NOT acquire Basalt, vendored UCAN, token verification, key access, proof access, replay storage, or network authority.

#### Scenario: Default build excludes Basalt

r[agent_harness_runtime.basalt_authority_adapter.isolation.default]
- GIVEN a consumer builds Animus without the `basalt-authority` feature
- WHEN dependency, feature, public-API, default, `no_std`, Wasm, runtime, Steel, TUI, and unrelated host checks run
- THEN Basalt and its transitive UCAN standard-library boundary MUST be absent from those selected graphs and APIs
- AND existing hosts MUST retain their prior authority-provider choices.

#### Scenario: Required adapter is unavailable

r[agent_harness_runtime.basalt_authority_adapter.isolation.required_unavailable]
- GIVEN host policy requires the Basalt provider for a guarded action but the feature, service, compatible cohort, or current provider instance is unavailable
- WHEN authorization runs
- THEN the guarded action MUST fail closed before dispatch
- AND Animus MUST NOT fall back to a local allow, ambient grant, reference-only re-export, broad capability, or another provider.

### Requirement: Adapter policy is typed Nickel with deterministic projection

r[agent_harness_runtime.basalt_authority_adapter.policy] Animus MUST author Basalt-adapter mode, action-to-contract mapping, immutable dependency identities, supported action classes, named limits, and required non-claims in typed Nickel. Runtime code MUST consume a deterministic checked projection without evaluating Nickel or discovering policy files.

#### Scenario: Reviewed mapping projects exactly

r[agent_harness_runtime.basalt_authority_adapter.policy.valid]
- GIVEN the adapter policy declares unique guarded-action mappings, exact Basalt and UCAN cohort identities, positive named limits, and required non-claims
- WHEN Nickel export and Rust parity validation run
- THEN the generated projection MUST match the reviewed source exactly
- AND the adapter MUST use only an explicitly supplied compatible Basalt policy at runtime.

#### Scenario: Unsafe or ambiguous mapping is rejected

r[agent_harness_runtime.basalt_authority_adapter.policy.invalid]
- GIVEN adapter policy contains duplicate mappings, wildcards, unknown actions, empty contracts, broad fallbacks, ambient policy discovery, runtime Nickel evaluation, weakened non-claims, or unreviewed dependency identities
- WHEN policy validation runs
- THEN validation MUST fail before the optional adapter is constructed or used.

### Requirement: Animus requests translate exactly into Basalt requests

r[agent_harness_runtime.basalt_authority_adapter.translation] A pure deterministic adapter core MUST translate each validated Animus guarded-action request and admitted agent binding into one exact Basalt action, contract, resource, ability, operation profile, and request digest. Translation MUST NOT broaden, truncate, substitute, normalize across authority boundaries, or infer missing authority fields.

#### Scenario: Exact translation preserves every binding

r[agent_harness_runtime.basalt_authority_adapter.translation.exact]
- GIVEN a valid guarded action, admitted agent binding, reviewed action mapping, and current policy identity
- WHEN pure translation runs
- THEN it MUST produce one deterministic Basalt request bound to the same action, principal, definition, subject, task, role, contract, resource, ability, policy, and request digest
- AND repeated translation of the same canonical inputs MUST produce the same BLAKE3 operation and request identities.

#### Scenario: Resource or ability substitution is rejected

r[agent_harness_runtime.basalt_authority_adapter.translation.mismatch]
- GIVEN translation would use a different contract, resource, ability, action class, principal, definition, subject, task, role, policy, operation profile, or digest than the validated Animus request
- WHEN the mapping core evaluates the inputs
- THEN it MUST return a typed binding denial without constructing an executable Basalt invocation.

### Requirement: Live authority uses raw holder-signed UCAN enforcement

r[agent_harness_runtime.basalt_authority_adapter.raw_ucan] The optional host adapter MUST use Basalt raw holder-signed UCAN invocation enforcement for live authority. It MUST NOT treat caller-created grant DTOs, token possession, an agent name, or a prior allow receipt as current authority.

#### Scenario: Current holder-signed invocation is admitted

r[agent_harness_runtime.basalt_authority_adapter.raw_ucan.allowed]
- GIVEN the host supplies a current token, holder proof, key resolution, proof chain, revocation result, caveat result, replay result, time, limits, Basalt policy, and exact request binding
- WHEN Basalt raw UCAN enforcement runs
- THEN an allowed result MAY satisfy the Basalt authority gate for that exact request
- AND replay admission MUST remain the final consuming authorization step before the safe decision commit.

#### Scenario: Missing or denied verification input blocks dispatch

r[agent_harness_runtime.basalt_authority_adapter.raw_ucan.denied]
- GIVEN the token, holder, audience, key, proof, time, capability, caveat, revocation, replay, policy, resource, ability, operation profile, request digest, or limit input is missing, malformed, stale, mismatched, denied, unavailable, or uncertain
- WHEN raw enforcement runs
- THEN the adapter MUST return a stable typed denial before the dependent effect
- AND it MUST NOT fetch, discover, synthesize, persist, or broadly substitute the missing input.

### Requirement: Basalt is an additive non-widening authority ceiling

r[agent_harness_runtime.basalt_authority_adapter.intersection] A Basalt allow MUST satisfy only the Basalt authority-provider gate. Effective authority MUST remain the exact intersection of the verified UCAN and Basalt decision with agent-definition, semantic-role, parent, task, model, tool, workspace, local policy, approval, sandbox, cancellation, timeout, budget, runtime-seal, evidence, and completion gates.

#### Scenario: Basalt allow cannot reverse another denial

r[agent_harness_runtime.basalt_authority_adapter.intersection.no_override]
- GIVEN Basalt allows an exact guarded action but another required Animus ceiling or gate denies, is stale, is unavailable, or lacks a required capability
- WHEN effective authority is resolved
- THEN the action MUST remain denied with the denying sources recorded safely
- AND the Basalt allow MUST NOT mint, union, inherit, or override any missing authority.

#### Scenario: Basalt denial always blocks the configured path

r[agent_harness_runtime.basalt_authority_adapter.intersection.provider_deny]
- GIVEN every non-Basalt Animus gate allows but the configured Basalt provider denies the exact action
- WHEN dispatch admission runs
- THEN the guarded effect MUST remain blocked
- AND no approval, role label, local allow, sandbox descriptor, or completion policy may rewrite the provider denial as allowed.

### Requirement: Supported guarded effects pass the authority port before dispatch

r[agent_harness_runtime.basalt_authority_adapter.dispatch] Every supported live model, tool, filesystem, process, network, Steel, and semantic agent-role guarded-action path MUST invoke the configured implementation-neutral authority port and commit its safe decision before the dependent host effect. Unsupported required paths MUST fail closed with an explicit class.

#### Scenario: Named agent completes one authorized dispatch

r[agent_harness_runtime.basalt_authority_adapter.dispatch.allowed]
- GIVEN a named agent has an admitted DID and definition binding, every local ceiling allows, Basalt raw enforcement allows, and every later host gate allows
- WHEN a supported guarded action is dispatched
- THEN Animus MUST commit the safe authority decision before invoking the selected effect adapter
- AND the effect MUST remain bound to the exact request and decision identities.

#### Scenario: Ungated or unsupported path is denied

r[agent_harness_runtime.basalt_authority_adapter.dispatch.ungated]
- GIVEN a required guarded-action path lacks an authority-port call, compatible action mapping, current decision, durable decision commit, or supported provider behavior
- WHEN the path attempts model, tool, filesystem, process, network, Steel, semantic role, output, selection, veto, or handoff influence
- THEN dispatch MUST fail before the effect or influence occurs
- AND no generic success, compatibility grant, or unguarded fallback path may continue.

### Requirement: Basalt authority receipts are metadata only

r[agent_harness_runtime.basalt_authority_adapter.receipts] Basalt authority request, binding, decision, and dispatch receipts MUST contain only bounded safe action labels and BLAKE3 identities for public stack-owned canonical metadata. They MUST NOT contain raw tokens, keys, signatures, proofs, caveat bodies, revocation entries, replay nonces, credentials, prompts, tool arguments, provider payloads, environment values, full paths, or effect outputs.

#### Scenario: Allowed receipt preserves bounded evidence

r[agent_harness_runtime.basalt_authority_adapter.receipts.allowed]
- GIVEN the optional adapter allows and commits one exact guarded action
- WHEN the receipt core constructs evidence
- THEN it MUST record safe provider, policy, action, ability, principal-hash, definition, binding, request, freshness, outcome, count, redaction, and receipt identities
- AND it MUST state that the decision does not prove global trust, backend truth, replay durability, confinement, effect correctness, agent safety, completion, or release readiness.

#### Scenario: Receipt leak is rejected

r[agent_harness_runtime.basalt_authority_adapter.receipts.reject_leak]
- GIVEN a receipt candidate contains bearer, secret, private, raw effect, provider, environment, path, prompt, proof, caveat, revocation, replay, or output material
- WHEN receipt validation runs
- THEN validation MUST fail with a deterministic leak class
- AND no authorization, dispatch, evidence, completion, or release claim may use the unsafe candidate.

### Requirement: Basalt and UCAN cohorts are compatibility gated

r[agent_harness_runtime.basalt_authority_adapter.compatibility] The optional adapter MUST support only the exact reviewed Basalt revision and vendored UCAN cohort after action mapping, raw enforcement, request binding, receipt, dependency, feature, and license compatibility fixtures pass. An unreviewed cohort MUST remain unsupported.

#### Scenario: Reviewed cohort is admitted explicitly

r[agent_harness_runtime.basalt_authority_adapter.compatibility.reviewed]
- GIVEN the pinned Basalt revision, vendored UCAN revision, policy schema, raw-enforcement API, receipt schema, enabled features, source identities, and license metadata match the reviewed adapter policy
- WHEN compatibility validation runs
- THEN the optional adapter MAY be constructed for the configured host
- AND downstream evidence MUST identify the exact optional AGPL dependency boundary.

#### Scenario: Dependency or license drift blocks the adapter

r[agent_harness_runtime.basalt_authority_adapter.compatibility.drift]
- GIVEN the Basalt or UCAN revision, schema, API, feature set, vendored source, policy identity, receipt shape, or license metadata differs from the reviewed cohort
- WHEN compatibility validation runs
- THEN adapter construction MUST fail with an unsupported-cohort or license-boundary diagnostic
- AND the default Animus graph MUST remain unaffected.

### Requirement: Basalt adapter verification includes positive and negative paths

r[agent_harness_runtime.basalt_authority_adapter.verification] The change MUST include positive and negative tests for generic provider replacement, agent-to-DID binding, typed policy projection, exact translation, raw UCAN enforcement, non-widening intersection, guarded dispatch, safe receipts, compatibility, AGPL isolation, default features, `no_std`, Wasm, runtime, Steel, TUI, unrelated host builds, component and facade inventories, and Cairn gates.

#### Scenario: Closeout evidence is bounded and complete

r[agent_harness_runtime.basalt_authority_adapter.verification.closeout]
- GIVEN maintainers intend to sync and archive the optional Basalt authority adapter
- WHEN closeout validation runs
- THEN evidence MUST name the exact Animus, Basalt, and UCAN cohorts plus every required focused, isolation, compatibility, license, workspace, Nickel, Nix, and Cairn result
- AND every unavailable or skipped result MUST record an exact blocker and next-best check without claiming global authorization, production readiness, effect safety, agent safety, or completion.
