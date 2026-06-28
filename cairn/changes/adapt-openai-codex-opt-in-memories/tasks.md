# Tasks

- [ ] [serial] Define memory policy DTOs for independent use and generation controls at user, project, and thread scopes. r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls]
- [ ] [depends:policy_controls] Add positive opted-in tests and negative missing-consent/disabled-scope tests. r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls.consent]
- [ ] [serial] Define memory record contracts for kind, summary, scope, freshness, source thread identity, supporting evidence, redaction status, and confidence. r[agent_harness_runtime.openai_codex_opt_in_memories.record_contract]
- [ ] [depends:record_contract] Add validation tests for malformed records, stale records, missing evidence handles, and invalid scope. r[agent_harness_runtime.openai_codex_opt_in_memories.record_contract.validation]
- [ ] [serial] Add conservative generation eligibility checks for external context, secrets, redaction failures, inactive consent, and budget gates. r[agent_harness_runtime.openai_codex_opt_in_memories.generation_eligibility]
- [ ] [depends:generation_eligibility] Add negative tests proving disallowed external context and secret-bearing content cannot generate memories. r[agent_harness_runtime.openai_codex_opt_in_memories.generation_eligibility.exclusions]
- [ ] [serial] Integrate optional memory injection into prompt assembly ledgers with included, skipped, expired, redacted, and over-budget states. r[agent_harness_runtime.openai_codex_opt_in_memories.injection_ledger]
- [ ] [depends:injection_ledger] Add tests proving memory injection is bounded and source-attributed. r[agent_harness_runtime.openai_codex_opt_in_memories.injection_ledger.bounded]
- [ ] [serial] Document that required team rules belong in guidance or policy, not memory-only state. r[agent_harness_runtime.openai_codex_opt_in_memories.non_authoritative]
- [ ] [depends:non_authoritative] Add negative tests proving disabled memories do not remove required guidance or policy rules. r[agent_harness_runtime.openai_codex_opt_in_memories.non_authoritative.guidance]
- [ ] [depends:policy_controls.consent,record_contract.validation,generation_eligibility.exclusions,injection_ledger.bounded,non_authoritative.guidance] Update docs and run focused memory tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.openai_codex_opt_in_memories.policy_controls]
