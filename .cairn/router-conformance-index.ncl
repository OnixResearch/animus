let RouterFixture = {
  id | String,
  kind | String,
  expected_issue | String,
  family | String,
  roles | Array String,
  output_contract | String,
  max_candidates | Number,
  max_parallelism | Number,
  min_success_quorum | Number,
  timeout_class | String,
  max_synthesis_attempts | Number,
  max_repair_attempts | Number,
  fallback_policy | String,
  backend | String,
} in
let ok = "OK" in
let positive_kind = "positive" in
let negative_kind = "negative" in
let family_single = "single" in
let family_confidence = "confidence" in
let family_ratings = "ratings" in
let family_remom = "remom" in
let family_fusion = "fusion" in
let family_workflow = "workflow" in
let role_primary = "primary" in
let role_escalation = "escalation" in
let role_rater = "rater" in
let role_synthesizer = "synthesizer" in
let role_judge = "judge" in
let role_finalizer = "finalizer" in
let role_worker = "worker" in
let output_json = "json" in
let deadline_standard = "standard" in
let deadline_extended = "extended" in
let fallback_deny = "deny" in
let fallback_best_valid_evidence = "best_valid_evidence" in
let fallback_safe_single = "safe_single" in
let backend_local = "backend:local" in
let backend_remote = "backend:remote" in
let issue_unknown_family = "ROUTER_POLICY_UNKNOWN_FAMILY" in
let issue_invalid_role_combo = "ROUTER_POLICY_INVALID_ROLE_COMBO" in
let issue_bound_out_of_range = "ROUTER_POLICY_BOUND_OUT_OF_RANGE" in
let issue_disallowed_fallback = "ROUTER_POLICY_DISALLOWED_FALLBACK" in
let issue_unknown_backend = "ROUTER_POLICY_UNKNOWN_BACKEND" in
let candidates_one = 1 in
let candidates_two = 2 in
let candidates_three = 3 in
let candidates_too_many = 9 in
let parallelism_one = 1 in
let parallelism_two = 2 in
let parallelism_three = 3 in
let quorum_one = 1 in
let quorum_two = 2 in
let synthesis_none = 0 in
let synthesis_one = 1 in
let repair_none = 0 in
let repair_one = 1 in
let positive = fun id_value family_value roles_value max_candidates_value max_parallelism_value min_success_quorum_value timeout_class_value max_synthesis_attempts_value max_repair_attempts_value fallback_policy_value backend_value => {
  id = id_value,
  kind = positive_kind,
  expected_issue = ok,
  family = family_value,
  roles = roles_value,
  output_contract = output_json,
  max_candidates = max_candidates_value,
  max_parallelism = max_parallelism_value,
  min_success_quorum = min_success_quorum_value,
  timeout_class = timeout_class_value,
  max_synthesis_attempts = max_synthesis_attempts_value,
  max_repair_attempts = max_repair_attempts_value,
  fallback_policy = fallback_policy_value,
  backend = backend_value,
} in
let negative = fun id_value expected_issue_value family_value roles_value max_candidates_value max_parallelism_value min_success_quorum_value timeout_class_value max_synthesis_attempts_value max_repair_attempts_value fallback_policy_value backend_value => {
  id = id_value,
  kind = negative_kind,
  expected_issue = expected_issue_value,
  family = family_value,
  roles = roles_value,
  output_contract = output_json,
  max_candidates = max_candidates_value,
  max_parallelism = max_parallelism_value,
  min_success_quorum = min_success_quorum_value,
  timeout_class = timeout_class_value,
  max_synthesis_attempts = max_synthesis_attempts_value,
  max_repair_attempts = max_repair_attempts_value,
  fallback_policy = fallback_policy_value,
  backend = backend_value,
} in
let fixture_records = [
  positive "valid-single" family_single [role_primary] candidates_one parallelism_one quorum_one deadline_standard synthesis_none repair_none fallback_deny backend_local,
  positive "valid-confidence" family_confidence [role_primary, role_escalation] candidates_two parallelism_two quorum_one deadline_standard synthesis_one repair_one fallback_best_valid_evidence backend_remote,
  positive "valid-ratings" family_ratings [role_primary, role_rater] candidates_three parallelism_three quorum_two deadline_standard synthesis_one repair_one fallback_best_valid_evidence backend_remote,
  positive "valid-remom" family_remom [role_worker, role_synthesizer] candidates_three parallelism_three quorum_two deadline_extended synthesis_one repair_one fallback_best_valid_evidence backend_remote,
  positive "valid-fusion" family_fusion [role_judge, role_finalizer] candidates_two parallelism_two quorum_one deadline_extended synthesis_one repair_one fallback_safe_single backend_remote,
  positive "valid-workflow" family_workflow [role_worker, role_finalizer] candidates_three parallelism_two quorum_one deadline_standard synthesis_one repair_one fallback_best_valid_evidence backend_remote,
  negative "unknown-family" issue_unknown_family "surprise" [role_primary] candidates_one parallelism_one quorum_one deadline_standard synthesis_none repair_none fallback_deny backend_local,
  negative "invalid-role-combo" issue_invalid_role_combo family_single [role_worker, role_finalizer] candidates_two parallelism_two quorum_one deadline_standard synthesis_one repair_one fallback_deny backend_local,
  negative "out-of-range-bound" issue_bound_out_of_range family_confidence [role_primary, role_escalation] candidates_too_many parallelism_three quorum_one deadline_standard synthesis_one repair_one fallback_best_valid_evidence backend_remote,
  negative "disallowed-fallback" issue_disallowed_fallback family_ratings [role_primary, role_rater] candidates_three parallelism_three quorum_two deadline_standard synthesis_one repair_one "unsafe_any" backend_remote,
  negative "unknown-backend" issue_unknown_backend family_remom [role_worker, role_synthesizer] candidates_three parallelism_three quorum_two deadline_extended synthesis_one repair_one fallback_best_valid_evidence "backend:surprise",
] in
let csv = fun values => std.string.join "," values in
let fixture_line = fun record =>
  "fixture|%{record.kind}|%{record.id}|%{record.expected_issue}|%{record.family}|%{csv record.roles}|%{record.output_contract}|%{record.max_candidates}|%{record.max_parallelism}|%{record.min_success_quorum}|%{record.timeout_class}|%{record.max_synthesis_attempts}|%{record.max_repair_attempts}|%{record.fallback_policy}|%{record.backend}"
in
{
  fixtures | Array RouterFixture = fixture_records,
  fixture_lines = std.string.join "\n" (std.array.map fixture_line fixture_records),
}
