# Tasks

- [ ] [serial] Define non-interactive run request DTOs for prompt input, external context handles, sandbox profile, output preferences, and optional resume identity. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request]
- [ ] [depends:run_request] Add positive request-construction tests and negative missing-prompt/invalid-sandbox/resume-mismatch tests. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request.validation]
- [ ] [serial] Define versioned append-only event envelopes for thread, turn, item, model, tool, policy, hook, usage, completion, and failure events. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.event_stream]
- [ ] [depends:event_stream] Add JSONL and Preserves-ready serialization tests with stable event ordering and bounded event bodies. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.event_stream.ordering]
- [ ] [serial] Define final response selection from the event stream and shell-owned output sink behavior. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.final_output]
- [ ] [depends:final_output] Add tests proving the final output is extracted from the terminal response item and not from human-readable logs. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.final_output.selection]
- [ ] [serial] Add structured-output validation interfaces for schema-identified final responses. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.structured_output]
- [ ] [depends:structured_output] Add positive schema-valid output tests and negative malformed/extra-field/missing-field tests. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.structured_output.validation]
- [ ] [serial] Define controller traits for starting, resuming, running, inspecting, and cancelling runtime threads. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.thread_controller]
- [ ] [depends:thread_controller] Add resume and cancellation tests proving thread identity and ledger hashes are checked before execution continues. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.thread_controller.resume]
- [ ] [depends:run_request.validation,event_stream.ordering,final_output.selection,structured_output.validation,thread_controller.resume] Update docs and run focused exec/SDK tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.openai_codex_jsonl_exec_sdk.run_request]
