let schema = import "../release-evidence-index-schema.ncl" in
{
  expected_issue = "RELEASE_EVIDENCE_MISSING_MODE",
  index_lines = std.string.join "\n" [
    schema.head_line (schema.head "git" "0123456789abcdef" "main" false),
    schema.check_line (schema.check "focused-rails" schema.pure schema.pass true),
    schema.check_line (schema.check "workspace-tests" schema.pure schema.pass true),
    schema.check_line (schema.check "flake-check" schema.pure schema.pass true),
    schema.check_line (schema.check "nickel-drift" schema.pure schema.pass true),
    schema.check_line (schema.check "cairn-validate" schema.pure schema.pass true),
    schema.check_line (schema.check "cairn-gates" schema.pure schema.pass true),
    schema.missing_mode_line (schema.missing_mode "clippy" "not-run"),
    schema.claim_line schema.current_head_pure_evidence,
    schema.non_claim_line schema.live_providers,
    schema.non_claim_line schema.tui_daemon_dogfood,
    schema.non_claim_line schema.vm_checks,
    schema.non_claim_line schema.remote_services,
    schema.non_claim_line schema.plugin_runtime,
    schema.non_claim_line schema.production_readiness,
  ],
}
