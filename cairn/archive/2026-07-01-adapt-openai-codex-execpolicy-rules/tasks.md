# Tasks

- [x] [serial] Define exec policy DTOs for command intents, sandbox context, decisions, rule identities, rationales, and positive/negative rule examples. r[agent_harness_runtime.openai_codex_execpolicy_rules.policy_contract]
- [x] [depends:policy_contract] Add typed Nickel contracts and generated runtime fixtures for valid and invalid policy rules. r[agent_harness_runtime.openai_codex_execpolicy_rules.nickel_source]
- [x] [serial] Implement a pure prefix-rule evaluator that returns the most restrictive decision and matched-rule evidence. r[agent_harness_runtime.openai_codex_execpolicy_rules.rule_evaluation]
- [x] [depends:rule_evaluation] Add positive allow/prompt tests and negative forbid/empty-pattern/malformed-rule tests. r[agent_harness_runtime.openai_codex_execpolicy_rules.rule_evaluation.examples]
- [x] [serial] Add conservative shell-wrapper parsing for simple command chains and fail-closed opaque handling for unsupported shell constructs. r[agent_harness_runtime.openai_codex_execpolicy_rules.shell_splitting]
- [x] [depends:shell_splitting] Add negative tests proving destructive commands cannot be smuggled beside allowed prefixes. r[agent_harness_runtime.openai_codex_execpolicy_rules.shell_splitting.smuggling]
- [x] [serial] Connect policy outcomes to sandbox escalation events without adding concrete process execution to core. r[agent_harness_runtime.openai_codex_execpolicy_rules.sandbox_binding]
- [x] [depends:sandbox_binding] Add runtime event tests proving allow, prompt, and forbid outcomes are recorded with redacted command evidence. r[agent_harness_runtime.openai_codex_execpolicy_rules.sandbox_binding.audit]
- [x] [depends:nickel_source,rule_evaluation.examples,shell_splitting.smuggling,sandbox_binding.audit] Update docs and run focused policy tests, Nickel drift checks, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.openai_codex_execpolicy_rules.policy_contract]
