let Case = {
  id | String,
  issue | String,
  blocked_before_reduction | Bool,
}
in
[
  { id = "missing-service", issue = "absent", blocked_before_reduction = true },
  { id = "denied-capability", issue = "capability-widening", blocked_before_reduction = true },
  { id = "stale-correlation", issue = "correlation-mismatch", blocked_before_reduction = true },
  { id = "wrong-plan-task-attempt", issue = "binding-mismatch", blocked_before_reduction = true },
  { id = "shared-verifier-environment", issue = "verifier-isolation", blocked_before_reduction = true },
  { id = "artifact-manifest-widening", issue = "artifact-mismatch", blocked_before_reduction = true },
  { id = "early-terminal-event", issue = "commit-order", blocked_before_reduction = true },
  { id = "content-only-retry", issue = "unsafe-coalescing", blocked_before_reduction = true },
  { id = "malformed-observation", issue = "incomplete-result", blocked_before_reduction = true },
  { id = "storage-evidence-mismatch", issue = "storage-contradiction", blocked_before_reduction = true },
  { id = "raw-provider-body", issue = "receipt-leak", blocked_before_reduction = true },
  { id = "release-overclaim", issue = "receipt-overclaim", blocked_before_reduction = true },
] | Array Case
