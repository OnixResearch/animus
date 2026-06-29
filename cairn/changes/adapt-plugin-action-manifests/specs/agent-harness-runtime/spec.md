# agent-harness-runtime Change Spec: Plugin/action manifest adaptation

## Requirements

### Requirement: Manifest role declarations

r[agent_harness_runtime.plugin_action_manifests.manifest_roles] Animus MUST define data-only plugin descriptor roles for actions, event hooks, host-pane entrypoints, and link handlers without executing commands, opening panes, registering runtime actions dynamically, or requiring a specific manifest file format.

#### Scenario: Manifest roles validate as data

r[agent_harness_runtime.plugin_action_manifests.manifest_roles.validation]
- GIVEN a plugin descriptor declares actions, event hooks, pane entrypoints, and link handlers with stable IDs, titles, context labels, and command or artifact labels
- WHEN descriptor validation runs
- THEN the roles are accepted as data-only declarations
- AND duplicate IDs, missing required labels, unsafe command labels, and unsupported roles are rejected with typed diagnostics.

### Requirement: Compatibility gating

r[agent_harness_runtime.plugin_action_manifests.compatibility] Plugin manifests and entrypoints MUST declare or inherit minimum host capability/version and effective platform compatibility before host link or invocation planning treats them as available.

#### Scenario: Unsupported platform blocks invocation

r[agent_harness_runtime.plugin_action_manifests.compatibility.unsupported]
- GIVEN a plugin action is valid data but its effective platform set excludes the current host platform
- WHEN invocation planning evaluates that action
- THEN planning returns a typed platform-unsupported diagnostic
- AND no command descriptor is dispatched to a host executor.

### Requirement: Safe invocation context

r[agent_harness_runtime.plugin_action_manifests.invocation_context] Action, event, pane, and link-handler planning MUST use typed invocation context DTOs containing safe workspace, tab, pane, agent status, worktree/session provenance, request identity, plugin root/config/state labels, and source metadata while excluding raw prompts, transcripts, credentials, full private paths, unbounded logs, and provider payloads by default.

#### Scenario: Context injection rejects unsafe payloads

r[agent_harness_runtime.plugin_action_manifests.invocation_context.redacted]
- GIVEN an invocation context fixture contains a raw prompt, terminal transcript, credential-looking value, full private path, unbounded log, or provider payload
- WHEN context validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the unsafe context is not passed to invocation planning.

### Requirement: Registry operations are host-owned

r[agent_harness_runtime.plugin_action_manifests.host_registry] Link, unlink, enable, disable, list, action-list, invocation planning, event planning, pane-entrypoint planning, registry persistence, plugin logs, install source tracking, command execution, and pane opening MUST be host-owned operations outside reusable runtime core.

#### Scenario: Planning does not execute plugin code

r[agent_harness_runtime.plugin_action_manifests.host_registry.no_execution]
- GIVEN a validated plugin action descriptor and safe invocation context
- WHEN reusable invocation planning runs
- THEN it returns a typed plan for a host adapter to evaluate
- AND it does not persist registry state, install plugins, execute commands, open panes, read plugin storage, or weaken runtime policy.

### Requirement: Diagnostics distinguish warnings from blockers

r[agent_harness_runtime.plugin_action_manifests.diagnostics] Plugin manifest handling MUST distinguish link-time warnings from invocation-blocking failures for unknown events, disabled plugins, unknown entrypoints, unsupported platforms, newer host requirements, policy denials, malformed contexts, and malformed manifests.

#### Scenario: Disabled plugin fails closed

r[agent_harness_runtime.plugin_action_manifests.diagnostics.fail_closed]
- GIVEN a linked plugin is disabled by host registry state
- WHEN a caller requests one of its actions
- THEN invocation planning returns a typed disabled-plugin failure
- AND no host executor command, pane entrypoint, event hook, or link handler is dispatched.

### Requirement: Plugin manifest verification

r[agent_harness_runtime.plugin_action_manifests.verification] The adaptation MUST include focused positive and negative tests for manifest roles, compatibility gating, safe invocation contexts, host-owned registry operations, warnings, invocation blockers, disabled plugins, unknown events, unsafe command labels, and no-execution validation.

#### Scenario: Closeout evidence covers plugin manifest rails

r[agent_harness_runtime.plugin_action_manifests.verification.closeout]
- GIVEN maintainers intend to archive the plugin manifest adaptation
- WHEN closeout validation runs
- THEN evidence names focused manifest-role, compatibility, context-redaction, host-registry, diagnostic, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.
