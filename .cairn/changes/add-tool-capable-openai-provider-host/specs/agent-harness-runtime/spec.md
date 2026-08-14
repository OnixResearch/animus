# Agent Harness Runtime

## ADDED Requirements

### Requirement: OpenAI tool projection preserves canonical meaning

r[agent_harness_runtime.tool_capable_openai_host.projection] The non-default OpenAI host MUST project admitted canonical tool declarations, assistant tool calls, and correlated tool results without dropping required roles, schemas, identities, or output controls.

#### Scenario: The model requests steel_repl

r[agent_harness_runtime.tool_capable_openai_host.projection.steel_repl]
- GIVEN a canonical request declares the admitted `steel_repl` tool and its exact schema
- WHEN the OpenAI host projects the request
- THEN the wire request MUST preserve the declaration name, description, schema, and tool-selection policy
- AND no tool execution may occur inside the provider adapter.

#### Scenario: A required field is unsupported

r[agent_harness_runtime.tool_capable_openai_host.projection.lossy]
- GIVEN a canonical request requires a field that the reviewed OpenAI cohort cannot preserve
- WHEN request projection runs
- THEN projection MUST fail before network contact with a typed unsupported-feature outcome
- AND it MUST NOT silently remove or downgrade the field.

### Requirement: OpenAI streaming uses the production parser

r[agent_harness_runtime.tool_capable_openai_host.streaming] The host MUST parse bounded SSE chunks through one deterministic production entrypoint that emits ordered canonical content, reasoning, tool-call, usage, finish, cancellation, and failure frames.

#### Scenario: Tool arguments arrive in multiple events

r[agent_harness_runtime.tool_capable_openai_host.streaming.tool_delta]
- GIVEN a provider stream contains ordered argument fragments for one tool call
- WHEN the production parser processes the chunks
- THEN it MUST emit correlated ordered tool-call frames and one bounded completed call
- AND malformed, duplicate, or over-limit fragments MUST fail closed.

### Requirement: Provider and tool correlation is exact

r[agent_harness_runtime.tool_capable_openai_host.correlation] Provider continuation requests MUST match the Animus model-call identity, provider response identity, provider tool-call identity, committed assistant call, and supplied tool result before network contact.

#### Scenario: A tool result names another call

r[agent_harness_runtime.tool_capable_openai_host.correlation.mismatch]
- GIVEN a continuation carries a tool result for a stale or different provider call
- WHEN correlation admission runs
- THEN admission MUST reject the continuation before transport
- AND no result may enter another response chain.

### Requirement: Capability negotiation precedes contact

r[agent_harness_runtime.tool_capable_openai_host.capabilities] The host MUST negotiate requested tool, stream, reasoning, usage, role, and budget requirements against exact selected-model capability metadata before transport.

#### Scenario: Tool calls are not supported by the selected model

r[agent_harness_runtime.tool_capable_openai_host.capabilities.denied]
- GIVEN a request requires tool calls and selected-model metadata does not admit them
- WHEN capability negotiation runs
- THEN dispatch MUST fail closed before credential or network use
- AND the host MUST NOT switch models, endpoints, or providers automatically.

### Requirement: Network and credential authority stays in the host shell

r[agent_harness_runtime.tool_capable_openai_host.authority] Endpoint selection, credential values, HTTPS transport, deadlines, response bounds, cancellation, and failure observation MUST remain in the non-default authority-scoped host shell.

#### Scenario: No explicit credential is supplied

r[agent_harness_runtime.tool_capable_openai_host.authority.no_credential]
- GIVEN an admitted tool-capable request has no explicit credential value or approved credential service result
- WHEN the host prepares transport
- THEN it MUST return a typed missing-credential outcome
- AND it MUST NOT inspect ambient provider files, accounts, keychains, or environment values by default.

### Requirement: Tool-capable provider evidence is bounded

r[agent_harness_runtime.tool_capable_openai_host.verification] The host MUST include positive and negative production-parser fixtures, projection tests, correlation tests, capability tests, fake transport tests, authority tests, leak tests, dependency isolation, and separately classified optional live evidence.

#### Scenario: The provider change is ready to archive

r[agent_harness_runtime.tool_capable_openai_host.verification.closeout]
- GIVEN maintainers intend to sync and archive the tool-capable host
- WHEN focused and lifecycle validation runs
- THEN provider, projection, streaming, capability, runtime-event, retry, receipt, Cairn, and relevant Nix checks MUST pass
- AND live reliability, tool safety, task success, arbitrary endpoint support, broad provider parity, and production readiness MUST remain non-claims.
