# Tasks

- [x] [serial] Define typed workflow command manifests with validated name, summary, inputs, capabilities, side-effect posture, retrieval budget reference, validation gates, output contract, and provenance metadata. r[agent_harness_runtime.claude_code_commands.manifest]
- [x] [depends:manifest] Add negative manifest tests for invalid names, missing schemas, missing side-effect posture, undeclared capabilities, absent output contracts, and untrusted project sources. r[agent_harness_runtime.claude_code_commands.manifest.invalid]
- [x] [serial] Implement deterministic command registry construction and collision diagnostics over in-memory manifests. r[agent_harness_runtime.claude_code_commands.registry]
- [x] [depends:registry] Add duplicate-name tests that prove deterministic winner/disable behavior and safe source diagnostics. r[agent_harness_runtime.claude_code_commands.registry.duplicate]
- [x] [serial] Add pure workflow invocation argument binding and validation before prompt expansion or dispatch. r[agent_harness_runtime.claude_code_commands.binding]
- [x] [depends:binding] Add negative binding tests for missing, unknown, malformed, and wrong-type arguments that prove no workflow side effect starts. r[agent_harness_runtime.claude_code_commands.binding.invalid]
- [x] [serial] Add output contract DTOs and validation paths for explanation, review, patch summary, validation report, commit-message proposal, and structured DTO outputs. r[agent_harness_runtime.claude_code_commands.outputs]
- [x] [depends:outputs] Add mismatch tests proving malformed workflow outputs are not treated as successful completion. r[agent_harness_runtime.claude_code_commands.outputs.mismatch]
- [x] [serial] Document Claude Code commands as a bounded reference and explicitly exclude default shell-script execution and PR creation. r[agent_harness_runtime.claude_code_commands.scope]
- [x] [depends:scope.no_policy_bypass,manifest.invalid,registry.duplicate,binding.invalid,outputs.mismatch] Run focused command tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_commands.scope]
