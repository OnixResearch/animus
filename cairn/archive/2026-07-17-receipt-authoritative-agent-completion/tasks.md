## Phase 1: Registry and contract foundation

- [x] [serial] Define the typed Nickel agent-check registry contract and Animus-owned rows for exact argv, affected paths/crates/surfaces, prerequisites, capabilities, tiers, budgets, artifacts, reproduction, docs, and positive/negative contracts. r[agent_harness_runtime.authoritative_completion.registry]
- [x] [serial] Wire the registry through `config/repo.ncl`, deterministic generated runtime data, schema fixtures, export freshness checks, and an exact repair command without weakening existing generated-config ownership. r[agent_harness_runtime.authoritative_completion.registry]
- [x] [serial] Add safe DTOs for changed-artifact snapshots, registry rows, selected/blocked/not-selected sets, execution results, diagnostics, completion candidates, Cairn validation summaries, and completion receipts. r[agent_harness_runtime.authoritative_completion.execution_receipts]
- [x] [serial] Extend result status vocabulary so only `passed` is passing and skipped/deferred/blocked/timeout/dry-run/cleanup/infrastructure/cancelled/unknown states remain explicit. r[agent_harness_runtime.authoritative_completion.pass_only]

## Phase 2: Pure selection and completion core

- [x] [serial] Implement deterministic change-aware required-check selection with prerequisite closure, conservative unknown-path fallback, capability blockers, and registry/selection/policy/final-subject hashes. r[agent_harness_runtime.authoritative_completion.selection]
- [x] [serial] Validate execution result shape, command identity, final-subject freshness, bounded output facts, artifacts, cleanup, receipt hashes, and unsafe payload rejection in reusable logic. r[agent_harness_runtime.authoritative_completion.execution_receipts]
- [x] [serial] Replace skipped-as-no-follow-up behavior with pass-only aggregate reduction and blocker/feedback intents for every required non-pass status. r[agent_harness_runtime.authoritative_completion.pass_only]
- [x] [serial] Implement the pure completion reducer over candidate, selection, host results, current subject, continuation state, and normalized Cairn summary; derive one coherent verified-complete or blocked outcome. r[agent_harness_runtime.authoritative_completion.receipt_gate]
- [x] [serial] Distinguish queue emptiness from verified completion and preserve exact work-ID acknowledgement behavior for completion evaluation and validation follow-ups. r[agent_harness_runtime.authoritative_completion.continuation]

## Phase 3: Steel and host-shell integration

- [x] [serial] Replace Ralph `string-contains?` completion with a typed completion-request action or exact compatibility marker that creates a candidate but cannot emit verified final response by itself. r[agent_harness_runtime.authoritative_completion.candidate]
- [x] [serial] Extend Nickel Steel profile policy, generated script hash, Rust decision DTO validation, and positive/negative tests for completion candidate admission and incidental-substring rejection. r[agent_harness_runtime.authoritative_completion.candidate]
- [x] [serial] Add injected host service boundaries for changed-path observation, bounded check execution, artifact persistence, final-subject hashing, and exact-pinned Cairn completion validation without granting authority to reusable crates. r[agent_harness_runtime.authoritative_completion.boundary]
- [x] [serial] Convert non-passing Cairn/check results into bounded diagnostics with stable IDs, hashes, source spans, remediation, reproduction argv, artifact refs, non-claims, and validation follow-up work IDs. r[agent_harness_runtime.authoritative_completion.feedback]
- [x] [serial] Enforce re-plan/revalidation after any admitted mutation and prevent old candidates or passing receipts from crossing final-subject changes. r[agent_harness_runtime.authoritative_completion.execution_receipts.stale]

## Phase 4: Workspace consumer and documentation

- [x] [serial] Publish the reviewed consumer handoff for canonical `../ralph-drain-stack.scm`: preserve task reduction as progress only and require fresh linked lifecycle plus authoritative completion receipts before repository completion. r[agent_harness_runtime.authoritative_completion.workspace_supervisor]
- [x] [serial] Update Ralph, controller continuation, scoped validation planner, focused rail, and agent operating docs with pass-only statuses, completion authority, feedback flow, reproduction safety, and bounded non-claims. r[agent_harness_runtime.authoritative_completion.feedback]
- [x] [serial] Add a focused authoritative-completion rail that validates registry export, selection, Steel candidate handling, result freshness, Cairn adapter parity, diagnostic safety, continuation state, and receipt determinism. r[agent_harness_runtime.authoritative_completion.verification]

## Phase 5: Positive, negative, and closeout evidence

- [x] [parallel] Add positive fixtures for reviewed registry rows, direct/prerequisite selection, conservative baseline, passing execution receipts, exact completion candidates, matching Cairn summaries, feedback acknowledgement, final-subject stability, and verified completion. r[agent_harness_runtime.authoritative_completion.verification.positive]
- [x] [parallel] Add negative fixtures for registry/export drift, unsafe argv/artifacts, unknown path with no fallback, empty selection, every non-pass status, duplicate/missing/unknown checks, failed prerequisites, post-check mutation, incidental `COMPLETE`, model self-attestation, missing/stale Cairn summary, unsafe diagnostics, stale IDs, and archive-without-receipts. r[agent_harness_runtime.authoritative_completion.verification.negative]
- [x] [serial] Run baseline focused tests before core changes, then run the registry rail, focused planner/runtime/Steel/continuation tests, `scripts/cairn/check-steel-runtime-policy --root .`, generated-config checks, workspace checks, clippy as warranted, stack-helper self-tests for the consumer handoff, Cairn validation, and proposal/design/tasks gates. r[agent_harness_runtime.authoritative_completion.verification]
- [x] [serial] Record unavailable host-specific checks as blockers or unexecuted tiers, cite current safe receipt hashes, and preserve non-claims before sync/archive. r[agent_harness_runtime.authoritative_completion.verification.non_claims]
