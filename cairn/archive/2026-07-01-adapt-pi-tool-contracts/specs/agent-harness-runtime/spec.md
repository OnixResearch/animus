# agent-harness-runtime Change Spec: Pi tool contract adaptation

## Requirements

### Requirement: Rich tool definition metadata

r[agent_harness_runtime.pi_tool_contracts.definition_metadata] Animus tool definitions MUST expose validated name, label, description, parameter schema identity, prompt snippet, prompt guidelines, source/provenance metadata, capability labels, and result metadata shape without embedding concrete tool execution code in the reusable runtime.

#### Scenario: Tool metadata enters prompt construction safely

r[agent_harness_runtime.pi_tool_contracts.definition_metadata.prompt]
- GIVEN a registered tool with prompt snippet and tool-specific guidelines
- WHEN runtime builds model-facing tool context
- THEN only active tool metadata is included
- AND each guideline remains attributable to the tool name or capability it describes.

### Requirement: Argument preparation and strict validation

r[agent_harness_runtime.pi_tool_contracts.argument_validation] Tool-call arguments MUST pass optional pure compatibility preparation before strict validation against the current public parameter contract.

#### Scenario: Legacy arguments normalize before validation

r[agent_harness_runtime.pi_tool_contracts.argument_validation.compat]
- GIVEN a resumed ledger contains an older accepted tool-call argument shape
- WHEN the tool definition provides a compatibility preparer
- THEN runtime validates the prepared current-shape arguments
- AND unknown or malformed fields after preparation fail before tool execution.

### Requirement: Bounded tool output contract

r[agent_harness_runtime.pi_tool_contracts.output_bounds] Tool results MUST provide bounded model-visible output and safe metadata for truncated or full outputs, including sizes, truncation status, and BLAKE3 content hashes when full output identity is needed.

#### Scenario: Oversized tool output is truncated

r[agent_harness_runtime.pi_tool_contracts.output_bounds.truncated]
- GIVEN a tool executor returns output larger than the configured visible-output budget
- WHEN runtime records the result
- THEN model-visible content is truncated according to policy
- AND the result metadata records bounded sizes, truncation status, and a BLAKE3 hash without embedding the full raw body.

### Requirement: Resource mutation serialization

r[agent_harness_runtime.pi_tool_contracts.mutation_queue] Tools that mutate shared resources MUST declare mutation targets so a host-owned mutation coordinator can serialize conflicting mutation windows and avoid lost updates.

#### Scenario: Conflicting mutations serialize

r[agent_harness_runtime.pi_tool_contracts.mutation_queue.conflict]
- GIVEN two parallel tool calls declare the same mutation target
- WHEN runtime dispatches the calls through a mutation coordinator
- THEN the mutation windows execute in a deterministic serialized order
- AND neither call observes a stale pre-mutation state caused by the other call racing.

### Requirement: Typed tool failure classes

r[agent_harness_runtime.pi_tool_contracts.failure_classes] Tool dispatch MUST report typed failures for invalid input, missing executor, unavailable service, capability denial, cancellation, malformed output, oversized unbounded output, and execution failure.

#### Scenario: Missing executor fails before execution

r[agent_harness_runtime.pi_tool_contracts.failure_classes.missing_executor]
- GIVEN a loop decision requests a known tool with no injected executor
- WHEN runtime evaluates the tool dispatch
- THEN runtime emits a missing-executor failure class
- AND no fallback process, shell command, or provider call is attempted.
