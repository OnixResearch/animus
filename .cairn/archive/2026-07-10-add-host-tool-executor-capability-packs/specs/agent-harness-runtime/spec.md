# agent-harness-runtime Change Spec: Host tool executor capability packs

## Requirements

### Requirement: Tool executor packs describe admitted host authority

r[agent_harness_runtime.tool_executor_packs.descriptor] Animus MUST represent host-owned tool executor packs with typed descriptors that bind tool definitions, schema identities, executor service labels, exact capability requirements, output policies, mutation policies, approval posture, and prompt-exposure state.

#### Scenario: Read-only executor pack is declared

r[agent_harness_runtime.tool_executor_packs.descriptor.read_only]
- GIVEN a host supplies a read-only executor pack for a known tool definition
- WHEN descriptor validation runs
- THEN Animus accepts safe tool name, schema identity, executor label, required capability, bounded output policy, and non-mutating policy
- AND it does not import tool implementation code, inspect plugins, or grant filesystem, process, network, or shell authority.

### Requirement: Executor pack admission precedes prompt exposure and dispatch

r[agent_harness_runtime.tool_executor_packs.admission] Tool prompt exposure and dispatch MUST require active trusted definition metadata, valid schema identity, executor service readiness, exact declared capability labels, approval posture, output policy, and mutation policy before an executor is called.

#### Scenario: Executor-less tool is not dispatch-ready

r[agent_harness_runtime.tool_executor_packs.admission.missing_executor]
- GIVEN a trusted active tool definition exists
- AND no executor pack is injected for that tool
- WHEN prompt exposure and dispatch admission run
- THEN dispatch fails with a missing-executor class
- AND no fallback executor, shell command, plugin process, network call, or provider call is attempted.

### Requirement: Fake executor packs provide deterministic first evidence

r[agent_harness_runtime.tool_executor_packs.fake_executors] The first tool executor pack implementation SHOULD include deterministic fake executors that exercise read-only, bounded-output, and denied-dispatch paths without host effects.

#### Scenario: Fake executor returns bounded output

r[agent_harness_runtime.tool_executor_packs.fake_executors.bounded_output]
- GIVEN an admitted fake executor pack receives valid prepared arguments
- WHEN the fake executor returns output
- THEN Animus bounds or hashes the output according to policy
- AND the response records safe redaction and output status metadata.

### Requirement: Executor outcomes produce safe receipts

r[agent_harness_runtime.tool_executor_packs.receipts] Tool executor pack outcomes MUST produce safe receipts for completed, denied, missing, unavailable, timed-out, cancelled, malformed-output, unsafe-reference, unbounded-output, mutation-conflict, and leak-rejected cases using safe labels, counts, BLAKE3 identities, failure classes, and redaction markers only.

#### Scenario: Unsafe output reference is rejected

r[agent_harness_runtime.tool_executor_packs.receipts.unsafe_reference]
- GIVEN an executor returns a private path, URL, raw handle, or unbounded output reference not admitted by output policy
- WHEN response and receipt validation run
- THEN Animus returns an unsafe-reference or unbounded-output failure class
- AND durable evidence omits the unsafe reference body.

### Requirement: Tool pack docs preserve non-default product boundary

r[agent_harness_runtime.tool_executor_packs.docs] Documentation for executor packs MUST state that reusable crates expose contracts only and that packaged tools, plugin execution, filesystem/process/network/shell authority, product catalogs, and global tool discovery remain host-owned non-default behavior.

#### Scenario: Capability pack example scopes authority

r[agent_harness_runtime.tool_executor_packs.docs.scope]
- GIVEN a user reads a capability-pack example
- WHEN the example lists supported tools
- THEN it also lists exact capability labels, output policy, mutation policy, and omitted product behavior
- AND it does not imply default product tools ship with reusable Animus crates.

### Requirement: Tool executor pack verification

r[agent_harness_runtime.tool_executor_packs.verification] The executor pack change MUST include positive and negative tests for descriptor validation, admission, fake executor success, missing executor, capability gaps, denied approval, malformed arguments, unsafe references, unbounded output, mutation conflict, timeout, cancellation, and receipt leak rejection before archive.

#### Scenario: Closeout evidence names tool non-claims

r[agent_harness_runtime.tool_executor_packs.verification.closeout]
- GIVEN maintainers prepare to archive executor pack work
- WHEN closeout validation runs
- THEN evidence names descriptor DTOs, admission, fake executors, receipts, docs, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records product tool catalogs, plugin execution, global discovery, shell/process/network/filesystem authority, and production readiness as non-claims.
