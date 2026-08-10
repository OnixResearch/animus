# Agent harness runtime delta

## ADDED Requirements

### Requirement: Real-check completion reuses accepted composition

r[agent_harness_runtime.real_check_completion_host_smoke.composition] Animus MUST expose an optional packaged completion path that injects selected-check execution and Cairn-summary services into the accepted four-role advisory and authoritative-completion workflow.

#### Scenario: Injected services produce one completion decision

r[agent_harness_runtime.real_check_completion_host_smoke.composition.injected]

- **GIVEN** exact generated inputs and judge selection
- **WHEN** bounded passing observations and a matching Cairn summary are supplied
- **THEN** the workflow SHALL return one metadata-only authoritative-completion receipt

#### Scenario: Role output remains advisory

r[agent_harness_runtime.real_check_completion_host_smoke.composition.roles]

- **GIVEN** an active semantic role
- **WHEN** that role emits output
- **THEN** that output SHALL NOT replace selected-check evidence or the Cairn completion summary

### Requirement: Process authority remains isolated

r[agent_harness_runtime.real_check_completion_host_smoke.isolation] The real-check smoke MUST live in a separate non-default authority crate whose license and dependencies make the bounded process authority explicit.

#### Scenario: Default crates remain process-free

r[agent_harness_runtime.real_check_completion_host_smoke.isolation.default]

- **GIVEN** the default and no-default reusable surfaces
- **WHEN** those surfaces compile
- **THEN** they SHALL NOT require the new real-check host crate or `bounded-exec`

### Requirement: Selected commands execute under exact limits

r[agent_harness_runtime.real_check_completion_host_smoke.execution] The host MUST execute only the exact reviewed `nickel-config` and `cairn-validate` argv through `bounded-exec`, without a shell, using null stdin, an explicit absolute root, a cleared reviewed environment, named timeout and output budgets, accepted exit code zero, and process-group teardown.

#### Scenario: Reviewed quick checks run

r[agent_harness_runtime.real_check_completion_host_smoke.execution.pass]

- **GIVEN** the generated registry selects the two reviewed quick checks
- **WHEN** both children exit zero within bounds
- **THEN** the host SHALL produce two passing observations

#### Scenario: Command drift is rejected before spawn

r[agent_harness_runtime.real_check_completion_host_smoke.execution.drift]

- **GIVEN** a reviewed check identifier
- **WHEN** that identifier carries different argv
- **THEN** the host SHALL reject it before any child starts

### Requirement: Child facts fail closed

r[agent_harness_runtime.real_check_completion_host_smoke.failure] The host MUST retain bounded child facts and MUST classify spawn failure, nonzero exit, signal termination, deadline expiry, output overflow, cancellation, or incomplete cleanup as non-passing or infrastructure failure.

#### Scenario: Nonzero child blocks completion

r[agent_harness_runtime.real_check_completion_host_smoke.failure.nonzero]

- **GIVEN** one required child process
- **WHEN** that child exits nonzero
- **THEN** authoritative completion SHALL be blocked

#### Scenario: Deadline blocks completion

r[agent_harness_runtime.real_check_completion_host_smoke.failure.timeout]

- **GIVEN** one required child process with a named deadline
- **WHEN** that child exceeds its deadline
- **THEN** authoritative completion SHALL be blocked and process-group teardown SHALL be attempted

#### Scenario: Output overflow blocks completion

r[agent_harness_runtime.real_check_completion_host_smoke.failure.output]

- **GIVEN** a required child with named output bounds
- **WHEN** stdout or stderr exceeds its retention bound
- **THEN** authoritative completion SHALL be blocked without printing retained child output

### Requirement: Pinned Cairn validates captured observations

r[agent_harness_runtime.real_check_completion_host_smoke.cairn] The host MUST construct the Cairn verification contract and result rows from the same selected checks and admitted observations, evaluate them with the pinned Cairn core, and inject only the normalized report into the completion reducer.

#### Scenario: Matching Cairn report permits completion

r[agent_harness_runtime.real_check_completion_host_smoke.cairn.pass]

- **GIVEN** all required observations pass
- **WHEN** every registry, selection, policy, subject, command, evidence, and receipt identity matches
- **THEN** Cairn SHALL return a valid passing summary

#### Scenario: Stale Cairn input blocks completion

r[agent_harness_runtime.real_check_completion_host_smoke.cairn.stale]

- **GIVEN** selected checks and captured observations
- **WHEN** any subject or selection identity drifts
- **THEN** Cairn SHALL return a non-passing summary and completion SHALL be blocked

### Requirement: Real-check behavior has positive and negative tests

r[agent_harness_runtime.real_check_completion_host_smoke.verification] The host MUST keep positive fixture-child coverage with negative command-drift, missing-executable, nonzero, timeout, output-overflow, stale-subject, dirty-state, and output-leak coverage.

#### Scenario: Fixture rail passes

r[agent_harness_runtime.real_check_completion_host_smoke.verification.fixtures]

- **GIVEN** the focused fixture rail
- **WHEN** that rail runs
- **THEN** it SHALL exercise actual bounded fixture children and both positive and negative completion paths

### Requirement: Current-head live evidence is explicit

r[agent_harness_runtime.real_check_completion_host_smoke.receipt] The focused rail MUST separate sandbox-safe fixture evidence from an opt-in live mode that requires a clean Git head and executes the current generated registry commands.

#### Scenario: Clean current head emits live receipt

r[agent_harness_runtime.real_check_completion_host_smoke.receipt.live]

- **GIVEN** live mode runs from a clean Git head
- **WHEN** both reviewed checks pass
- **THEN** the rail SHALL emit a metadata-only receipt bound to that head and a BLAKE3 source identity

#### Scenario: Dirty live run is rejected

r[agent_harness_runtime.real_check_completion_host_smoke.receipt.dirty]

- **GIVEN** live mode is requested
- **WHEN** it observes a dirty Git worktree
- **THEN** it SHALL fail before it claims current-head evidence

### Requirement: Documentation states the bounded claim

r[agent_harness_runtime.real_check_completion_host_smoke.docs] Maintained documentation MUST state that the new rung proves bounded child observations for two reviewed checks but not check-oracle soundness, executable authenticity, sandboxing, hermeticity, release readiness, or production readiness.

#### Scenario: Roadmap names the successor evidence

r[agent_harness_runtime.real_check_completion_host_smoke.docs.roadmap]

- **GIVEN** the maintained dogfood roadmap
- **WHEN** that roadmap is reviewed
- **THEN** it SHALL name the real-check smoke, focused rail, live receipt, and remaining non-claims

### Requirement: Closeout uses final-subject evidence

r[agent_harness_runtime.real_check_completion_host_smoke.closeout] The change MUST pass focused, boundary, workspace, Nix, Cairn, and clean-current-head live checks before archive and publication, or record an exact bounded blocker.

#### Scenario: Final published head is verified

r[agent_harness_runtime.real_check_completion_host_smoke.closeout.final]

- **GIVEN** the change is archived
- **WHEN** that archived change is published
- **THEN** `origin/main` SHALL contain the completion commit and the final live receipt SHALL bind the same clean head
