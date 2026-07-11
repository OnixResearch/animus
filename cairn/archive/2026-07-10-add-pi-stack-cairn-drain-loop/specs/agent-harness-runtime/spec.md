# Agent Harness Runtime Specification

## ADDED Requirements

### Requirement: Pi-hosted stack Cairn drain supervisor [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor]]

Animus MUST provide an opt-in Steel Scheme supervisor that can coordinate bounded Cairn drain rounds across selected sibling repositories while Pi temporarily owns agent execution and Cairn remains the lifecycle authority.

#### Scenario: Native Cairn repositories are discovered deterministically [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.discovery]]

- GIVEN an operator supplies a workspace root containing independent child repositories
- WHEN the supervisor builds a stack drain plan
- THEN it MUST select only direct child repositories with native `cairn/changes` lifecycle roots
- AND explicit repository filters MUST preserve operator ordering and reject unknown repositories.

#### Scenario: Plan mode performs no agent or lifecycle effects [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.plan_only]]

- GIVEN the operator does not pass the explicit run option
- WHEN the supervisor renders the discovered stack plan
- THEN it MUST NOT launch Pi, execute Cairn mutations, modify lifecycle files, or perform VCS operations
- AND the plan MUST report selected repositories, active change counts, round bounds, and cross-repository ordering non-claims.

#### Scenario: Pi receives one bounded fresh drain round [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.pi_host]]

- GIVEN a selected repository has active Cairn changes and is not blocked for the current run
- WHEN the supervisor dispatches a round
- THEN it MUST launch a fresh named Pi print-mode session without continuing, resuming, or forking prior session state
- AND the host MUST apply a named finite process timeout with a bounded termination grace
- AND the prompt MUST constrain Pi to at most one dependency-ready change, repository-local instructions, Cairn gates and validation, preserved user changes, no push, and no pull-request operations.

#### Scenario: Lifecycle state determines progress [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.progress]]

- GIVEN a Pi round exits
- WHEN the supervisor compares before and after Cairn lifecycle snapshots
- THEN one removed active change with one new matching archive entry MUST classify as archived progress
- AND a decreased unchecked-task count in exactly one still-active change MAY classify as task progress
- AND unchanged lifecycle state MUST classify as no progress rather than completion.

#### Scenario: Scope breaches fail closed [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.progress.scope_breach]]

- GIVEN a Pi round adds an active change, removes multiple active changes, changes task state in multiple active packages, or removes a change without a new matching archive
- WHEN the supervisor classifies the result
- THEN it MUST stop the stack run with a scope-breach diagnostic
- AND it MUST NOT dispatch another repository until an operator inspects the state.

#### Scenario: Observed archives are independently validated [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.verification]]

- GIVEN a Pi round appears to archive exactly one change
- WHEN the supervisor verifies the resulting repository
- THEN it MUST require a newly created matching archive directory and a passing canonical Cairn validation
- AND Pi process success or model completion text alone MUST NOT establish lifecycle completion.

#### Scenario: Local blockers do not stall independent repositories [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.blockers]]

- GIVEN a Pi process fails or produces no lifecycle progress in one repository
- WHEN other selected repositories still contain active changes
- THEN the failed repository MUST remain active and blocked for the current run
- AND the supervisor SHOULD continue with an independent unblocked repository until the round budget or another terminal state is reached.

#### Scenario: Cross-repository ordering remains a non-claim [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.cross_repo_non_claim]]

- GIVEN receipt-backed Cairn cross-repository dependencies are not available for all selected changes
- WHEN the supervisor orders sibling repositories
- THEN the order MUST be described as an advisory scheduling preference
- AND the supervisor MUST NOT claim cross-repository dependency satisfaction, release eligibility, or stack correctness.

#### Scenario: Positive and negative fixtures cover the supervisor [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.tests]]

- GIVEN in-memory repository snapshots for empty, active, progressing, archived, blocked, and scope-breach states
- WHEN the Steel self-test evaluates planning and classification
- THEN positive cases MUST select and classify the expected bounded action
- AND negative cases MUST reject unknown repositories, reused Pi sessions, added changes, multiple removals, multi-change task mutation, and missing archive evidence.

#### Scenario: Operator documentation explains the temporary boundary [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.docs]]

- GIVEN an operator reads the Ralph loop documentation
- WHEN they inspect stack drain usage
- THEN the documentation MUST show plan-only and explicit run examples, fresh Pi session behavior, validation and blocker semantics, and self-test commands
- AND it MUST state that this Pi-hosted helper is temporary and is not the embedded Animus production host.

#### Scenario: Focused closeout checks pass [r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.closeout]]

- GIVEN the Pi-hosted supervisor implementation and documentation are complete
- WHEN the focused Steel self-test, stack plan smoke, Cairn gates, and lifecycle validation run
- THEN each check MUST pass without launching a mutation-mode Pi session
- AND closeout evidence MUST preserve and report unrelated pre-existing user changes.
