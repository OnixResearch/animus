# Tasks

- [x] [serial] Define the conversation render model as a pure reducer over `RuntimeEvent` and adapter DTOs, with no terminal I/O and no new reusable-crate contracts. r[agent_harness_runtime.tui_conversation.boundary]
- [x] [depends:boundary] Add typed, bounded transcript entries for user messages, assistant text, thinking, tool-use, tool-result, image, and error with safe metadata and no raw provider bodies or credentials. r[agent_harness_runtime.tui_conversation.transcript_reducer]
- [x] [depends:transcript_reducer] Add a deterministic `render_text_snapshot` path and positive tests proving recorded runtime/adapter events produce the expected ordered transcript without a terminal. r[agent_harness_runtime.tui_conversation.test_backend]
- [x] [depends:transcript_reducer] Add bounded-output handling and negative tests for oversized entries, truncation metadata, unknown block kinds, malformed events, and leak rejection. r[agent_harness_runtime.tui_conversation.bounded_output]
- [x] [depends:boundary,transcript_reducer,test_backend,bounded_output] Run focused reducer/snapshot tests, dependency-isolation checks, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.tui_conversation.verification]
