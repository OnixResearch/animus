# agent-harness-runtime Change Spec: Hermes scoped validation planner contracts

## Requirements

### Requirement: Scoped validation plans use typed descriptors

r[agent_harness_runtime.hermes_validation_planner.plan] Animus MUST represent scoped validation planning with typed DTOs for changed-artifact summaries, validation profile candidates, validation plan items, validation result summaries, continuation follow-up intents, and safe receipts.

#### Scenario: Changed Rust-like artifact maps to profile label

r[agent_harness_runtime.hermes_validation_planner.plan.profile_label]
- GIVEN a host supplies a changed-artifact summary with safe package labels and a trusted validation profile catalog
- WHEN scoped validation planning runs
- THEN the plan selects profile or gate labels with bounded arguments and safe reasons
- AND it does not run `cargo`, inspect manifests, walk directories, or construct shell commands.

### Requirement: Validation planning is pure over host-supplied facts

r[agent_harness_runtime.hermes_validation_planner.pure_planner] Scoped validation planning MUST be pure deterministic logic over host-supplied artifact summaries, validation profile descriptors, policy, budgets, prior result summaries, and route hints, and MUST NOT read files, query VCS, discover package managers, inspect environment variables, execute processes, open networks, or call providers.

#### Scenario: Same summaries produce same plan

r[agent_harness_runtime.hermes_validation_planner.pure_planner.deterministic]
- GIVEN two hosts provide equivalent artifact summaries, profile descriptors, and validation policy
- WHEN planning runs
- THEN the selected plan items, ordering, fallback reasons, and receipt metadata are equivalent
- AND filesystem ordering or ambient workspace state cannot change the result.

### Requirement: Unsafe or unavailable validation fails closed

r[agent_harness_runtime.hermes_validation_planner.fail_closed] Validation planning and result admission MUST fail closed when a selected profile is missing, a raw shell command is supplied outside a trusted profile, a changed-artifact summary is stale, a validation service is unavailable, a language/workspace label is unsupported by policy, or result metadata contains unsafe material.

#### Scenario: Raw command payload is rejected

r[agent_harness_runtime.hermes_validation_planner.fail_closed.raw_command]
- GIVEN a validation candidate contains `bash -c` or another raw command payload instead of a trusted process-job profile label
- WHEN validation plan admission runs
- THEN admission rejects the candidate with an unsafe-command diagnostic
- AND no process execution request or follow-up success claim is produced.

### Requirement: Validation follow-up is explicit and non-mutating

r[agent_harness_runtime.hermes_validation_planner.followup] Failed validation results MAY produce safe continuation or follow-up intents with bounded failure summaries, but MUST NOT automatically trigger model turns, edit files, suppress failures, mark work complete, retry indefinitely, or start auto-fix loops without a separate host policy and admission decision.

#### Scenario: Failed check creates advisory follow-up only

r[agent_harness_runtime.hermes_validation_planner.followup.advisory]
- GIVEN a validation result summary reports failure for a selected profile
- WHEN follow-up reduction runs
- THEN the reducer returns a bounded advisory intent with profile label, status, output hash, and next-check reason
- AND no model call, tool call, mutation, or retry is dispatched by reusable logic.

### Requirement: Scoped validation planner verification

r[agent_harness_runtime.hermes_validation_planner.verification] The scoped validation planner adaptation MUST include focused positive and negative tests for changed-artifact DTOs, profile-label planning, deterministic ordering, fallback policy, missing validation service, unsafe raw command rejection, stale summaries, follow-up intent bounds, no default auto-test behavior, and receipt leak rejection.

#### Scenario: Closeout evidence states host-owned execution

r[agent_harness_runtime.hermes_validation_planner.verification.closeout]
- GIVEN maintainers intend to archive the scoped validation planner adaptation
- WHEN closeout validation runs
- THEN evidence names planner DTO, pure planning, fail-closed, follow-up, receipt, docs, Cairn validation, and Cairn gate checks
- AND it lists command execution, file discovery, language workspace detection, and auto-fix loops as host-owned non-claims.
