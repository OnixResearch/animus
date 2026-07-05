# agent-harness-runtime Change Spec: OpenAI Codex guidance and skill budget adaptation

## Requirements

### Requirement: Layered guidance source attribution

r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources] Animus guidance assembly MUST represent each guidance source with scope, precedence, override semantics, origin, and content hash.

#### Scenario: Nested guidance overrides broader guidance

r[agent_harness_runtime.openai_codex_guidance_skill_budget.guidance_sources.precedence]
- GIVEN a root guidance source and a nested guidance source both define behavior for the current working scope
- WHEN Animus assembles guidance for that scope
- THEN the nested guidance is applied after the root guidance
- AND the assembly ledger records both sources and their precedence order.

### Requirement: Prompt assembly budget ledger

r[agent_harness_runtime.openai_codex_guidance_skill_budget.budget_ledger] Prompt assembly MUST record included, skipped, truncated, and redacted guidance sources with budget consumption and reasons.

#### Scenario: Over-budget guidance is explicit

r[agent_harness_runtime.openai_codex_guidance_skill_budget.budget_ledger.truncation]
- GIVEN available guidance exceeds the configured prompt budget
- WHEN Animus assembles model context
- THEN lower-priority or over-budget guidance is skipped or truncated according to policy
- AND the assembly ledger records source identity, budget impact, and truncation reason.

### Requirement: Progressive skill metadata discovery

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_metadata] Skill discovery MUST expose bounded metadata for available skills without loading full instructions, references, assets, or scripts into initial model context.

#### Scenario: Initial skill context is metadata-only

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_metadata.progressive]
- GIVEN a repository has multiple skill packages with large references
- WHEN Animus builds initial model context
- THEN only bounded skill metadata is included
- AND full skill instructions and references are not loaded until a skill is selected.

### Requirement: Deferred skill loading respects invocation policy

r[agent_harness_runtime.openai_codex_guidance_skill_budget.deferred_loading] Full skill loading MUST require explicit invocation or a policy-approved implicit invocation decision.

#### Scenario: Implicit invocation is denied for disabled skill

r[agent_harness_runtime.openai_codex_guidance_skill_budget.deferred_loading.denied]
- GIVEN a skill is disabled or disallows implicit invocation
- WHEN a task appears to match that skill's description without explicit invocation
- THEN Animus does not load the full skill
- AND the selection ledger records the denied invocation reason.

### Requirement: Skill packages are trust-gated resources

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_package] Skill package descriptors MUST treat instructions, optional scripts, references, assets, dependencies, and trust metadata as resources subject to runtime policy before use.

#### Scenario: Untrusted skill script cannot execute

r[agent_harness_runtime.openai_codex_guidance_skill_budget.skill_package.trust]
- GIVEN a selected skill references an untrusted script
- WHEN the runtime reaches a step that would execute the script
- THEN execution is denied unless shell policy and trust records authorize it
- AND the denial is recorded without bypassing hook or tool policy.
