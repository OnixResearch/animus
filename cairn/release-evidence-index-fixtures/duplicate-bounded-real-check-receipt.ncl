let fixture = import "../real-check-evidence-fixture-schema.ncl" in
let schema = fixture.index_schema in
let receipt_line = schema.real_check_receipt_line (fixture.make_receipt schema.real_check_mode schema.pass fixture.synthetic_commit false fixture.synthetic_source_blake3 schema.real_check_selected_ids schema.real_check_cairn_binding) in
{
  expected_issue = "RELEASE_EVIDENCE_DUPLICATE_REAL_CHECK_RECEIPT",
  index_lines = std.string.join "\n" [
    fixture.head_line,
    receipt_line,
    receipt_line,
    fixture.claim_line,
    fixture.non_claim_lines,
  ],
}
