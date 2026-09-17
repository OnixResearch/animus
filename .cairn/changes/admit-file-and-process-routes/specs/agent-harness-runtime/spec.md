# Agent harness runtime delta

## MODIFIED Requirements

### Requirement: Application configuration is typed and explicit

r[agent_harness_runtime.live_steel_agent_shell.configuration] Human-authored application configuration MUST use typed Nickel contracts for exact component identities, profiles, endpoint cohort, credential reference, capability roots, command and tool bindings, store, skills, limits, evidence policy, and non-claims. Startup admission of host operations MUST require the required operations `context_read` and `subagent`, MUST accept only the labels the runtime implements — `context_read`, `file_edit`, `file_read`, `shell`, `subagent`, and `tool` — and MUST refuse every operation outside that vocabulary.

#### Scenario: A required component is implicit

r[agent_harness_runtime.live_steel_agent_shell.configuration.missing]
- GIVEN configuration omits a required provider, executor, store, authority, or bound
- WHEN startup validation runs
- THEN startup MUST fail before store, credential, network, process, or workspace effects
- AND the shell MUST NOT fill the value from ambient current-directory, environment, account, or registry state.

## ADDED Requirements

### Requirement: An implemented route is admissible

r[agent_harness_runtime.live_steel_agent_shell.route_vocabulary] The admission vocabulary MUST cover every route class the runtime implements a pack for, so a product can admit a route without an Animus release.

#### Scenario: A file-read product admits its route

- GIVEN a configuration that admits `context_read`, `subagent`, and `file_read`
- WHEN startup validation runs
- THEN admission succeeds and the parsed configuration lists `file_read`

#### Scenario: An unimplemented route stays refused

- GIVEN a configuration that admits `kernel_clear`
- WHEN startup validation runs
- THEN admission fails with `InvalidCollection` before any effect
