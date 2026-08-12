let fixture = import "../real-check-evidence-fixture-schema.ncl" in
{
  expected_issue = "RELEASE_EVIDENCE_MALFORMED_RECORD",
  index_lines = std.string.join "\n" [
    fixture.head_line,
    "real_check_receipt|too-few-fields",
    fixture.claim_line,
    fixture.non_claim_lines,
  ],
}
