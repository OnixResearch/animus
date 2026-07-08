# Tasks

- [x] [serial] Add declarative fan-out plan DTOs for child-run sets, concurrency policy, shared budgets, cancellation policy, aggregation schema, and mutation arbitration. r[agent_harness_runtime.subagent_fanout_fanin.plan]
- [x] [depends:plan] Add deterministic fan-in aggregation over child outcomes, reports, budget usage, and mutation intents. r[agent_harness_runtime.subagent_fanout_fanin.aggregation]
- [x] [depends:aggregation] Add cancellation and budget exhaustion classification that cannot count missing or cancelled children as success. r[agent_harness_runtime.subagent_fanout_fanin.cancellation_budget]
- [x] [depends:aggregation] Add mutation-intent arbitration results for unresolved, denied, and conflict states without applying effects. r[agent_harness_runtime.subagent_fanout_fanin.mutation_arbitration]
- [x] [depends:plan,aggregation,cancellation_budget,mutation_arbitration] Add positive all-success and partial-success tests plus negative timeout, cancellation, malformed, denied-capability, write-conflict, budget, and mutation-bypass tests. r[agent_harness_runtime.subagent_fanout_fanin.fail_closed]
- [x] [depends:fail_closed] Update docs and focused fan-out/fan-in verification evidence. r[agent_harness_runtime.subagent_fanout_fanin.verification]
