# agent-harness-runtime Change Spec: Hermes plugin and tool manifest adaptation

## Requirements

### Requirement: Typed plugin toolset manifests

r[agent_harness_runtime.hermes_plugin_manifests.toolset] Animus MUST define typed plugin/toolset manifest descriptors with toolset name, version, kind, provided tool names, descriptions, parameter schema identities, result contract identities, capability labels, provenance, and trust status.

#### Scenario: Valid plugin manifest advertises tools

r[agent_harness_runtime.hermes_plugin_manifests.toolset.valid]
- GIVEN a host supplies a valid plugin manifest for a review-gate-style toolset
- WHEN manifest validation and tool definition construction run
- THEN every provided tool becomes an inactive or active typed tool definition according to trust and executor availability
- AND prompt metadata includes only validated names, descriptions, capability labels, and safe provenance labels.

### Requirement: Manifest validation rejects ambiguous tools

r[agent_harness_runtime.hermes_plugin_manifests.validation] Animus MUST reject plugin manifests with missing tool descriptions, invalid names, unsupported kinds, missing capability labels, duplicate provided tools, unsafe provenance, malformed schema identities, or malformed result contract identities.

#### Scenario: Duplicate tool names fail closed

r[agent_harness_runtime.hermes_plugin_manifests.validation.duplicate]
- GIVEN two plugin manifests provide the same public tool name without an explicit host policy override
- WHEN tool registry construction runs
- THEN registry construction returns a typed duplicate-tool diagnostic
- AND neither duplicate is silently selected by map insertion order.

### Requirement: Plugin execution requires host-injected executor

r[agent_harness_runtime.hermes_plugin_manifests.executor_boundary] Plugin manifests MUST NOT cause reusable Animus runtime code to import plugin modules, run scripts, inspect installation directories, or shell out. Tool execution MUST require an injected executor matching the validated tool definition.

#### Scenario: Manifest tool without executor is unavailable

r[agent_harness_runtime.hermes_plugin_manifests.executor_boundary.missing]
- GIVEN a trusted plugin manifest advertises a tool but the host injects no executor for that tool
- WHEN the loop requests the tool
- THEN runtime reports a missing-executor failure class
- AND no script, shell command, module import, or provider fallback is attempted.

### Requirement: Capability labels separate read-only and mutating actions

r[agent_harness_runtime.hermes_plugin_manifests.capabilities] Every manifest-provided tool MUST declare side-effect capability labels sufficient to distinguish read-only inspection, dry-run planning, explicit metrics writes, filesystem mutation, process execution, and network access when applicable.

#### Scenario: Metrics write is explicit

r[agent_harness_runtime.hermes_plugin_manifests.capabilities.metrics_write]
- GIVEN a review metrics toolset includes both summary and record actions
- WHEN tool metadata is built
- THEN summary actions are labeled read-only
- AND record actions carry an explicit metrics-write or append-only mutation capability label.

### Requirement: Plugin manifest adaptation verification

r[agent_harness_runtime.hermes_plugin_manifests.verification] The plugin manifest adaptation MUST include focused tests for valid manifests, malformed manifest rejection, duplicate tool diagnostics, capability-label prompt construction, missing executor failure, untrusted plugin omission, and host-injected executor dispatch.

#### Scenario: Closeout evidence covers manifest and executor rails

r[agent_harness_runtime.hermes_plugin_manifests.verification.closeout]
- GIVEN maintainers intend to archive the Hermes plugin manifest adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest tests, executor-boundary tests, prompt metadata tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
