{
  cases = [
    {name = "missing-manifest", issue = 'manifest_rejected},
    {name = "implicit-profile", issue = 'missing_profile},
    {name = "undeclared-scope", issue = 'undeclared_scope},
    {name = "unknown-declaration", issue = 'unknown_declaration},
    {name = "provider-fallback", issue = 'provider_fallback_denied},
    {name = "write-request", issue = 'write_denied},
    {name = "missing-provider-credential", issue = 'policy_rejected},
    {name = "missing-sops-executable", issue = 'provider_executable_missing},
    {name = "malformed-sops-output", issue = 'provider_output_malformed},
    {name = "nonzero-sops-exit", issue = 'provider_process_failed},
    {name = "worker-timeout", issue = 'timed_out},
    {name = "worker-cancelled", issue = 'cancelled},
    {name = "worker-output-overflow", issue = 'output_limit_exceeded},
    {name = "secret-in-receipt", issue = 'unsafe_evidence},
  ],
}
