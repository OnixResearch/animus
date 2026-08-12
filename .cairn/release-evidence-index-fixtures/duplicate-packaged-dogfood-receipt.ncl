let schema = import "../release-evidence-index-schema.ncl" in
{
  expected_issue = "RELEASE_EVIDENCE_DUPLICATE_CHECK",
  index_lines = std.string.join "\n" [
    schema.head_line (schema.head "git" "0123456789abcdef" "main" false),
    schema.check_line (schema.check schema.packaged_headless_dogfood_smoke schema.pure schema.pass true),
    schema.check_line (schema.check schema.packaged_headless_dogfood_smoke schema.pure schema.pass true),
    schema.claim_line schema.packaged_headless_dogfood,
    schema.non_claim_line schema.live_providers,
    schema.non_claim_line schema.tui_daemon_dogfood,
    schema.non_claim_line schema.vm_checks,
    schema.non_claim_line schema.remote_services,
    schema.non_claim_line schema.plugin_runtime,
    schema.non_claim_line schema.production_readiness,
  ],
}
