# Agent Harness Runtime

## ADDED Requirements

### Requirement: The live Steel agent has one visible composition root

r[agent_harness_runtime.live_steel_agent_shell.composition] Animus MUST provide the live Steel agent only as a non-default host application that selects concrete provider, store, coding-session, Steel, executor, child, skill, control, event, and receipt adapters at one visible composition root.

#### Scenario: The application starts with complete inputs

r[agent_harness_runtime.live_steel_agent_shell.composition.complete]
- GIVEN validated configuration and every required explicit service are available
- WHEN the composition root constructs the application
- THEN it MUST bind stable component and authority identities before the first effect
- AND reusable and default crates MUST remain free of product provider, process, filesystem, credential, daemon, and terminal authority.

### Requirement: The provider-to-Steel tool loop is complete

r[agent_harness_runtime.live_steel_agent_shell.tool_loop] The application MUST drive provider tool calls, committed `steel_repl` dispatch, resumable host results, committed tool results, and provider continuation through the accepted coding-session facade until a terminal outcome.

#### Scenario: The provider requests steel_repl

r[agent_harness_runtime.live_steel_agent_shell.tool_loop.complete]
- GIVEN the configured provider emits an admitted correlated `steel_repl` call
- WHEN the coding session handles the call
- THEN it MUST commit the assistant call before Steel and host dispatch
- AND it MUST commit the correlated tool result before the next provider request.

#### Scenario: The host returns only an admitted marker

r[agent_harness_runtime.live_steel_agent_shell.tool_loop.marker_only]
- GIVEN Steel evaluation yields a host request that requires execution
- WHEN the live application processes the request
- THEN it MUST run the resumable admitted host path or return a typed failure
- AND a model-visible admitted-marker string MUST NOT count as tool completion.

### Requirement: Application configuration is typed and explicit

r[agent_harness_runtime.live_steel_agent_shell.configuration] Human-authored application configuration MUST use typed Nickel contracts for exact component identities, profiles, endpoint cohort, credential reference, capability roots, command and tool bindings, store, skills, limits, evidence policy, and non-claims.

#### Scenario: A required component is implicit

r[agent_harness_runtime.live_steel_agent_shell.configuration.missing]
- GIVEN configuration omits a required provider, executor, store, authority, or bound
- WHEN startup validation runs
- THEN startup MUST fail before store, credential, network, process, or workspace effects
- AND the shell MUST NOT fill the value from ambient current-directory, environment, account, or registry state.

### Requirement: Session and recovery ordering remains authoritative

r[agent_harness_runtime.live_steel_agent_shell.session] The application MUST preserve durable message and tool ordering, queue compare-and-swap, ordered runtime events, safe receipts, active-turn recovery, interrupted-tool repair, and effect-disabled kernel replay.

#### Scenario: Restart follows an observed host effect

r[agent_harness_runtime.live_steel_agent_shell.session.restart]
- GIVEN a host effect completed but its active Steel cell did not commit before process exit
- WHEN the application restarts
- THEN recovery MUST restore the prior committed kernel and retain the observed effect as interrupted-turn evidence
- AND it MUST NOT automatically repeat the host effect.

### Requirement: Application lifecycle is bounded

r[agent_harness_runtime.live_steel_agent_shell.lifecycle] Startup, provider turns, tool calls, host calls, child work, cancellation, shutdown, result output, and recovery MUST use named finite bounds and typed degraded outcomes.

#### Scenario: Cancellation occurs during a tool loop

r[agent_harness_runtime.live_steel_agent_shell.lifecycle.cancelled]
- GIVEN a run receives cancellation while provider or tool work is active
- WHEN the shell observes the cancellation checkpoint
- THEN it MUST stop new dispatch, request owned teardown, persist the observed terminal state, and emit a bounded outcome
- AND it MUST not report normal completion from partial results.

### Requirement: End-to-end evidence separates fixture and live claims

r[agent_harness_runtime.live_steel_agent_shell.verification] The application MUST include a deterministic production-entrypoint scenario and negative scenarios for configuration, authority, provider, tools, Steel, host effects, children, queues, cancellation, bounds, crash recovery, leaks, and fallback, while optional live evidence remains separate.

#### Scenario: The live shell is ready to archive

r[agent_harness_runtime.live_steel_agent_shell.verification.closeout]
- GIVEN maintainers intend to sync and archive the live Steel shell
- WHEN focused and lifecycle validation runs
- THEN all prerequisite focused rails, full composition fixtures, dependency checks, Cairn gates, and relevant Nix checks MUST pass
- AND provider correctness, tool correctness, task success, daemon or TUI readiness, production readiness, and release eligibility MUST remain non-claims.
