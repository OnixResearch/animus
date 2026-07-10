# agent-harness-runtime Change Spec: Add an explicit tool executor kit

## Requirements

### Requirement: Tool executor kit contracts

r[agent_harness_runtime.tool_executor_kit.contracts] Animus MUST define an explicit host-injected tool executor kit covering tool catalogs, tool definitions, executor binding labels, executor service contracts, execution plans, outcome failure classes, and safe execution receipt drafts without shipping default product tools or granting ambient execution authority.

#### Scenario: Host advertises a tool without executing it

r[agent_harness_runtime.tool_executor_kit.contracts.metadata_only]
- GIVEN a host supplies a tool definition with name, schema identity, argument schema, capability labels, mutation posture, output policy, approval posture, trust status, provenance, and executor binding label
- WHEN Animus validates the definition
- THEN validation treats the definition as metadata only
- AND no shell command, filesystem mutation, network call, plugin launch, provider request, or process execution occurs from reusable crates.

### Requirement: Tool execution admission is pure and fail-closed

r[agent_harness_runtime.tool_executor_kit.admission] Tool execution MUST pass a pure admission gate over typed request, catalog, policy, capability, cancellation, timeout, mutation, and executor availability inputs before any host executor effect occurs, and non-admitted plans MUST fail closed without fallback execution.

#### Scenario: Missing executor blocks dispatch

r[agent_harness_runtime.tool_executor_kit.admission.missing_executor]
- GIVEN a valid-looking tool request names a tool whose executor binding is absent or unavailable
- WHEN the executor kit admission gate runs
- THEN admission returns a typed missing or unavailable executor denial
- AND no fallback shell command, plugin runtime, MCP dispatch, network call, provider call, filesystem scan, or process execution is attempted.

### Requirement: Tool execution receipts are safe

r[agent_harness_runtime.tool_executor_kit.receipts] Tool execution receipts MUST contain only bounded safe labels, status classes, failure classes, schema identities, capability summaries, mutation target summaries, counts, redaction markers, safe references, and BLAKE3 identities where content identity matters, and MUST reject raw credentials, private paths, raw private file bodies, unbounded stdout/stderr, provider payloads, and unbounded tool output.

#### Scenario: Unsafe receipt is rejected

r[agent_harness_runtime.tool_executor_kit.receipts.rejects_leaks]
- GIVEN a tool executor outcome attempts to persist a credential, raw private file body, absolute private path, provider payload, or unbounded stdout/stderr in a durable receipt
- WHEN receipt validation runs
- THEN validation rejects the receipt with a deterministic leak diagnostic
- AND no completed-effect evidence is emitted for the unsafe receipt.

### Requirement: Tool executor kit verification

r[agent_harness_runtime.tool_executor_kit.verification] The tool executor kit MUST include focused positive and negative tests for valid fake execution, missing executors, denied capabilities, malformed arguments, inactive or untrusted definitions, unsafe outputs, mutation conflicts, cancellation, timeout, fallback attempts, receipt leaks, catalog metadata, and composition conformance.

#### Scenario: Closeout evidence covers executor behavior

r[agent_harness_runtime.tool_executor_kit.verification.closeout]
- GIVEN maintainers intend to archive the tool executor kit change
- WHEN closeout validation runs
- THEN evidence names tool contract tests, runtime admission tests, fake executor composition tests, safe receipt checks, catalog checks, conformance checks, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
