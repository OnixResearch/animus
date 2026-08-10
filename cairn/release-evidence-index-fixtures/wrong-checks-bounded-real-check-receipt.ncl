let fixture = import "../real-check-evidence-fixture-schema.ncl" in
let schema = fixture.index_schema in
{
  expected_issue = "RELEASE_EVIDENCE_INVALID_REAL_CHECK_RECEIPT",
  index_lines = std.string.join "\n" [
    fixture.head_line,
    schema.real_check_receipt_line (fixture.make_receipt schema.real_check_mode schema.pass fixture.synthetic_commit false fixture.synthetic_source_blake3 "nickel-config" schema.real_check_cairn_binding),
    fixture.claim_line,
    fixture.non_claim_lines,
  ],
}
