## Why

Animus validates ACP descriptors, tool pass-through ceilings, session correlation, and offline external-subagent conformance, but it intentionally does not run a live ACP transport. The coding-session facade and durable authority-profile work provide the missing product seam for exposing Animus itself as an ACP agent to an IDE or editor without moving protocol, terminal, filesystem, credential, or process authority into reusable crates.

Grok Build demonstrates that a real ACP stdio host is materially more useful than descriptor-only compatibility. Animus should adapt that operational mechanism through a focused optional host crate, use the existing facade as the sole session authority, and defer WebSocket or other remote transports until they pass separate remote-admission and transport evidence.

## What Changes

- Add a non-default ACP stdio host boundary with an exact pinned protocol cohort and no default dependency in reusable crates. r[agent_harness_runtime.acp_stdio_host.boundary]
- Define a deterministic protocol/session reducer over already-decoded ACP messages, facade state observations, and typed effect outcomes. r[agent_harness_runtime.acp_stdio_host.protocol_core]
- Keep stdin/stdout framing, process lifecycle, buffering, deadlines, cancellation signals, and stderr diagnostics in a thin imperative shell. r[agent_harness_runtime.acp_stdio_host.transport]
- Map ACP initialize, session creation/load, prompt, cancellation, and supported updates to the coding-session facade without creating a second transcript or run-state owner. r[agent_harness_runtime.acp_stdio_host.session_bridge]
- Route ACP permission and tool requests through existing permission, confirmation, capability, sandbox, and tool pass-through intersections. r[agent_harness_runtime.acp_stdio_host.permission_bridge]
- Project bounded facade runtime events into ordered ACP updates with backpressure and hidden-reasoning/redaction rules. r[agent_harness_runtime.acp_stdio_host.events]
- Bind durable ACP resume to session authority-profile and store validation before continuation. r[agent_harness_runtime.acp_stdio_host.resume]
- Emit metadata-only transport/session/permission receipts and add fixture-client conformance scenarios. r[agent_harness_runtime.acp_stdio_host.receipts] r[agent_harness_runtime.acp_stdio_host.verification]

## Impact

- **Reusable crates**: gain only narrowly required transport-neutral protocol mapping DTOs or pure validation; they do not gain stdin/stdout, async runtime, process, socket, filesystem, terminal, or credential authority.
- **Optional host crate**: owns ACP protocol dependency, stdio transport, bounded codec/buffers, facade wiring, cancellation plumbing, and safe diagnostics.
- **Session lifecycle**: remains authoritative in the coding-session facade and durable store; ACP correlation IDs are transport metadata, not replacement session identity.
- **Security**: ACP client requests cannot widen Animus authority or bypass confirmation, tool admission, sandboxing, profile resume checks, or receipt redaction.
- **Claims**: the first evidence rung proves a local stdio fixture-client path. It does not claim WebSocket, remote IDE trust, arbitrary ACP-client compatibility, MCP serving, plugin runtime, live-provider quality, or production readiness.
