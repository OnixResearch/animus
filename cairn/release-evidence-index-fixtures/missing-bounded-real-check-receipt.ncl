let fixture = import "../real-check-evidence-fixture-schema.ncl" in
let schema = fixture.index_schema in
{
  expected_issue = "RELEASE_EVIDENCE_MISSING_REAL_CHECK_RECEIPT",
  index_lines = std.string.join "\n" [
    fixture.head_line,
    schema.check_line (schema.check "focused-rails" schema.pure schema.pass true),
    fixture.claim_line,
    fixture.non_claim_lines,
  ],
}
