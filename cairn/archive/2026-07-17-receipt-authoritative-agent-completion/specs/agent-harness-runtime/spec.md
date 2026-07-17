# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Agent checks come from a repository-owned registry

r[agent_harness_runtime.authoritative_completion.registry] Animus MUST maintain a typed Nickel-authored registry for checks that may contribute to agent completion, and the generated runtime export MUST be deterministic and freshness-checked.

#### Scenario: Registry row is complete

r[agent_harness_runtime.authoritative_completion.registry.row]
- GIVEN a check is eligible for agent validation planning
- WHEN its registry row is validated
- THEN the row MUST include a stable check ID, profile, exact argv, affected repository-relative paths/crates/surfaces, prerequisite IDs, capability and risk/tier classes, timeout and output-budget classes, expected artifacts, exact reproduction argv, documentation refs, and concrete positive and negative test contracts
- AND registry membership alone MUST NOT claim that the check passed or that its oracle is sound.

#### Scenario: Registry drift fails closed

r[agent_harness_runtime.authoritative_completion.registry.drift]
- GIVEN authored Nickel, generated runtime data, docs, check entrypoints, prerequisite IDs, or self-test refs disagree
- WHEN the registry rail runs
- THEN it MUST fail with deterministic diagnostics and an exact regeneration/revalidation path
- AND stale generated data MUST NOT select completion checks.

#### Scenario: Unsafe command row is rejected

r[agent_harness_runtime.authoritative_completion.registry.unsafe_command]
- GIVEN a registry row contains a raw shell payload, `bash -c`, credential, private absolute path, unbounded environment map, unknown prerequisite, or unsafe artifact path
- WHEN registry validation runs
- THEN the row MUST be rejected before planning or host execution.

### Requirement: Required check selection is pure and change-aware

r[agent_harness_runtime.authoritative_completion.selection] Animus MUST derive selected required, blocked, and not-selected checks through pure deterministic planning over host-supplied changed-artifact facts, the validated registry, capability observations, tier policy, and final subject identity.

#### Scenario: Changed surface selects direct and prerequisite checks

r[agent_harness_runtime.authoritative_completion.selection.direct]
- GIVEN a current changed-artifact snapshot affects registered Animus crates, policy, lifecycle, docs, or host surfaces
- WHEN selection runs
- THEN it MUST select matching direct checks and their prerequisite closure in deterministic order
- AND it MUST emit registry, selection, policy, changed-artifact, and final-subject hashes.

#### Scenario: Unknown change uses conservative baseline

r[agent_harness_runtime.authoritative_completion.selection.unknown]
- GIVEN a safe changed path or surface does not match a reviewed registry rule
- WHEN selection runs
- THEN it MUST select the configured conservative repository baseline or fail blocked
- AND it MUST NOT silently produce an empty passing plan.

#### Scenario: Pure selector has no host authority

r[agent_harness_runtime.authoritative_completion.selection.pure]
- GIVEN changed-artifact, registry, capability, tier, and policy values in memory
- WHEN the selector is tested
- THEN it MUST require no VCS query, filesystem read, process execution, environment inspection, clock, network, provider, terminal, or persistence access.

### Requirement: Check execution receipts bind bounded host facts

r[agent_harness_runtime.authoritative_completion.execution_receipts] Host-executed checks MUST produce safe versioned result receipts that bind check, command, registry, selection, policy, final subject, output, artifact, cleanup, and execution identities.

#### Scenario: Passing execution receipt is complete

r[agent_harness_runtime.authoritative_completion.execution_receipts.passed]
- GIVEN an injected host executes a selected check successfully for the final subject
- WHEN it emits a result receipt
- THEN the receipt MUST include check ID, exact command hash, status `passed`, registry/selection/policy/final-subject hashes, exit/signal/timeout/cleanup facts, bounded output counts and BLAKE3 identities, actual safe artifact refs and hashes, reproduction argv, and receipt hash
- AND raw unbounded output MUST remain outside the receipt.

#### Scenario: Host failure remains explicit

r[agent_harness_runtime.authoritative_completion.execution_receipts.failure]
- GIVEN execution fails, blocks, times out, is cancelled, is skipped, runs only as a dry run, fails cleanup, or encounters infrastructure failure
- WHEN the host emits a result receipt
- THEN the exact non-pass status and bounded reason MUST be preserved
- AND the receipt MUST NOT be rewritten as `passed` because a wrapper process itself exited successfully.

#### Scenario: Mutation makes receipt stale

r[agent_harness_runtime.authoritative_completion.execution_receipts.stale]
- GIVEN a result receipt cites one final-subject hash and admitted mutation later changes the subject
- WHEN completion evaluation sees the prior receipt
- THEN it MUST classify the receipt as stale and require new selection or execution for the changed subject.

### Requirement: Completion markers are non-authoritative candidates

r[agent_harness_runtime.authoritative_completion.candidate] Steel, model, tool, and wrapper outputs MAY request completion only through a typed completion candidate that Rust revalidates, and no completion marker or prose substring may directly produce verified final response state.

#### Scenario: Exact completion request schedules evaluation

r[agent_harness_runtime.authoritative_completion.candidate.exact]
- GIVEN the Ralph profile receives a valid typed completion request or an admitted exact compatibility marker
- WHEN Rust validates the Steel decision
- THEN runtime MUST create a completion-evaluation candidate bound to current work and subject identities
- AND it MUST NOT emit a verified final response until required evidence passes.

#### Scenario: Incidental substring cannot terminate

r[agent_harness_runtime.authoritative_completion.candidate.substring]
- GIVEN model prose contains `COMPLETE` as part of `not COMPLETE`, an explanation, quoted text, a larger token, or an otherwise untyped response
- WHEN the Ralph decision path evaluates the message
- THEN it MUST NOT treat the substring as verified completion
- AND normal continuation or candidate-validation rules MUST apply.

#### Scenario: Model self-attestation is insufficient

r[agent_harness_runtime.authoritative_completion.candidate.self_attestation]
- GIVEN a model says all checks passed but supplies no current host/Cairn receipt links
- WHEN completion admission runs
- THEN the statement MUST remain advisory model output
- AND it MUST NOT populate passing check evidence.

### Requirement: Required checks use pass-only completion semantics

r[agent_harness_runtime.authoritative_completion.pass_only] Animus MUST count only `passed` as passing evidence for a required check and MUST preserve every other supported status as non-passing.

#### Scenario: Skipped check creates a blocker or follow-up

r[agent_harness_runtime.authoritative_completion.pass_only.skipped]
- GIVEN a selected required validation result has status `skipped`
- WHEN result reduction and completion planning run
- THEN the result MUST create blocked or more-evidence-required state with a safe reason
- AND it MUST NOT reduce to `ValidationFollowUpStatus::None` or satisfy completion.

#### Scenario: Dry run does not pass

r[agent_harness_runtime.authoritative_completion.pass_only.dry_run]
- GIVEN a host produced a valid dry-run plan without executing the selected check
- WHEN completion evidence is aggregated
- THEN the check MUST remain non-passing even if plan validation succeeded.

#### Scenario: All required checks pass exactly once

r[agent_harness_runtime.authoritative_completion.pass_only.complete]
- GIVEN every selected required check and prerequisite has exactly one fresh `passed` receipt with matching identities
- WHEN aggregate reduction runs
- THEN the required and passed sets MUST match
- AND no failed, blocked, skipped, deferred, timed-out, cancelled, dry-run, cleanup-failed, infrastructure-error, missing, duplicate, unknown, or stale row may remain.

### Requirement: Cairn receipt gates verified completion

r[agent_harness_runtime.authoritative_completion.receipt_gate] Animus MUST represent an agent run as verified complete only when an injected current Cairn authoritative completion-validation summary passes and matches the Animus registry, selection, policy, final subject, and required-check identities.

#### Scenario: Matching Cairn receipt authorizes finalization

r[agent_harness_runtime.authoritative_completion.receipt_gate.pass]
- GIVEN Animus has a completion candidate, all required host result receipts, and a passing Cairn validation summary for the same final subject and contract identities
- WHEN the pure completion reducer runs
- THEN it MAY produce verified complete state and a safe final completion receipt
- AND the receipt MUST cite the Cairn validation receipt hash.

#### Scenario: Missing or stale Cairn receipt blocks finalization

r[agent_harness_runtime.authoritative_completion.receipt_gate.missing]
- GIVEN model output, host results, task state, or archive state suggests completion but the Cairn validation summary is absent, failing, unsupported, malformed, or stale
- WHEN completion reduction runs
- THEN verified completion MUST remain blocked
- AND the reducer MUST return a bounded reason or validation follow-up intent.

#### Scenario: Cairn adapter is host-owned

r[agent_harness_runtime.authoritative_completion.receipt_gate.host_adapter]
- GIVEN a host uses exact-pinned Cairn CLI, MCP, or library validation
- WHEN it supplies the safe normalized summary to reusable Animus logic
- THEN Animus core MUST consume values only
- AND it MUST NOT spawn Cairn, discover sibling checkouts, read receipt files, or contact a server itself.

### Requirement: Verification feedback is actionable and safe

r[agent_harness_runtime.authoritative_completion.feedback] Non-passing validation MUST produce bounded typed diagnostics suitable for a controlled follow-up turn without exposing raw transcripts, secrets, or unbounded command output.

#### Scenario: Failed check feeds a reproducible follow-up

r[agent_harness_runtime.authoritative_completion.feedback.failed]
- GIVEN a selected check has a current non-pass receipt and safe producer diagnostics
- WHEN Animus builds validation follow-up work
- THEN each diagnostic MUST preserve stable diagnostic ID/code, check ID/status, applicable subject/registry/selection/evidence/receipt hashes, severity, remediation label and bounded summary, exact reproduction argv, safe working-root label, and safe artifact refs
- AND it SHOULD preserve a bounded repository-relative source path and line/column span when supplied.

#### Scenario: Feedback dispatch is policy-controlled

r[agent_harness_runtime.authoritative_completion.feedback.dispatch]
- GIVEN actionable diagnostics exist
- WHEN continuation policy evaluates the next action
- THEN reusable logic MAY return typed validation follow-up work with an exact work ID
- BUT model dispatch, retry count, edit authority, escalation, waiting, and cancellation MUST remain explicit host-policy decisions.

#### Scenario: Unsafe feedback is rejected

r[agent_harness_runtime.authoritative_completion.feedback.safe]
- GIVEN a diagnostic contains raw prompts, model/tool/provider payloads, credentials, environment values, private absolute paths, shell-expanded secrets, raw PTY data, or unbounded logs
- WHEN feedback validation runs
- THEN it MUST reject or safely redact the payload and retain only bounded labels, summaries, hashes, spans, argv, and artifact refs.

### Requirement: Continuation completion waits for verification

r[agent_harness_runtime.authoritative_completion.continuation] Continuation policy MUST distinguish no pending dispatchable work from verified completion whenever the active profile requires authoritative verification.

#### Scenario: Empty queue requests completion evaluation

r[agent_harness_runtime.authoritative_completion.continuation.empty]
- GIVEN no queued prompt, loop continuation, or validation follow-up is pending but the current work lacks a matching passing completion receipt
- WHEN continuation selection runs
- THEN it MUST select completion evaluation, blocked, or awaiting-evidence state
- AND it MUST NOT select verified `Complete` solely because queues are empty.

#### Scenario: Failed feedback remains bound to work ID

r[agent_harness_runtime.authoritative_completion.continuation.work_id]
- GIVEN a validation follow-up was created for one completion candidate
- WHEN a stale, duplicate, or mismatched follow-up acknowledgement arrives
- THEN existing exact-ID rules MUST reject it
- AND it MUST NOT clear the current completion blocker or install an unrelated receipt.

### Requirement: Workspace supervisor keeps progress separate from completion

r[agent_harness_runtime.authoritative_completion.workspace_supervisor] The workspace-root Cairn drain consumer MUST treat task-count reduction and bounded lifecycle mutation as progress observations only, while repository completion requires current receipt-authoritative evidence for the final lifecycle state.

#### Scenario: Task reduction is progress, not completion

r[agent_harness_runtime.authoritative_completion.workspace_supervisor.progress]
- GIVEN one agent round reduces unchecked tasks for one active change
- WHEN the workspace supervisor classifies the round
- THEN it MAY report `task-progress`
- BUT it MUST NOT report the repository complete while any active change or required non-passing check remains.

#### Scenario: Archive requires fresh linked receipts

r[agent_harness_runtime.authoritative_completion.workspace_supervisor.archive]
- GIVEN one matching active change disappears and an archive appears
- WHEN the supervisor evaluates archived progress
- THEN it MUST validate the archive and require current linked gate/validation and authoritative completion receipt identities for the final subject before reporting repository completion
- AND stale, blocked, skipped, deferred, timed-out, dry-run, infrastructure-failed, malformed, or missing evidence MUST keep the repository blocked.

#### Scenario: Exit and prose never establish completion

r[agent_harness_runtime.authoritative_completion.workspace_supervisor.non_authority]
- GIVEN Pi exits zero or emits completion prose
- WHEN lifecycle and receipt state did not authoritatively pass
- THEN the supervisor MUST report no completion
- AND it MAY continue only according to explicit blocker and independent-repository policy.

### Requirement: Authoritative completion preserves functional core and imperative shell

r[agent_harness_runtime.authoritative_completion.boundary] Animus MUST keep registry decoding/revalidation, selection, result admission, feedback reduction, and completion decisions pure while keeping VCS discovery, filesystem reads, process groups, timers, output capture, artifact persistence, environment and capability probes, and Cairn invocation in host shells.

#### Scenario: Core tests need no host world

r[agent_harness_runtime.authoritative_completion.boundary.pure]
- GIVEN registry, changed-artifact, result, diagnostic, continuation, and Cairn summary values in memory
- WHEN authoritative completion logic is tested
- THEN decisions MUST be deterministic without files, processes, environment, clocks, network, providers, credentials, terminals, daemons, or persistence.

#### Scenario: Shell cannot override core blocker

r[agent_harness_runtime.authoritative_completion.boundary.no_override]
- GIVEN pure completion reduction reports a required check or receipt blocker
- WHEN a host shell renders or persists the outcome
- THEN it MUST preserve the non-pass status and diagnostic identities
- AND it MUST NOT upgrade the result based on model text, process exit, task counters, archive presence, or unavailable capability.

### Requirement: Authoritative completion has positive and negative verification

r[agent_harness_runtime.authoritative_completion.verification] The change MUST include positive and negative registry, selector, execution-receipt, Steel, continuation, Cairn-adapter, feedback, freshness, safety, and workspace-supervisor fixtures.

#### Scenario: Positive end-to-end fixture passes

r[agent_harness_runtime.authoritative_completion.verification.positive]
- GIVEN a reviewed registry, final changed-artifact snapshot, complete required selection, fresh passing host receipts, exact completion candidate, and matching passing Cairn summary
- WHEN the bounded fixture flow runs
- THEN verified completion MUST pass with deterministic identities and safe receipts.

#### Scenario: Negative matrix fails closed

r[agent_harness_runtime.authoritative_completion.verification.negative]
- GIVEN fixtures for registry drift, unsafe argv, unknown path, empty selection, every non-pass status, duplicate/missing/unknown checks, failed prerequisites, post-check mutation, incidental `COMPLETE` text, model self-attestation, missing/stale Cairn summary, unsafe diagnostics, stale work IDs, and archive-without-receipts
- WHEN focused tests and rails run
- THEN each fixture MUST produce its expected blocker without finalizing completion, dispatching an implicit retry, mutating files, escalating authority, pushing, or performing pull-request operations.

#### Scenario: Passing result remains bounded

r[agent_harness_runtime.authoritative_completion.verification.non_claims]
- GIVEN authoritative completion passes for a selected Animus scope
- WHEN its receipt and docs are rendered
- THEN they MUST state the exact tested scope and unexecuted/blocked tiers
- AND they MUST NOT claim check-oracle soundness, model correctness, whole-product correctness, production readiness, arbitrary-host support, release eligibility, push authority, or pull-request authority.
