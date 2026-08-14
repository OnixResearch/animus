# Proposal: Add a tool-capable OpenAI provider host

## Why

The live OpenAI-compatible host supports one non-streaming text response. It rejects tools, tool correlation, images, and structured output before contact.

A Prime Agent-style loop needs the provider to request `steel_repl`, receive the correlated tool result, and continue until a final response.

Animus already has canonical tool declarations, model stream frames, provider projection fixtures, retry policy, and a narrow HTTPS host. It lacks one concrete tool-capable provider adapter.

## What Changes

- Extend the non-default OpenAI Responses host with an exact tool-capable request and response cohort. r[agent_harness_runtime.tool_capable_openai_host.projection]
- Add production SSE parsing for content, reasoning, tool-call, usage, finish, cancellation, and failure frames. r[agent_harness_runtime.tool_capable_openai_host.streaming]
- Preserve provider call and tool call identities across provider continuation requests. r[agent_harness_runtime.tool_capable_openai_host.correlation]
- Negotiate every required capability before network contact and reject lossy projection. r[agent_harness_runtime.tool_capable_openai_host.capabilities]
- Keep credentials, endpoint selection, transport, retry, and cancellation in the authority-scoped shell. r[agent_harness_runtime.tool_capable_openai_host.authority]
- Add literal positive and negative fixtures plus separately scoped optional live evidence. r[agent_harness_runtime.tool_capable_openai_host.verification]

## Prerequisites

- `agent_harness_runtime.live_openai_provider_host.boundary`
- `agent_harness_runtime.model_streaming.adapter_contract`
- `agent_harness_runtime.provider_streaming.parser_entrypoint`
- `agent_harness_runtime.model_capability_matrix.negotiation`
- `agent_harness_runtime.provider_projection_conformance.harness`
- `agent_harness_runtime.coding_session_host.durable_boundaries`

## Impact

- **Crates**: `animus-std-openai`, canonical provider adapters, runtime event projection, and the root feature surface.
- **Wire scope**: one reviewed OpenAI Responses API cohort with `steel_repl` tool declarations and results.
- **Runtime**: the adapter emits canonical stream frames. The coding-session shell owns the tool loop.
- **Evidence**: fixture support and live evidence remain separate claims.

## Out of Scope

- Provider discovery, OAuth, automatic credential lookup, arbitrary OpenAI-compatible endpoints, hidden retry, automatic fallback, images, structured output, or broad provider parity.
- Tool execution, Steel evaluation, task correctness, provider reliability, or production readiness.

## Affected Specs

- `agent-harness-runtime`: OpenAI tool projection, streaming, correlation, capability gates, authority, and verification.
