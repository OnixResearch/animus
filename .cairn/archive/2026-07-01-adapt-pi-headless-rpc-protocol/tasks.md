# Tasks

- [x] [serial] Define headless command, response, and event DTOs for prompt, steer, follow-up, abort, state, messages, model selection, ledger/session operations, compaction, and shutdown. r[agent_harness_runtime.pi_headless_protocol.command_response]
- [x] [depends:command_response] Add prompt acknowledgement tests proving request IDs are preserved and runtime events stream separately. r[agent_harness_runtime.pi_headless_protocol.command_response.prompt]
- [x] [serial] Implement or specify strict LF record framing with CRLF tolerance and no Unicode-separator splitting. r[agent_harness_runtime.pi_headless_protocol.framing]
- [x] [depends:framing] Add parser fixtures for LF, CRLF, embedded Unicode separators, truncated records, and multiple records per buffer. r[agent_harness_runtime.pi_headless_protocol.framing.unicode_separator]
- [x] [depends:command_response] Add explicit active-turn queueing validation for prompt, steer, and follow-up commands. r[agent_harness_runtime.pi_headless_protocol.queueing]
- [x] [depends:queueing] Add negative tests proving ambiguous active-turn prompts are rejected before ledger mutation. r[agent_harness_runtime.pi_headless_protocol.queueing.reject_ambiguous]
- [x] [depends:command_response] Map runtime lifecycle events into headless event DTOs with model/tool/turn correlation IDs. r[agent_harness_runtime.pi_headless_protocol.event_stream]
- [x] [depends:event_stream] Add event-stream fixtures for tool start/update/end correlation and model/turn ordering. r[agent_harness_runtime.pi_headless_protocol.event_stream.tool]
- [x] [serial] Add bounded safe protocol error DTOs for parse, invalid command, invalid state, unavailable service, cancellation, and runtime failures. r[agent_harness_runtime.pi_headless_protocol.safe_errors]
- [x] [depends:safe_errors] Add malformed-record and unavailable-service negative fixtures proving raw payloads and secrets are not echoed. r[agent_harness_runtime.pi_headless_protocol.safe_errors.malformed]
- [x] [depends:command_response.prompt,framing.unicode_separator,queueing.reject_ambiguous,event_stream.tool,safe_errors.malformed] Update protocol docs and run focused protocol tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.pi_headless_protocol.command_response]
