let family_lines_value = "family|fixture|crates/fixture/src/lib.rs|stable|injected authority boundary|safe receipt behavior|no bridge required|fixture focused surface|cargo test fixture|fixture_positive|fixture_negative|family:fixture"
in
let cases = [
  "case|fixture|happy_path|covered|fixture_positive|",
  "case|fixture|missing_service|covered|fixture_negative|",
  "case|fixture|denied_capability|not-applicable||fixture has no policy gate",
  "case|fixture|unavailable_service|not-applicable||fixture has no service availability state",
  "case|fixture|timeout|not-applicable||fixture has no clock authority",
  "case|fixture|cancellation|not-applicable||fixture has no cancellation handle",
  "case|fixture|malformed_output|not-applicable||fixture has no parser output",
  "case|fixture|oversized_output|not-applicable||fixture has no large output shape",
  "case|fixture|unsafe_reference|not-applicable||fixture has no references",
  "case|fixture|schema_mismatch|not-applicable||fixture has no schema version",
  "case|fixture|duplicate_field|not-applicable||fixture has no keyed fields",
  "case|fixture|unknown_field|not-applicable||fixture has no open fields",
  "case|fixture|leak_rejection|not-applicable||fixture has no receipt text",
]
in
{
  expected_issue = "OK",
  family_lines = family_lines_value,
  case_lines = std.string.join "\n" cases,
}
