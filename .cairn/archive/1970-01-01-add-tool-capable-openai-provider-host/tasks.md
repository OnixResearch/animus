# Tasks

## B. Cohort and projection

- [x] [serial] B1: Record the reviewed OpenAI Responses wire cohort, Prime Agent behavior, fixture provenance, and supported feature matrix. r[agent_harness_runtime.tool_capable_openai_host.verification]
- [x] [serial] B2: Extend typed Nickel and Rust capability metadata for the exact tool and stream cohort. r[agent_harness_runtime.tool_capable_openai_host.capabilities]
- [x] [depends:B2] B3: Implement pure request projection for tool declarations, prior response binding, and correlated tool results. r[agent_harness_runtime.tool_capable_openai_host.projection]

## S. Streaming and shell

- [x] [depends:B3] S1: Implement the bounded deterministic SSE parser through the existing canonical stream frame contract. r[agent_harness_runtime.tool_capable_openai_host.streaming]
- [x] [depends:S1] S2: Enforce model-call, provider-response, and tool-call correlation across continuations. r[agent_harness_runtime.tool_capable_openai_host.correlation]
- [x] [depends:S2] S3: Extend the explicit HTTPS shell without provider discovery, hidden retry, fallback, or ambient credentials. r[agent_harness_runtime.tool_capable_openai_host.authority]
- [x] [depends:S3] S4: Project stream progress and terminal outcomes into existing runtime and retry classifications. r[agent_harness_runtime.tool_capable_openai_host.streaming]

## T. Tests and closeout

- [x] [depends:S4] T1: Add literal positive fixtures for content, reasoning, tool calls, tool results, usage, and finish events through the production parser. r[agent_harness_runtime.tool_capable_openai_host.verification]
- [x] [depends:S4] T2: Add negative fixtures for malformed SSE, unsupported capabilities, lossy projection, stale identities, duplicate terminals, unsafe reasoning, cancellation, body overflow, and secret leaks. r[agent_harness_runtime.tool_capable_openai_host.verification]
- [x] [depends:T2] T3: Add a deterministic fake transport loop and a separately opt-in live smoke with safe evidence. r[agent_harness_runtime.tool_capable_openai_host.authority]
- [x] [depends:T3] T4: Update docs, conformance index, component metadata, focused rails, and close through relevant workspace and Cairn checks. r[agent_harness_runtime.tool_capable_openai_host.verification]
