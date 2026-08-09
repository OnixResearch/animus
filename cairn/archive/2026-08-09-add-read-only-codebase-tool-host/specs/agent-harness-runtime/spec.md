# agent-harness-runtime Change Spec: Read-only codebase tool host

## ADDED Requirements

### Requirement: Tool executor effects bind exact request values

r[agent_harness_runtime.read_only_codebase_tool.request_binding] The executor kit MUST bind each admitted plan to the exact current tool request values and MUST reject a substituted request before calling the injected service.

#### Scenario: Matching request reaches the executor

r[agent_harness_runtime.read_only_codebase_tool.request_binding.match]
- GIVEN a current request and a plan derived from that exact request
- WHEN executor dispatch validates the binding
- THEN it calls only the selected executor service
- AND raw argument values remain absent from durable receipts.

#### Scenario: Substituted value fails before effects

r[agent_harness_runtime.read_only_codebase_tool.request_binding.substitution]
- GIVEN an admitted plan and a different request with matching names but different values
- WHEN executor dispatch validates the binding
- THEN it returns a typed request-binding failure
- AND it does not call the service.

### Requirement: Tool execution returns bounded output and safe evidence

r[agent_harness_runtime.read_only_codebase_tool.outcome] Successful executor-kit dispatch MUST return the existing bounded model-visible response together with a metadata-only execution receipt.

#### Scenario: Raw output is bounded once

r[agent_harness_runtime.read_only_codebase_tool.outcome.bounded]
- GIVEN an admitted executor returns finite text
- WHEN the kit normalizes the output
- THEN the response contains only the admitted visible prefix
- AND the receipt contains counts and a BLAKE3 identity without the output body.

#### Scenario: Unsafe or oversized output fails closed

r[agent_harness_runtime.read_only_codebase_tool.outcome.failure]
- GIVEN an executor returns unsafe metadata or bytes beyond the admitted raw limit
- WHEN output normalization runs
- THEN it returns a typed failure
- AND it returns neither a response nor a success receipt.

### Requirement: Codebase query authority is explicit and read-only

r[agent_harness_runtime.read_only_codebase_tool.host] Animus MUST expose the concrete codebase query executor only through a non-default host feature over an explicitly constructed capability-root graph.

#### Scenario: Admitted query returns syntactic facts

r[agent_harness_runtime.read_only_codebase_tool.host.query]
- GIVEN an explicit capability root, admitted relative files, a fresh graph, and an exact query request
- WHEN the selected executor runs
- THEN it returns bounded sorted relative-path and byte-span facts
- AND it performs no mutation, process, network, provider, plugin, or discovery effect.

#### Scenario: Invalid host input fails closed

r[agent_harness_runtime.read_only_codebase_tool.host.failure]
- GIVEN an unsafe path, stale graph, malformed kind, invalid result limit, wrong tool, or wrong schema
- WHEN planning or execution runs
- THEN it returns a typed failure before completion evidence
- AND it performs no fallback.

### Requirement: Packaged codebase-tool smoke is bounded

r[agent_harness_runtime.read_only_codebase_tool.smoke] Animus MUST provide a non-default packaged smoke that requires an explicit root, relative file list, and query arguments and emits metadata-only completion evidence.

#### Scenario: Explicit fixture smoke passes

r[agent_harness_runtime.read_only_codebase_tool.smoke.pass]
- GIVEN an operator supplies an explicit root and admitted Rust files
- WHEN the packaged smoke executes one query through the production executor kit
- THEN it emits a bounded receipt summary
- AND it does not print source bodies, absolute paths, prompts, credentials, or provider payloads.

### Requirement: Read-only codebase tool verification covers denial paths

r[agent_harness_runtime.read_only_codebase_tool.verification] The change MUST include positive and negative tests for exact request binding, capability-root admission, query decoding, stale indexes, output bounds, receipt safety, dependency isolation, and packaged smoke claims.

#### Scenario: Focused rail closes the change

r[agent_harness_runtime.read_only_codebase_tool.verification.closeout]
- GIVEN implementation, tests, docs, and lifecycle artifacts are complete
- WHEN the focused Rust and Nix rails and Cairn gates run
- THEN they pass deterministically without network or provider credentials
- AND live-provider, semantic-analysis, mutation, and production claims remain explicit non-claims.
