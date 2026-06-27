# Tasks

- [ ] [serial] Define runtime event DTOs for runtime/session start, context replay, turn start/end, loop decisions, model dispatch, tool dispatch, tool result, response, cancellation, compaction, and shutdown. r[agent_harness_runtime.pi_event_lifecycle.typed_events]
- [ ] [depends:typed_events] Add event-order tests for model-only, tool-using, final-response, stop, cancellation, and adapter-failure runs. r[agent_harness_runtime.pi_event_lifecycle.typed_events.order]
- [ ] [serial] Define host-injected middleware traits or service contracts for observe-only, mutate, block, and replace-style hooks. r[agent_harness_runtime.pi_event_lifecycle.middleware]
- [ ] [depends:middleware] Wire tool-call middleware before adapter execution and add denial tests proving the adapter is not called. r[agent_harness_runtime.pi_event_lifecycle.middleware.block_tool]
- [ ] [depends:middleware] Add deterministic hook-chain execution where later hooks see earlier accepted mutations. r[agent_harness_runtime.pi_event_lifecycle.hook_order]
- [ ] [depends:hook_order] Add repeated-run tests proving hook ordering produces stable event/report metadata. r[agent_harness_runtime.pi_event_lifecycle.hook_order.chained]
- [ ] [depends:middleware] Add fail-safe hook error handling for model dispatch, tool dispatch, compaction, replay, and session replacement surfaces. r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks]
- [ ] [depends:fail_safe_hooks] Add negative tests proving hook errors block guarded tool/model actions and surface typed hook-failure events. r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks.tool]
- [ ] [depends:typed_events] Review event DTOs for TUI-free mode neutrality and document intended SDK/RPC/receipt reuse. r[agent_harness_runtime.pi_event_lifecycle.mode_neutral]
- [ ] [depends:mode_neutral] Add a headless subscriber test or fixture proving lifecycle facts can be consumed without terminal UI state. r[agent_harness_runtime.pi_event_lifecycle.mode_neutral.headless]
- [ ] [depends:typed_events.order,middleware.block_tool,hook_order.chained,fail_safe_hooks.tool,mode_neutral.headless] Run focused event tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.pi_event_lifecycle.typed_events]
