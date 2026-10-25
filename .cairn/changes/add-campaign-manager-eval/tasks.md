## Phase 1: Tasks and crate

- [x] [serial] Add the typed Nickel task set `config/campaign-manager-eval-tasks.ncl`, register it in the export, standalone request, manifest, and diff rails, and revalidate the generated projection in Rust. r[agent_harness_runtime.campaign_manager_eval.tasks]
- [x] [serial] Add the non-default `AGPL-3.0-or-later` crate `animus-campaign-manager-eval` over the Harbor slice. It depends on `animus-runtime` and `animus-std` with only `harbor-eval`, and on `animus-std-openai` only under `live`. r[agent_harness_runtime.campaign_manager_eval.boundary]

## Phase 2: Lane

- [x] [serial] Implement the environment service: open the task's campaign on fresh Choregraph history, alternate manager turns with host planner steps, run fenced selections against hidden outcomes, publish results and stops, and classify replies as admitted, unchanged, refused, or failed. r[agent_harness_runtime.campaign_manager_eval.trial]
- [x] [serial] Implement the separate verifier: reload the control projection from the trial's history, refuse missing, stale, foreign, or unknown trials, and grade `found`, `efficiency`, and the advisory `refusal-rate`. r[agent_harness_runtime.campaign_manager_eval.verifier]
- [x] [serial] Project each trial as a metadata-only ATIF-v1.7 trajectory through `project_atif`. r[agent_harness_runtime.campaign_manager_eval.trajectory]
- [x] [serial] Run jobs through `plan_trials`, `execute_separate_verifier`, `reduce_rewards`, and `receipt`, with per-route summaries and reported failures. r[agent_harness_runtime.campaign_manager_eval.job]
- [x] [serial] Add the evaluation binary with a metadata-only report, the scripted routes, and the opt-in `live` OpenAI or OpenRouter route that reads its credential from standard input. r[agent_harness_runtime.campaign_manager_eval.report] r[agent_harness_runtime.campaign_manager_eval.live]

## Phase 3: Evidence

- [x] [parallel] Add task-set tests: the checked set is admitted, and every kind of drift and bound violation is refused. r[agent_harness_runtime.campaign_manager_eval.tasks] r[agent_harness_runtime.campaign_manager_eval.verification]
- [x] [parallel] Add lane tests: the job separates the scripted routes; a trial reaches an offered target and projects its trajectory; a partial manager misses it; the verifier refuses missing, stale, foreign, and unknown trials; and unreachable routes and unknown tasks fail closed. r[agent_harness_runtime.campaign_manager_eval.trial] r[agent_harness_runtime.campaign_manager_eval.verifier] r[agent_harness_runtime.campaign_manager_eval.verification]
- [x] [parallel] Add the report test (metadata only, refused arguments) and the offline live-route construction test. r[agent_harness_runtime.campaign_manager_eval.report] r[agent_harness_runtime.campaign_manager_eval.live]
- [x] [serial] Add and pass the focused checks `cargo-campaign-manager-eval` and `campaign-manager-eval-octet`, and the `nickel-config` rail. r[agent_harness_runtime.campaign_manager_eval.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document the lane, the task model, the grading, the routes, the live invocation, and the non-claims in `docs/campaign-manager-eval.md` and `README.md`. r[agent_harness_runtime.campaign_manager_eval.boundary]
- [x] [serial] Register the change in `.cairn/roadmap.ncl` and pass the roadmap and Cairn validation rails. r[agent_harness_runtime.campaign_manager_eval.verification]
- [ ] [serial] After review, sync the requirements into `.cairn/specs/agent-harness-runtime/spec.md` and archive the change. r[agent_harness_runtime.campaign_manager_eval.verification]
