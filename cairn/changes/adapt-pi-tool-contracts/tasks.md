# Tasks

- [ ] [serial] Extend tool definition DTOs with validated metadata, schema identity, prompt snippet, prompt guidelines, provenance/source info, capability labels, and result metadata shape. r[agent_harness_runtime.pi_tool_contracts.definition_metadata]
- [ ] [depends:definition_metadata] Add prompt construction tests proving only active tool metadata appears and guidelines remain tool-attributable. r[agent_harness_runtime.pi_tool_contracts.definition_metadata.prompt]
- [ ] [serial] Add optional pure argument preparation before strict current-schema validation. r[agent_harness_runtime.pi_tool_contracts.argument_validation]
- [ ] [depends:argument_validation] Add positive legacy-normalization tests and negative unknown-field/malformed-field tests. r[agent_harness_runtime.pi_tool_contracts.argument_validation.compat]
- [ ] [serial] Add bounded tool output DTOs with visible text, truncation status, byte/line counts, safe references, and BLAKE3 hashes where needed. r[agent_harness_runtime.pi_tool_contracts.output_bounds]
- [ ] [depends:output_bounds] Add oversized-output tests proving raw full bodies are not embedded in model-visible content or receipts. r[agent_harness_runtime.pi_tool_contracts.output_bounds.truncated]
- [ ] [serial] Define opaque mutation target keys and a host-injected mutation coordinator contract. r[agent_harness_runtime.pi_tool_contracts.mutation_queue]
- [ ] [depends:mutation_queue] Add concurrency tests proving same-target mutation windows serialize and distinct-target calls may proceed independently. r[agent_harness_runtime.pi_tool_contracts.mutation_queue.conflict]
- [ ] [serial] Define typed tool dispatch/execution failure classes for invalid input, missing executor, unavailable service, denial, cancellation, malformed output, unbounded oversized output, and execution failure. r[agent_harness_runtime.pi_tool_contracts.failure_classes]
- [ ] [depends:failure_classes] Add negative dispatch tests for missing executor and denied capability that prove no fallback shell/process/provider execution occurs. r[agent_harness_runtime.pi_tool_contracts.failure_classes.missing_executor]
- [ ] [depends:definition_metadata.prompt,argument_validation.compat,output_bounds.truncated,mutation_queue.conflict,failure_classes.missing_executor] Update docs and run focused tool-contract tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.pi_tool_contracts.definition_metadata]
