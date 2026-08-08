let Case = {
  id | String,
  service | [| 'task_source, 'agent_execution, 'verifier_execution, 'artifact_collect, 'artifact_restore, 'model_call_capture, 'result_append, 'result_replace, 'result_reopen |],
  outcome | [| 'admitted, 'observed |],
  bounded | Bool,
}
in
[
  { id = "all-separate-ports", service = 'task_source, outcome = 'admitted, bounded = true },
  { id = "exact-agent-observation", service = 'agent_execution, outcome = 'observed, bounded = true },
  { id = "isolated-verifier-observation", service = 'verifier_execution, outcome = 'observed, bounded = true },
  { id = "bounded-artifact-collect", service = 'artifact_collect, outcome = 'observed, bounded = true },
  { id = "bounded-artifact-restore", service = 'artifact_restore, outcome = 'observed, bounded = true },
  { id = "commit-ordered-model-capture", service = 'model_call_capture, outcome = 'observed, bounded = true },
  { id = "exact-result-append", service = 'result_append, outcome = 'observed, bounded = true },
  { id = "exact-result-replace", service = 'result_replace, outcome = 'observed, bounded = true },
  { id = "exact-result-reopen", service = 'result_reopen, outcome = 'observed, bounded = true },
] | Array Case
