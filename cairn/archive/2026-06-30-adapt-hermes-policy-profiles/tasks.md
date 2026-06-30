# Tasks

- [x] [serial] Define policy/profile descriptor DTOs with kind, scope, trust, priority, goal, success criteria, constraints, validation rules, stop rules, provenance, and optional prompt snippets. r[agent_harness_runtime.hermes_policy_profiles.descriptor]
- [x] [depends:descriptor] Add positive composition fixtures for trusted Hermes-style global posture. r[agent_harness_runtime.hermes_policy_profiles.descriptor.valid]
- [x] [serial] Add deterministic policy precedence and conflict diagnostics. r[agent_harness_runtime.hermes_policy_profiles.precedence]
- [x] [depends:precedence] Add tests proving global safety constraints defeat implicit project-local overrides. r[agent_harness_runtime.hermes_policy_profiles.precedence.safety_wins]
- [x] [serial] Add prompt-hygiene validation for goal-first bounded policy text with success criteria and stop rules. r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene]
- [x] [depends:prompt_hygiene] Add negative tests for process-heavy, unbounded, stale-model, and missing-validation policy text. r[agent_harness_runtime.hermes_policy_profiles.prompt_hygiene.process_heavy]
- [x] [serial] Gate project-local policy descriptors behind host trust decisions. r[agent_harness_runtime.hermes_policy_profiles.project_trust]
- [x] [depends:project_trust] Add tests proving untrusted project policy cannot influence prompts or override global policy. r[agent_harness_runtime.hermes_policy_profiles.project_trust.omit]
- [x] [serial] Document and test that policy descriptors do not replace runtime capability enforcement. r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary]
- [x] [depends:enforcement_boundary] Add read-only policy plus mutation-denial integration tests. r[agent_harness_runtime.hermes_policy_profiles.enforcement_boundary.readonly]
- [x] [depends:descriptor.valid,precedence.safety_wins,prompt_hygiene.process_heavy,project_trust.omit,enforcement_boundary.readonly] Update docs and run focused policy-profile tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_policy_profiles.verification]
