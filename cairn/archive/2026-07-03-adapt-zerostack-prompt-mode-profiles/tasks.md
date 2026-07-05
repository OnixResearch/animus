# Tasks

- [x] [serial] Define prompt profile descriptor DTOs for profile ID, purpose, action posture, capabilities, source identity, trust scope, precedence, body text, and inclusion policy. r[agent_harness_runtime.zerostack_prompt_profiles.descriptors]
- [x] [depends:descriptors] Add positive profile assembly tests and negative untrusted-source tests. r[agent_harness_runtime.zerostack_prompt_profiles.descriptors.untrusted]
- [x] [serial] Define directive parsing for mode and chain metadata that strips directive syntax from model-visible body text. r[agent_harness_runtime.zerostack_prompt_profiles.directives]
- [x] [depends:directives] Add negative tests for unknown modes, malformed directives, unsupported chain targets, and duplicate exclusive directives. r[agent_harness_runtime.zerostack_prompt_profiles.directives.invalid]
- [x] [serial] Bind profile-requested modes to permission policy as posture inputs only. r[agent_harness_runtime.zerostack_prompt_profiles.mode_binding]
- [x] [depends:mode_binding] Add tests proving profile modes cannot grant missing write, exec, network, model, or tool authority. r[agent_harness_runtime.zerostack_prompt_profiles.mode_binding.no_grant]
- [x] [serial] Define chain-of-prompts continuation plans and host/user decision handling. r[agent_harness_runtime.zerostack_prompt_profiles.chain]
- [x] [depends:chain] Add decline, accept, and bounded-extra-instruction tests. r[agent_harness_runtime.zerostack_prompt_profiles.chain.declined]
- [x] [depends:descriptors,directives,mode_binding,chain] Add prompt profile receipts with BLAKE3 body hashes and redaction markers. r[agent_harness_runtime.zerostack_prompt_profiles.receipts]
- [x] [depends:receipts] Update docs and run focused prompt/profile tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_prompt_profiles.receipts.hashes]
