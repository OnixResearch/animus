{
  schema = "animus.execution-backend-selection-lattice-consumer.v1",
  lattice_revision = "090162f758fdb43c86c9d93d58e4348a478bd86e",
  accepted_fields = [
    "plan_id",
    "workload_id",
    "backend_id",
    "backend_class",
    "policy_identity",
    "requirement_classes",
    "resource_profile_id",
    "non_claims",
  ],
  rejected_fields = [
    "command",
    "credentials",
    "environment",
    "host_path",
    "provider_payload",
    "sandbox_diagnostics",
  ],
  non_claims = [
    "not-backend-availability",
    "not-sandbox-enforcement",
    "not-work-completion",
    "not-output-correctness",
  ],
}
