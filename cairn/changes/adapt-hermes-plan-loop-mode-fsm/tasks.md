# Tasks

- [ ] [serial] Define mode descriptor, mode state, plan extraction, selected-step, loop condition, breakout acknowledgement, retry classification, and mode receipt DTOs. r[agent_harness_runtime.hermes_plan_loop_modes.fsm]
- [ ] [depends:fsm] Add explicit marker-bound plan extraction and selected-step validation with stale, duplicate, malformed, and unsafe text failures. r[agent_harness_runtime.hermes_plan_loop_modes.plan_extraction]
- [ ] [depends:plan_extraction] Integrate loop work and breakout acknowledgement with continuation policy without default model turns, timers, or provider probes. r[agent_harness_runtime.hermes_plan_loop_modes.continuation]
- [ ] [depends:continuation] Add host-owned boundaries for commands, shortcuts, UI pickers, tool activation, read-only enforcement, delegation, persistence, and provider error classification. r[agent_harness_runtime.hermes_plan_loop_modes.host_owned]
- [ ] [depends:host_owned] Add positive and negative tests for mode descriptors, plan extraction, selected steps, loop continuation, stale acknowledgements, no baked-in commands, and receipt leak rejection. r[agent_harness_runtime.hermes_plan_loop_modes.verification]
