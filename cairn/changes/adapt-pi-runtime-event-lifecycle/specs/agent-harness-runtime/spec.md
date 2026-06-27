# agent-harness-runtime Change Spec: Pi runtime event lifecycle adaptation

## Requirements

### Requirement: Typed runtime event stream

r[agent_harness_runtime.pi_event_lifecycle.typed_events] Animus MUST expose a typed runtime event stream for stable agent lifecycle transitions including runtime start, context replay, turn start, loop decision, model dispatch, tool dispatch, tool result, final response, cancellation, compaction, and shutdown.

#### Scenario: Events preserve turn order

r[agent_harness_runtime.pi_event_lifecycle.typed_events.order]
- GIVEN a runtime turn that calls a model and then a tool before producing a final response
- WHEN events are collected from the run
- THEN the events appear in deterministic transition order
- AND each event contains typed correlation IDs sufficient to associate decisions, dispatches, and results.

### Requirement: Host-injected middleware hooks

r[agent_harness_runtime.pi_event_lifecycle.middleware] Animus MUST support host-injected middleware hooks for observe-only, mutate, block, and replace-style boundary decisions without runtime code discovering plugins or loading code from disk.

#### Scenario: Middleware blocks a tool call

r[agent_harness_runtime.pi_event_lifecycle.middleware.block_tool]
- GIVEN a host injects a tool-call middleware that denies a requested capability
- WHEN the loop decision requests that tool
- THEN runtime emits a typed tool-denied event and report status
- AND the tool adapter is not called.

### Requirement: Deterministic hook chaining

r[agent_harness_runtime.pi_event_lifecycle.hook_order] Middleware hooks that can modify a boundary payload MUST run in configured deterministic order, and each later hook MUST observe the accepted output of earlier hooks.

#### Scenario: Hook mutation order is reproducible

r[agent_harness_runtime.pi_event_lifecycle.hook_order.chained]
- GIVEN two middleware hooks modify safe metadata on a model dispatch request
- WHEN runtime prepares the dispatch
- THEN the second hook observes the first hook's accepted mutation
- AND repeated runs with the same hook order produce identical event and report metadata.

### Requirement: Hook failures fail safe at guarded boundaries

r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks] Errors returned by middleware guarding model dispatch, tool dispatch, compaction, replay, or session replacement MUST fail safe with typed runtime failures rather than allowing the guarded action to continue.

#### Scenario: Tool guard error denies execution

r[agent_harness_runtime.pi_event_lifecycle.fail_safe_hooks.tool]
- GIVEN a middleware hook errors while evaluating a tool-call boundary
- WHEN runtime handles the hook error
- THEN runtime emits a typed hook-failure event and marks the tool call denied or aborted
- AND it does not execute the tool adapter.

### Requirement: Event stream is mode-neutral

r[agent_harness_runtime.pi_event_lifecycle.mode_neutral] Runtime events MUST be reusable by direct SDK-style embedding, headless protocol streaming, receipts, and future UI shells without embedding TUI-specific component state in the core event DTOs.

#### Scenario: Headless consumer receives same lifecycle facts

r[agent_harness_runtime.pi_event_lifecycle.mode_neutral.headless]
- GIVEN a headless host subscribes to runtime events
- WHEN a run completes with model and tool activity
- THEN the host receives the same stable lifecycle facts as an in-process SDK subscriber
- AND no event requires terminal UI state to interpret.
