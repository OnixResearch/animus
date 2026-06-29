# Tasks

- [ ] [serial] Define fresh-context task descriptor DTOs for goal, context references, tool profile, model policy, budgets, timeout, cancellation, and result schema. r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor]
- [ ] [depends:descriptor] Add missing-scheduler negative tests proving no ambient process/provider/tool fallback occurs. r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor.missing_scheduler]
- [ ] [serial] Define read-only side-question policy and child action class constraints. r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question]
- [ ] [depends:side_question] Add mutation-attempt negative tests for write, edit, shell, network, ledger, and non-read tools. r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question.readonly]
- [ ] [serial] Define context and authority isolation rules for child runs. r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation]
- [ ] [depends:isolation] Add tests proving parent allowlists, credentials, hidden prompts, and mutable process-global config are not inherited. r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation.allowlist]
- [ ] [serial] Define attributed task result DTOs with child-run identity, budget usage, citations, uncertainty, and redacted evidence. r[agent_harness_runtime.zerostack_fresh_context_tasks.results]
- [ ] [depends:results] Add malformed-result and unsafe-injection negative tests. r[agent_harness_runtime.zerostack_fresh_context_tasks.results.malformed]
- [ ] [depends:descriptor,side_question,isolation,results] Add task receipts and cancellation outcome tests for user, timeout, policy, and scheduler failures. r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts]
- [ ] [depends:receipts] Update docs and run focused subagent/task tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts.cancelled]
