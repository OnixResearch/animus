# agent-harness-runtime Change Spec: Pi headless RPC protocol adaptation

## Requirements

### Requirement: Headless command-response protocol

r[agent_harness_runtime.pi_headless_protocol.command_response] Animus MUST define a headless command-response protocol for driving runtime turns, queueing steering and follow-up messages, aborting work, inspecting state, selecting models, operating on session ledger state, requesting compaction, and shutting down without requiring a terminal UI.

#### Scenario: Prompt command acknowledges acceptance

r[agent_harness_runtime.pi_headless_protocol.command_response.prompt]
- GIVEN a headless client sends a valid prompt command with a request ID while runtime is idle
- WHEN the command is accepted
- THEN the protocol emits a response carrying the same request ID and an accepted status
- AND runtime lifecycle events stream separately from that acknowledgement.

### Requirement: Strict record framing

r[agent_harness_runtime.pi_headless_protocol.framing] Text protocol transports MUST use strict LF-delimited records, MUST tolerate a trailing CR before LF, and MUST NOT split records on Unicode line separators inside encoded payloads.

#### Scenario: Unicode separator remains payload data

r[agent_harness_runtime.pi_headless_protocol.framing.unicode_separator]
- GIVEN a valid encoded command contains a Unicode line separator inside a string payload
- WHEN the protocol reader consumes bytes
- THEN it treats the separator as payload data rather than a record boundary
- AND the command is parsed only when an LF record delimiter is received.

### Requirement: Explicit active-turn queueing

r[agent_harness_runtime.pi_headless_protocol.queueing] Headless prompt-like commands sent while runtime is active MUST declare steering or follow-up semantics, and runtime MUST reject ambiguous active-turn prompts before they enter the ledger or model context.

#### Scenario: Ambiguous mid-turn prompt is rejected

r[agent_harness_runtime.pi_headless_protocol.queueing.reject_ambiguous]
- GIVEN the runtime is processing a turn
- WHEN a headless client sends a prompt command without steering or follow-up behavior
- THEN the command response reports a preflight rejection
- AND no message is appended to the active session ledger.

### Requirement: Runtime events stream through protocol DTOs

r[agent_harness_runtime.pi_headless_protocol.event_stream] Headless mode MUST stream typed runtime events through protocol DTOs with correlation IDs where applicable and without requiring TUI-specific component state.

#### Scenario: Tool execution events correlate

r[agent_harness_runtime.pi_headless_protocol.event_stream.tool]
- GIVEN a turn dispatches a tool call
- WHEN the headless event stream is observed
- THEN tool-start, tool-update if any, and tool-end events carry a shared tool-call correlation ID
- AND a client can render progress without inspecting internal runtime state.

### Requirement: Safe bounded protocol errors

r[agent_harness_runtime.pi_headless_protocol.safe_errors] Protocol parse errors, invalid commands, invalid streaming behavior, unavailable services, cancelled operations, and runtime failures MUST produce bounded safe error responses that do not expose credentials, provider payloads, raw ledger dumps, full host paths, or unbounded tool output.

#### Scenario: Malformed record returns bounded error

r[agent_harness_runtime.pi_headless_protocol.safe_errors.malformed]
- GIVEN a client sends a malformed encoded record
- WHEN the protocol parser rejects it
- THEN the response reports a bounded parse error class and optional request ID if recoverable
- AND the response does not echo the full malformed payload.
