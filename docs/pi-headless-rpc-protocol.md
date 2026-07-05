# Pi-style headless protocol contract

Animus adapts Pi's headless RPC lessons as typed runtime DTOs and pure protocol helpers. The protocol boundary is not a daemon, network server, terminal UI, Pi compatibility shim, or shell-command surface. Hosts own transports and services; reusable runtime code validates records, acknowledges commands, projects events, and fails closed without ambient I/O.

## Contract map

- `r[agent_harness_runtime.pi_headless_protocol.command_response]`: `HeadlessCommandDraft`, `HeadlessCommand`, `HeadlessResponse`, and `HeadlessCommandKind` cover prompt, steer, follow-up, abort, state, messages, model selection, ledger/session, compaction, and shutdown commands.
- `r[agent_harness_runtime.pi_headless_protocol.command_response.prompt]`: `acknowledge_headless_command` preserves request IDs in command acknowledgements while `map_runtime_events_to_headless` emits runtime events without binding them to an acknowledgement request ID.
- `r[agent_harness_runtime.pi_headless_protocol.framing]`: `split_lf_records` splits only on LF, strips a trailing CR before LF, and leaves Unicode line separators inside payload data.
- `r[agent_harness_runtime.pi_headless_protocol.queueing]`: prompt-like commands during an active turn must declare `steer` or `follow_up`; ambiguous active prompts are rejected before ledger/model mutation.
- `r[agent_harness_runtime.pi_headless_protocol.event_stream]`: `HeadlessProtocolEvent::from_runtime_event` and `map_runtime_events_to_headless` project runtime events into protocol events with safe turn/model/tool correlation IDs where available and without TUI state.
- `r[agent_harness_runtime.pi_headless_protocol.safe_errors]`: parse, invalid-command, invalid-state, unavailable-service, cancellation, and runtime-failure errors use bounded safe `HeadlessProtocolError` responses and do not echo malformed payloads.

## Text fixture record shape

Focused fixtures use a small pipe-shaped record, `request_id|kind|queueing|payload`, to exercise conformance without adding a JSON parser or network transport. Host transports may encode the same DTOs as JSON, Preserves text, or Preserves binary, but they must preserve the same safe command, acknowledgement, event, framing, and error semantics.

## Non-claims

This contract does not add a network daemon, web server, MCP server, TUI, Pi subprocess compatibility mode, generic shell command execution, provider calls, session persistence, or ledger storage. Those remain host-shell or future-change scope.

## Focused verification

Use `cargo test -p animus-runtime --locked`. The protocol tests include positive prompt acknowledgement, LF/CRLF framing, Unicode separator preservation, active-turn steering acceptance, and runtime tool-event correlation, plus negative ambiguous prompt, malformed record, and unsafe payload fixtures.
