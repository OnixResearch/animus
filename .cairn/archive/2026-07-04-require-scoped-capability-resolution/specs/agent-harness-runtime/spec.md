# agent-harness-runtime Change Spec: Scoped capability resolution

## Requirements

### Requirement: Required capability identity

r[agent_harness_runtime.scoped_capabilities.required_identity] Model and tool dispatch MUST declare the specific required capability identity, including stable capability label, service kind, and optional safe scope, before runtime admits an adapter call.

#### Scenario: Capability identity validates before dispatch

r[agent_harness_runtime.scoped_capabilities.required_identity.validation]
- GIVEN runtime prepares a model or tool dispatch request
- WHEN required capability identity validation runs
- THEN default and explicit scoped capability labels are accepted with supported service kinds and safe scope labels
- AND empty labels, unsupported service kinds, unsafe scope text, and duplicate labels fail before adapter dispatch.

### Requirement: Exact registry resolution

r[agent_harness_runtime.scoped_capabilities.exact_resolution] Runtime dispatch MUST resolve the declared capability label and service kind exactly, and MUST NOT treat the presence of any service with the same broad kind as sufficient authority.

#### Scenario: Unrelated service cannot satisfy scoped request

r[agent_harness_runtime.scoped_capabilities.exact_resolution.fail_closed]
- GIVEN a host registers a model or tool service that does not match the required capability label, service kind, or safe scope
- WHEN runtime attempts dispatch
- THEN dispatch fails closed with a typed missing or mismatched capability diagnostic
- AND the unrelated service is not called, no fallback service is discovered, and no completed receipt is produced.

### Requirement: Compatibility default capabilities

r[agent_harness_runtime.scoped_capabilities.compatibility_defaults] Existing model/tool runtime constructors MUST remain available as thin wrappers that bind adapters to documented default capability labels and no unsafe scope.

#### Scenario: Default constructor behavior remains equivalent

r[agent_harness_runtime.scoped_capabilities.compatibility_defaults.equivalent]
- GIVEN a runtime loop uses the existing model/tool constructor path and another loop uses explicit default capability descriptors
- WHEN both loops receive equivalent fabric decisions and adapter responses
- THEN they produce equivalent reports and events
- AND both paths perform exact capability resolution without ambient provider, filesystem, process, network, credential, or plugin fallback.

### Requirement: Scoped capability verification

r[agent_harness_runtime.scoped_capabilities.verification] The scoped capability resolution change MUST include focused positive and negative tests for identity validation, exact resolution, compatibility defaults, unrelated-service denial, no ambient fallback, facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers scoped capability resolution

r[agent_harness_runtime.scoped_capabilities.verification.closeout]
- GIVEN maintainers intend to archive scoped capability resolution
- WHEN closeout validation runs
- THEN evidence names identity validation, exact resolution, compatibility defaults, fail-closed, no-fallback, inventory, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
