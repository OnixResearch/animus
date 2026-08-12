# Tasks

- [x] [serial] Define changed-artifact summary, validation profile candidate, validation plan item, validation result summary, follow-up intent, and receipt DTOs. r[agent_harness_runtime.hermes_validation_planner.plan]
- [x] [depends:plan] Add pure scoped planning over safe artifact kinds, workspace labels, profile labels, policies, and budgets. r[agent_harness_runtime.hermes_validation_planner.pure_planner]
- [x] [depends:pure_planner] Reject raw shell commands, missing profiles, stale artifact summaries, unsupported language labels, and unavailable validation services before execution. r[agent_harness_runtime.hermes_validation_planner.fail_closed]
- [x] [depends:fail_closed] Map validation results to safe continuation/follow-up intents without automatic fix loops or mutations. r[agent_harness_runtime.hermes_validation_planner.followup]
- [x] [depends:followup] Add positive and negative tests for scoped planning, profile labels, fallback policy, missing service, unsafe commands, no auto-test, and receipt leak rejection. r[agent_harness_runtime.hermes_validation_planner.verification]
