let fixture = import "../real-check-evidence-fixture-schema.ncl" in
let schema = fixture.index_schema in
{
  expected_issue = "RELEASE_EVIDENCE_DIRTY_HEAD",
  index_lines = std.string.join "\n" [
    schema.head_line (schema.head "git" fixture.synthetic_commit "main" true),
    schema.real_check_receipt_line (fixture.make_receipt schema.real_check_mode schema.pass fixture.synthetic_commit false fixture.synthetic_source_blake3 schema.real_check_selected_ids schema.real_check_cairn_binding),
    fixture.claim_line,
    fixture.non_claim_lines,
  ],
}
