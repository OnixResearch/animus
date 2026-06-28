# agent-harness-runtime Change Spec: Claude Code workflow command adaptation

## Requirements

### Requirement: Bounded workflow command reference

r[agent_harness_runtime.claude_code_commands.scope] Animus MUST treat Claude Code slash commands as a bounded reference for named workflow invocation, and MUST NOT execute command content as shell scripts by default, open pull requests, or treat command prompt text as a substitute for runtime policy enforcement.

#### Scenario: Command text cannot bypass policy

r[agent_harness_runtime.claude_code_commands.scope.no_policy_bypass]
- GIVEN a workflow command asks for a mutating action in its prompt text
- WHEN runtime expands and evaluates the command
- THEN the resulting guarded action still passes tool, permission, and capability policy checks
- AND no shell, VCS, network, or provider side effect occurs solely because command text requested it.

### Requirement: Typed workflow command manifests

r[agent_harness_runtime.claude_code_commands.manifest] Workflow commands MUST be represented as typed manifests with validated name, summary, input schema, capability requirements, side-effect posture, retrieval budget reference, validation gates, output contract, and source/provenance metadata.

#### Scenario: Unsafe manifest is rejected

r[agent_harness_runtime.claude_code_commands.manifest.invalid]
- GIVEN a command manifest has an invalid name, missing input schema, missing side-effect posture, undeclared capability, or absent output contract
- WHEN command manifest validation runs
- THEN validation returns typed diagnostics
- AND the command is not advertised or invokable.

### Requirement: Deterministic command registry

r[agent_harness_runtime.claude_code_commands.registry] Command registry construction MUST deterministically resolve or reject duplicate command names across host, user, and trusted project sources according to explicit policy.

#### Scenario: Duplicate command name reports diagnostic

r[agent_harness_runtime.claude_code_commands.registry.duplicate]
- GIVEN two trusted command manifests advertise the same invocation name
- WHEN registry construction runs
- THEN policy selects a deterministic winner or disables the duplicate name
- AND diagnostics include source IDs without exposing unsafe paths or raw command bodies.

### Requirement: Pure argument binding before expansion

r[agent_harness_runtime.claude_code_commands.binding] Workflow invocation arguments MUST be bound and validated against the command input schema before prompt expansion, tool dispatch, or workflow state mutation.

#### Scenario: Malformed command arguments stop expansion

r[agent_harness_runtime.claude_code_commands.binding.invalid]
- GIVEN a user invokes a workflow command with missing, unknown, or malformed arguments
- WHEN argument binding runs
- THEN runtime returns a typed binding error
- AND no workflow prompt, tool call, shell action, network call, or VCS mutation is started.

### Requirement: Contract-bound workflow outputs

r[agent_harness_runtime.claude_code_commands.outputs] Workflow commands MUST declare expected output contracts and validation evidence so hosts can distinguish explanation, review, patch summary, validation report, commit-message proposal, and structured DTO outputs.

#### Scenario: Output contract mismatch is reported

r[agent_harness_runtime.claude_code_commands.outputs.mismatch]
- GIVEN a workflow command declares a structured validation-report output
- WHEN the model or tool result cannot be validated against that output contract
- THEN runtime reports an output-contract mismatch
- AND downstream automation does not treat the malformed result as a successful workflow completion.
