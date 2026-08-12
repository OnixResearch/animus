# Tasks

- [x] [serial] Define typed headless run requests with objective, identity, input references, policy profile, allowed capabilities, output contract, validation plan, and noninteractive behavior. r[agent_harness_runtime.claude_code_headless.request]
- [x] [depends:request] Add negative request validation tests for missing objective, identity, policy profile, output contract, noninteractive behavior, and malformed input references. r[agent_harness_runtime.claude_code_headless.request.invalid]
- [x] [serial] Define stable headless event DTOs for lifecycle, prompt, model, tool, permission, background work, validation, receipt, and final-response events. r[agent_harness_runtime.claude_code_headless.events]
- [x] [depends:events] Add bounded-event tests proving large model/tool/validation/background outputs are represented with truncation metadata and BLAKE3 hashes. r[agent_harness_runtime.claude_code_headless.events.bounded]
- [x] [serial] Add JSONL and Preserves event projection adapters that preserve identity, ordering metadata, status classes, redaction markers, and hashes. r[agent_harness_runtime.claude_code_headless.serialization]
- [x] [depends:serialization] Add unsupported-projection tests proving runtime fails safely instead of emitting lossy or unbounded debug text. r[agent_harness_runtime.claude_code_headless.serialization.unsupported]
- [x] [serial] Define noninteractive completion status classes and final receipt evidence references. r[agent_harness_runtime.claude_code_headless.completion]
- [x] [depends:completion] Add prompt-required tests proving unresolved permission asks complete without dispatching guarded actions. r[agent_harness_runtime.claude_code_headless.completion.prompt_required]
- [x] [serial] Document Claude Code headless automation as a bounded reference and exclude default PR creation and cloud-session requirements. r[agent_harness_runtime.claude_code_headless.scope]
- [x] [depends:scope.no_default_pr,request.invalid,events.bounded,serialization.unsupported,completion.prompt_required] Run focused headless tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_headless.scope]
