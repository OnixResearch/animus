let Family = {
  id | String,
  api_style | String,
  owner_module | String,
  support_claim | String,
}
in
let Case = {
  family_id | String,
  feature | String,
  status | String,
  evidence | String,
  reason | String,
}
in
let covered_status = "covered" in
let unsupported_status = "unsupported" in
let not_applicable_status = "not-applicable" in
let family = fun family_id api_style_value support_claim_value => {
  id = family_id,
  api_style = api_style_value,
  owner_module = "crates/animus-adapters/src/model/provider_conformance.rs",
  support_claim = support_claim_value,
}
in
let covered = fun family_id_value feature_value evidence_value => {
  family_id = family_id_value,
  feature = feature_value,
  status = covered_status,
  evidence = evidence_value,
  reason = "",
}
in
let unsupported = fun family_id_value feature_value reason_value => {
  family_id = family_id_value,
  feature = feature_value,
  status = unsupported_status,
  evidence = "",
  reason = reason_value,
}
in
let not_applicable = fun family_id_value feature_value reason_value => {
  family_id = family_id_value,
  feature = feature_value,
  status = not_applicable_status,
  evidence = "",
  reason = reason_value,
}
in
let family_line = fun record => "family|%{record.id}|%{record.api_style}|%{record.owner_module}|%{record.support_claim}"
in
let case_line = fun record => "case|%{record.family_id}|%{record.feature}|%{record.status}|%{record.evidence}|%{record.reason}"
in
let family_entries = [
  family "openai_responses" "OpenAI Responses" "literal request, canonical response normalization, and streaming parser-entrypoint fixtures are covered",
  family "openai_chat" "OpenAI-compatible chat or local HTTP" "literal request, canonical response normalization, and streaming parser-entrypoint fixtures are covered",
  family "anthropic_messages" "Anthropic Messages" "literal request, canonical response normalization, and streaming parser-entrypoint fixtures are covered",
  family "mock" "deterministic mock/local fixture" "literal request, canonical response normalization, and streaming parser-entrypoint fixtures are covered",
  family "future_provider_template" "future provider family" "no support is claimed until the family adds literal fixtures and explicit reasons",
]
in
let shared_request_evidence = "model::provider_conformance_tests::request_projection_matches_literal_fixtures_for_claimed_families"
in
let shared_lossy_evidence = "model::provider_conformance_tests::request_projection_fails_closed_for_lossy_provider_support"
in
let request_leak_evidence = "model::provider_conformance_tests::request_leaks_and_malformed_metadata_fail_before_provider_projection"
in
let response_positive_evidence = "model::provider_conformance_tests::response_normalization_preserves_supported_semantics"
in
let response_negative_evidence = "model::provider_conformance_tests::response_normalization_fails_closed_for_malformed_and_leaky_fixtures"
in
let streaming_positive_evidence = "model::provider_conformance_tests::streaming_fixtures_use_the_production_parser_entrypoint"
in
let streaming_negative_evidence = "model::provider_conformance_tests::streaming_fixtures_reject_malformed_or_unexpected_order"
in
let index_evidence = "model::provider_conformance_tests::conformance_index_requires_evidence_or_reason_by_status"
in
let case_entries = [
  covered "openai_responses" "literal_request_projection" shared_request_evidence,
  covered "openai_responses" "lossy_request_projection" shared_lossy_evidence,
  covered "openai_responses" "request_leak_rejection" request_leak_evidence,
  covered "openai_responses" "response_normalization" response_positive_evidence,
  covered "openai_responses" "response_fail_closed" response_negative_evidence,
  covered "openai_responses" "streaming_parser_entrypoint" streaming_positive_evidence,
  covered "openai_responses" "streaming_fail_closed" streaming_negative_evidence,
  covered "openai_responses" "index_validation" index_evidence,

  covered "openai_chat" "literal_request_projection" shared_request_evidence,
  covered "openai_chat" "lossy_request_projection" shared_lossy_evidence,
  covered "openai_chat" "request_leak_rejection" request_leak_evidence,
  covered "openai_chat" "response_normalization" response_positive_evidence,
  covered "openai_chat" "response_fail_closed" response_negative_evidence,
  covered "openai_chat" "streaming_parser_entrypoint" streaming_positive_evidence,
  covered "openai_chat" "streaming_fail_closed" streaming_negative_evidence,
  covered "openai_chat" "index_validation" index_evidence,

  covered "anthropic_messages" "literal_request_projection" shared_request_evidence,
  covered "anthropic_messages" "lossy_request_projection" shared_lossy_evidence,
  covered "anthropic_messages" "request_leak_rejection" request_leak_evidence,
  covered "anthropic_messages" "response_normalization" response_positive_evidence,
  covered "anthropic_messages" "response_fail_closed" response_negative_evidence,
  covered "anthropic_messages" "streaming_parser_entrypoint" streaming_positive_evidence,
  covered "anthropic_messages" "streaming_fail_closed" streaming_negative_evidence,
  covered "anthropic_messages" "index_validation" index_evidence,

  covered "mock" "literal_request_projection" shared_request_evidence,
  covered "mock" "lossy_request_projection" shared_lossy_evidence,
  covered "mock" "request_leak_rejection" request_leak_evidence,
  covered "mock" "response_normalization" response_positive_evidence,
  covered "mock" "response_fail_closed" response_negative_evidence,
  covered "mock" "streaming_parser_entrypoint" streaming_positive_evidence,
  covered "mock" "streaming_fail_closed" streaming_negative_evidence,
  covered "mock" "index_validation" index_evidence,

  unsupported "future_provider_template" "literal_request_projection" "future provider families must add reviewed literal request fixtures before claiming support",
  unsupported "future_provider_template" "response_normalization" "future provider families must add canonical response normalization fixtures before claiming support",
  unsupported "future_provider_template" "streaming_parser_entrypoint" "future streaming providers must feed raw chunks through the production parser entrypoint before claiming streaming support",
  not_applicable "future_provider_template" "live_provider_dispatch" "Animus does not include live provider clients, routers, credential lookup, or network dispatch in reusable adapter contracts",
]
in
{
  families | Array Family = family_entries,
  cases | Array Case = case_entries,
  family_lines = std.string.join "\n" (std.array.map family_line family_entries),
  case_lines = std.string.join "\n" (std.array.map case_line case_entries),
}
