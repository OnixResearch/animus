let family_lines_value = "family|fixture|crates/fixture/src/lib.rs|stable|injected authority boundary|safe receipt behavior|no bridge required|fixture focused surface|cargo test fixture|fixture_positive|fixture_negative|family:fixture"
in
let cases = [
  "case|fixture|happy_path|covered|fixture_positive|",
  "case|fixture|missing_service|not-applicable||fixture claims no missing service state",
  "case|fixture|denied_capability|not-applicable||fixture claims no policy gate",
  "case|fixture|unavailable_service|not-applicable||fixture claims no service availability state",
  "case|fixture|timeout|not-applicable||fixture claims no clock authority",
  "case|fixture|cancellation|not-applicable||fixture claims no cancellation handle",
  "case|fixture|malformed_output|not-applicable||fixture claims no parser output",
  "case|fixture|oversized_output|not-applicable||fixture claims no large output shape",
  "case|fixture|unsafe_reference|not-applicable||fixture claims no references",
  "case|fixture|schema_mismatch|not-applicable||fixture claims no schema version",
  "case|fixture|duplicate_field|not-applicable||fixture claims no keyed fields",
  "case|fixture|unknown_field|not-applicable||fixture claims no open fields",
  "case|fixture|leak_rejection|not-applicable||fixture claims no receipt text",
]
in
{
  expected_issue = "ADAPTER_CONFORMANCE_MISSING_NEGATIVE_COVERAGE",
  family_lines = family_lines_value,
  case_lines = std.string.join "\n" cases,
}
