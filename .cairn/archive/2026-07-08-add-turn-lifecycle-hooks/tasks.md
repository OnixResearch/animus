# Tasks

- [x] [serial] Define bounded turn lifecycle hook scope, mapping startup to existing `session_start` and excluding duplicate startup labels. r[agent_harness_runtime.turn_lifecycle_hooks.scope]
- [x] [depends:scope] Add stable `pre_turn` and `post_turn` hook point vocabulary with positive parsing tests. r[agent_harness_runtime.turn_lifecycle_hooks.points]
- [x] [depends:points] Add negative tests proving unknown startup aliases and unsupported turn hook capabilities fail validation. r[agent_harness_runtime.turn_lifecycle_hooks.points.negative]
- [x] [serial] Add a pure turn lifecycle FSM core that emits startup, pre-turn, and post-turn lifecycle hook effects without adapter execution. r[agent_harness_runtime.turn_lifecycle_hooks.fsm]
- [x] [depends:fsm] Add positive FSM sequencing tests for startup, pre-turn, post-turn, and completion. r[agent_harness_runtime.turn_lifecycle_hooks.fsm.positive]
- [x] [depends:fsm] Add negative FSM tests proving duplicate startup, post-turn-before-pre-turn, and adapter-owned transition attempts are rejected or impossible. r[agent_harness_runtime.turn_lifecycle_hooks.fsm.negative]
- [x] [depends:points] Add output validation tests proving `pre_turn` may deny before evaluation while `post_turn` cannot deny, rewrite, execute tools, weaken sandbox, or suppress approval. r[agent_harness_runtime.turn_lifecycle_hooks.no_bypass]
- [x] [depends:scope,points,fsm,no_bypass] Update docs and run focused hook/FSM tests, workspace formatting, Cairn validation, and Cairn gates with reviewable non-claim evidence. r[agent_harness_runtime.turn_lifecycle_hooks.verification]
