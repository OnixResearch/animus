# agent-harness-runtime Change Spec: Claude Code lifecycle hook adaptation

## Requirements

### Requirement: Bounded lifecycle hook reference

r[agent_harness_runtime.claude_code_hooks.scope] Animus MUST treat Claude Code hooks as a bounded reference for lifecycle extension points, and MUST NOT adopt arbitrary shell hook execution, hook-file authority, or unvalidated hook-driven state mutation in reusable runtime crates.

#### Scenario: Hook cannot bypass action policy

r[agent_harness_runtime.claude_code_hooks.scope.no_bypass]
- GIVEN a pre-action hook requests a broader filesystem, exec, network, model, Steel, or tool action
- WHEN runtime evaluates the hook outcome
- THEN the proposed action must pass normal DTO validation, permission checks, and capability authorization
- AND dispatch is denied if those checks fail.

### Requirement: Stable hook point vocabulary

r[agent_harness_runtime.claude_code_hooks.points] Animus hook descriptors MUST use a stable hook-point vocabulary for session lifecycle, prompt assembly, model request/response, tool pre/post, mutation pre/post, validation, receipt emission, and workflow completion events.

#### Scenario: Unknown hook point fails validation

r[agent_harness_runtime.claude_code_hooks.points.unknown]
- GIVEN a hook descriptor names an unsupported hook point
- WHEN hook registry validation runs
- THEN validation returns a typed unknown-hook-point diagnostic
- AND the hook is not registered.

### Requirement: Safe hook event payloads

r[agent_harness_runtime.claude_code_hooks.payloads] Hook event payloads MUST expose bounded safe metadata such as IDs, classes, redacted resource labels, sizes, hashes, status, and summaries, and MUST NOT include raw prompts, credentials, provider payloads, raw tool bodies, full paths, or unbounded outputs by default.

#### Scenario: Tool post-hook sees bounded result metadata

r[agent_harness_runtime.claude_code_hooks.payloads.bounded]
- GIVEN a tool call produced a large result
- WHEN a tool post-hook request is prepared
- THEN the hook payload includes truncation status, sizes, safe labels, and BLAKE3 hashes where needed
- AND the raw full result body is absent unless a host-specific policy explicitly grants a separate safe handle.

### Requirement: Host-owned hook execution

r[agent_harness_runtime.claude_code_hooks.execution] Hook execution MUST be performed through injected host adapters with declared capabilities, policy-gated execution, typed unavailable-service failures, and no process, filesystem, network, clock, or shell behavior in reusable core crates.

#### Scenario: Missing hook service fails safely

r[agent_harness_runtime.claude_code_hooks.execution.missing_service]
- GIVEN policy requires a pre-tool hook before dispatch
- WHEN no hook service is injected
- THEN runtime reports a required-hook-unavailable failure
- AND the guarded action is not dispatched.

### Requirement: Hook outcomes require revalidation

r[agent_harness_runtime.claude_code_hooks.outcomes] Hook outcomes MAY observe, annotate, deny, or request revalidation, but any action or payload modification proposed by a hook MUST pass normal runtime validation and authorization before use.

#### Scenario: Malformed hook modification is rejected

r[agent_harness_runtime.claude_code_hooks.outcomes.invalid_modification]
- GIVEN a hook returns a malformed or unauthorized modified tool request
- WHEN runtime processes the hook outcome
- THEN runtime rejects the modification with typed diagnostics
- AND the original guarded action is denied or revalidated according to policy without using malformed data.
