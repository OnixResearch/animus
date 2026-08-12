let schema = import "release-evidence-index-schema.ncl" in
let commit = "0123456789abcdef" in
let source_blake3 = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" in
let receipt = fun mode status head dirty source selected_ids cairn_binding =>
  schema.real_check_receipt
    schema.real_check_receipt_schema
    schema.real_check_rail
    mode
    status
    head
    dirty
    source
    selected_ids
    cairn_binding
in
{
  index_schema = schema,
  synthetic_commit = commit,
  synthetic_source_blake3 = source_blake3,
  make_receipt = receipt,
  head_line = schema.head_line (schema.head "git" commit "main" false),
  claim_line = schema.claim_line schema.bounded_real_check_completion,
  non_claim_line = schema.non_claim_line schema.bounded_real_check_completion,
  non_claim_lines = std.string.join "\n" [
    schema.non_claim_line schema.live_providers,
    schema.non_claim_line schema.tui_daemon_dogfood,
    schema.non_claim_line schema.vm_checks,
    schema.non_claim_line schema.remote_services,
    schema.non_claim_line schema.plugin_runtime,
    schema.non_claim_line schema.production_readiness,
  ],
}
