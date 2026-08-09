{
  cases = [
    {name = "wrong-release", issue = 'cohort_mismatch, release = "1.0.0-beta.17"},
    {name = "relative-repository", issue = 'invalid_request, repository = "relative/path"},
    {name = "short-hash-prefix", issue = 'ambiguous_prefix, selector = "QRY"},
    {name = "ambiguous-hash-prefix", issue = 'ambiguous_prefix, selector = "QRYOAJLY"},
    {name = "malformed-hash", issue = 'invalid_identity, hash = "lowercase"},
    {name = "unlisted-channel", issue = 'invalid_channel, channel = "private"},
    {name = "unsafe-path", issue = 'unsafe_evidence, path = "../private"},
    {name = "unknown-diff-status", issue = 'invalid_status, operation = "future-status"},
    {name = "rich-log-needs-identity-authority", issue = 'unsafe_evidence, authors = ["private"]},
    {name = "prompt-like-output", issue = 'interactive_output, stderr = "Password:"},
    {name = "truncated-output", issue = 'bound_exceeded, stdout_truncated = true},
    {name = "protected-record", issue = 'protected_mutation_denied, operation = 'record},
    {name = "remote-push", issue = 'unsupported_operation, operation = "push"},
  ],
}
