# agent-harness-runtime Change Spec: Breaking tool calling replacement

## Requirements

### Requirement: Current-shape tool calls only

r[agent_harness_runtime.tool_calling_breaking.current_shape] Runtime tool calls MUST use a current structured contract carrying tool name, schema identity, validated arguments, correlation identity, output policy, declared mutation targets, required capability labels, and approval posture before dispatch, and MUST NOT admit legacy raw string input or raw string output as the runtime tool-call contract.

#### Scenario: Legacy raw tool input is rejected

r[agent_harness_runtime.tool_calling_breaking.current_shape.rejects_legacy]
- GIVEN a loop decision, resumed ledger entry, Preserves payload, or headless request supplies only a tool name plus raw input string
- WHEN runtime validates the tool call for dispatch
- THEN validation returns a typed current-shape-required diagnostic
- AND no tool executor, shell command, provider call, plugin runtime, filesystem scan, credential read, network connection, mutation coordinator, or fallback compatibility preparer is invoked.

### Requirement: Catalog-backed tool admission

r[agent_harness_runtime.tool_calling_breaking.catalog_admission] Runtime tool prompt construction and dispatch admission MUST derive from active trusted tool definitions supplied by an Animus-owned catalog, manifest projection, or host-injected definition set, and MUST reject tools that are unknown, inactive, untrusted, schema-invalid, executor-less, or capability-denied before execution.

#### Scenario: Exact capability gates each tool

r[agent_harness_runtime.tool_calling_breaking.catalog_admission.exact_capability]
- GIVEN an active tool definition declares required capability labels and a host capability registry
- WHEN runtime plans dispatch for that tool
- THEN every required capability identity is resolved exactly with service kind and safe scope before execution
- AND a broad default tool capability does not authorize the call when the per-tool capability is absent.

### Requirement: Pure tool dispatch plan

r[agent_harness_runtime.tool_calling_breaking.dispatch_plan] Tool dispatch MUST have a pure planning DTO that records definition identity, request contract identity, schema identity, validated argument status, required capabilities, approval decision, output policy, mutation-window plan, cancellation posture, timeout posture, and fallback policy before any executor effect.

#### Scenario: Denied plan fails closed before execution

r[agent_harness_runtime.tool_calling_breaking.dispatch_plan.fail_closed]
- GIVEN a tool dispatch plan is denied because request shape, catalog membership, executor presence, capability, approval, output policy, mutation posture, cancellation, timeout, or fallback policy fails
- WHEN runtime processes the tool decision
- THEN the tool executor is not called
- AND runtime returns a typed fail-closed outcome without completed receipts, provider discovery, credential reads, filesystem scans, process spawning, plugin loading, network fallback, or raw output events.

### Requirement: Bounded tool result contract

r[agent_harness_runtime.tool_calling_breaking.bounded_result] Completed tool execution MUST return a bounded runtime-visible result containing model-visible text, completion or truncation status, byte count, character count, line count, redaction class, BLAKE3 content hash when required, and optional host-owned full-output handle, and MUST NOT store raw full output bodies in runtime state, runtime events, reports, receipts, Preserves event payloads, or headless projections.

#### Scenario: Raw tool body cannot enter runtime evidence

r[agent_harness_runtime.tool_calling_breaking.bounded_result.no_raw_body]
- GIVEN a tool executor produces an oversized body, unsafe full-output reference, NUL-containing output, or secret-adjacent output body
- WHEN runtime validates the completed tool result
- THEN it either returns a bounded result with safe visible text and BLAKE3 identity metadata or rejects the result with a typed malformed or unsafe-output outcome
- AND raw full output, private absolute paths, credentials, provider payloads, and unbounded stdout or stderr are not embedded in model-visible context or evidence.

### Requirement: Typed tool execution outcomes

r[agent_harness_runtime.tool_calling_breaking.typed_outcomes] Tool execution services MUST return typed completed or fail-closed outcomes for admitted calls, preserving invalid input, missing executor, unavailable service, denied capability, approval denial, cancelled, timed out, malformed output, unsafe output reference, unbounded oversized output, mutation conflict, and execution failure as distinct stable classes.

#### Scenario: Failure class survives runtime projection

r[agent_harness_runtime.tool_calling_breaking.typed_outcomes.projection]
- GIVEN an admitted tool executor reports denied, unavailable, cancelled, timed-out, malformed-output, unsafe-reference, unbounded-output, or execution-failure status
- WHEN runtime projects the result into events, reports, receipts, Preserves, headless, or TUI surfaces
- THEN the stable failure class is preserved with bounded safe metadata
- AND runtime does not collapse the failure into an ambiguous success, raw prose-only error, or completed tool response.

### Requirement: Mutation-window dispatch scheduler

r[agent_harness_runtime.tool_calling_breaking.mutation_scheduler] Tool dispatch coordination MUST evaluate declared opaque mutation targets before execution, serialize conflicting mutation windows deterministically, and allow independent read-only or non-overlapping tool calls only when their admitted plans cannot race on declared mutation targets.

#### Scenario: Conflicting mutation targets serialize

r[agent_harness_runtime.tool_calling_breaking.mutation_scheduler.conflict]
- GIVEN two admitted tool calls declare the same mutation target and a third admitted tool call is read-only or targets a distinct resource key
- WHEN runtime asks the host-owned coordinator for dispatch windows
- THEN the conflicting mutation calls are serialized in deterministic plan order
- AND the read-only or non-overlapping call may be admitted independently without observing a stale state caused by the serialized mutation pair.

### Requirement: Breaking tool-call verification

r[agent_harness_runtime.tool_calling_breaking.verification] The breaking tool-call replacement MUST update adapter contracts, runtime dispatch, runtime event/report projections, Preserves/headless/TUI surfaces, lifecycle docs, focused rails, and tests so the current-only structured contract is the only accepted tool-call shape.

#### Scenario: Closeout proves legacy path removal

r[agent_harness_runtime.tool_calling_breaking.verification.closeout]
- GIVEN maintainers intend to archive the breaking tool-call replacement
- WHEN closeout validation runs
- THEN evidence names positive structured-call coverage and negative coverage for legacy raw payloads, unknown tools, inactive tools, missing executors, denied capabilities, malformed arguments, unsafe output, unbounded output, typed outcome projection, and mutation conflicts
- AND any skipped rail includes a concrete blocker and next-best validation command.
