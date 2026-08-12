# Tasks

- [x] [serial] Define layered guidance source DTOs with scope, precedence, override semantics, origin, and content hash. r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources]
- [x] [depends:guidance_sources] Add positive nested-precedence tests and negative ambiguous-override tests. r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources.precedence]
- [x] [serial] Add prompt assembly budget ledgers for included, skipped, truncated, and redacted guidance sources. r[agent_harness_runtime.openai_codex_guidance_skill_budget.budget_ledger]
- [x] [depends:budget_ledger] Add negative over-budget tests proving truncation is recorded and source attribution is preserved. r[agent_harness_runtime.openai_codex_guidance_skill_budget.budget_ledger.truncation]
- [x] [serial] Define skill discovery metadata with name, description, source, invocation policy, dependency hints, and context-budget cost. r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_metadata]
- [x] [depends:skill_metadata] Add tests proving initial skill context includes metadata only and respects the configured skill budget. r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_metadata.progressive]
- [x] [serial] Add deferred full-skill loading for explicit invocation and policy-approved implicit invocation. r[agent_harness_runtime.openai_codex_guidance_skill_budget.deferred_loading]
- [x] [depends:deferred_loading] Add negative tests for disabled skills, ambiguous names, missing dependencies, and implicit invocation denial. r[agent_harness_runtime.openai_codex_guidance_skill_budget.deferred_loading.denied]
- [x] [serial] Define skill package descriptors for instructions, optional scripts, references, assets, dependencies, and trust metadata. r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_package]
- [x] [depends:skill_package] Add negative tests proving untrusted skill scripts cannot execute through runtime core or bypass hook/tool policy. r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_package.trust]
- [x] [depends:guidance_sources.precedence,budget_ledger.truncation,skill_metadata.progressive,deferred_loading.denied,skill_package.trust] Update docs and run focused guidance/skill tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources]
