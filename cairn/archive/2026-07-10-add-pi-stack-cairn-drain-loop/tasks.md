## Phase 1: Pure stack planning and progress classification

- [x] [serial] Add deterministic direct-child native Cairn repository discovery, explicit repository ordering/filtering, bounded round selection, and plan rendering. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.discovery]
- [x] [serial] Add pure before/after lifecycle classification for archived progress, single-change task progress, no progress, and fail-closed scope breaches. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.progress]
- [x] [serial] Add positive and negative self-tests for selection, filtering, task parsing, progress classification, path handling, and disallowed Pi session-reuse arguments. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.tests]

## Phase 2: Pi host shell

- [x] [serial] Add explicit `--run` dispatch through fresh named `pi -p` sessions with one-change prompts, repository-local instructions, no push/PR behavior, and bounded blocker handling. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.pi_host]
- [x] [serial] Require a new matching archive entry plus canonical Cairn validation after archived progress, continue independent repositories after local blockers, and stop globally on scope or validation failures. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.verification]

## Phase 3: Documentation and closeout

- [x] [parallel] Document usage, plan-only defaults, Pi session behavior, safety boundaries, and the cross-repository dependency non-claim in `README.md`, `AGENTS.md`, and `docs/ralph-loop.md`. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.docs]
- [x] [serial] Run the Steel self-test, inspect a local stack plan, run Cairn proposal/design/tasks gates and validation, verify preserved user changes, then sync and archive this change. r[agent_harness_runtime.ralph_stack_drain.pi_supervisor.closeout]
