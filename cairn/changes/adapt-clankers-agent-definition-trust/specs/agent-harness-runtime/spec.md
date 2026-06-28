# agent-harness-runtime Change Spec: Clankers agent definition trust adaptation

## Requirements

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
