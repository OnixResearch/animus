# Positive production-path conformance fixture inventory.
let NonEmpty = std.contract.from_predicate (fun value => std.is_string value && value != "") in
let Fixture = {
  id | NonEmpty,
  family | NonEmpty,
  entrypoints | Array NonEmpty,
} in
{
  schema = "animus.native-evaluation-conformance-fixtures.v1",
  fixtures | Array Fixture = [
    { id = "taskset-exact-bounded", family = "taskset", entrypoints = ["plan_taskset"] },
    { id = "recipes-production-paths", family = "recipes", entrypoints = ["plan_environment", "plan_recipe", "reduce_pass_at_n"] },
    { id = "episode-ledger-reference", family = "episode", entrypoints = ["reduce_episode"] },
    { id = "ports-exact-observations", family = "ports", entrypoints = ["admit_service", "build_service_request"] },
    { id = "gateway-durable-order", family = "gateway", entrypoints = ["project_model_call", "admit_identical_request_coalescing", "admit_terminal_after_commit", "execute_evaluation_host"] },
    { id = "verifier-isolated-artifacts", family = "verifier", entrypoints = ["preflight_host"] },
    { id = "rewards-finite-attributed", family = "rewards", entrypoints = ["reduce_reward_dimensions"] },
    { id = "resume-exact-complete", family = "persistence", entrypoints = ["build_resume_key", "evaluate_resume"] },
    { id = "harbor-atif-safe-subset", family = "compatibility", entrypoints = ["plan_trials", "project_atif", "reduce_rewards"] },
    { id = "feedback-dataset-advisory", family = "downstream", entrypoints = ["validate_eval_feedback_plan", "evaluate_feedback_review", "export_chat_messages_row"] },
    { id = "receipts-metadata-only", family = "evidence", entrypoints = ["project_evaluation_receipt", "project_host_receipt"] },
  ],
}
