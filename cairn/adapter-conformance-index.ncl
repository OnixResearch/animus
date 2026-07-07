let FamilyRecord = {
  id | String,
  owner_module | String,
  stability | String,
  authority_boundary | String,
  receipt_behavior | String,
  serialization_bridge | String,
  focused_surface | String,
  focused_validation_check | String,
  positive_test | String,
  negative_test | String,
  docs_anchor | String,
}
in
let CaseRecord = {
  family_id | String,
  case_id | String,
  disposition | String,
  evidence | String,
  reason | String,
}
in
let stable = "stable" in
let covered_label = "covered" in
let not_applicable_label = "not-applicable" in
let family = fun family_id_value owner_module_value authority_boundary_value receipt_behavior_value serialization_bridge_value focused_surface_value focused_validation_check_value positive_test_value negative_test_value docs_anchor_value => {
  id = family_id_value,
  owner_module = owner_module_value,
  stability = stable,
  authority_boundary = authority_boundary_value,
  receipt_behavior = receipt_behavior_value,
  serialization_bridge = serialization_bridge_value,
  focused_surface = focused_surface_value,
  focused_validation_check = focused_validation_check_value,
  positive_test = positive_test_value,
  negative_test = negative_test_value,
  docs_anchor = docs_anchor_value,
}
in
let covered = fun family_id_value case_id_value evidence_value => {
  family_id = family_id_value,
  case_id = case_id_value,
  disposition = covered_label,
  evidence = evidence_value,
  reason = "",
}
in
let na = fun family_id_value case_id_value reason_value => {
  family_id = family_id_value,
  case_id = case_id_value,
  disposition = not_applicable_label,
  evidence = "",
  reason = reason_value,
}
in
let family_line = fun record => "family|%{record.id}|%{record.owner_module}|%{record.stability}|%{record.authority_boundary}|%{record.receipt_behavior}|%{record.serialization_bridge}|%{record.focused_surface}|%{record.focused_validation_check}|%{record.positive_test}|%{record.negative_test}|%{record.docs_anchor}"
in
let case_line = fun record => "case|%{record.family_id}|%{record.case_id}|%{record.disposition}|%{record.evidence}|%{record.reason}"
in
let family_entries = [
  family "model-router" "crates/animus-router/src/lib.rs" "pure model-routing decisions run before host dispatch and own no ambient authority" "router receipts carry safe labels, counts, redaction markers, and BLAKE3 identities only" "Preserves projection is absent by default and must revalidate DTOs when enabled" "animus_router::{Recipe,Signals,Budget,SelectedPlan,Receipt,select_recipe,validate_receipt_text}; animus_adapters::router transition shim" "cargo test -p animus-router --locked; cargo test -p animus-adapters router --locked" "test_support::positive::deterministic_selection_covers_recipe_families" "test_support::negative::router_receipt_leak_fixtures_fail_closed" "family:model-router",
  family "model" "crates/animus-adapters/src/model.rs" "host-injected ModelAdapter only after exact runtime capability admission; canonical model DTOs remain provider-neutral" "safe responses and canonical envelopes only; durable receipts remain hashed runtime or policy metadata" "Preserves bridge optional through animus-preserves; adapter DTOs stay no_std" "animus_adapters::{ModelAdapter,ModelRequest,ModelResponse,CanonicalModelRequest,CanonicalModelResponseEnvelope,ModelOutcomeAdapter}; animus_runtime::tool_prelude" "cargo test -p animus-adapters model --locked; cargo test -p animus-runtime shell::tests --locked" "model::canonical_tests::canonical_request_accepts_structured_conversation" "model::canonical_tests::canonical_response_rejects_impossible_combinations_and_leaks" "family:model",
  family "tool" "crates/animus-adapters/src/tool.rs" "host-injected ToolAdapter only after exact runtime capability admission and tool provenance checks" "tool outputs and mutation receipts are bounded or hashed; raw output references are rejected" "Preserves bridge optional through animus-preserves; tool DTOs stay no_std" "animus_adapters::{ToolAdapter,ToolRequest,ToolResponse,ToolOutcomeAdapter}; animus_runtime::tool_prelude" "cargo test -p animus-adapters tool::tests --locked; cargo test -p animus-runtime shell::tests --locked" "tool::tests::positive::request_accepts_valid_input" "tool::tests::negative::missing_executor_and_denied_capability_fail_without_fallback" "family:tool",
  family "host-status" "crates/animus-adapters/src/status" "host-injected HostStatusAdapter reports observations; pure runtime selection resolves precedence without ambient polling" "status evidence uses bounded safe hashes and redacted summaries only" "status DTOs are text-safe no_std contracts; no required serialization backend" "animus_adapters::{HostStatusAdapter,HostStatusRequest,HostStatusObservationBatch}; animus_runtime::status_prelude" "cargo test -p animus-adapters status::tests --locked; cargo test -p animus-runtime status::tests --locked" "status::tests::host_adapter_uses_typed_observation_contract" "status::tests::lifecycle_report_rejects_malformed_state" "family:host-status",
  family "workspace" "crates/animus-adapters/src/workspace" "workspace adapters are descriptors and plans; concrete VCS or filesystem mutation stays shell-owned" "workspace receipts must carry safe labels, hashes, and restore evidence without raw prompt or file bodies" "workspace DTOs are no_std contracts; host shells may serialize with their own backend" "animus_adapters::{WorkspaceAdapterDescriptor,WorkspaceIsolationPlan,WorkspaceReceipt}; documented workspace export set" "cargo test -p animus-adapters workspace::tests --locked" "workspace::tests::planning::plan_accepts_leaf_commit_and_receipt_hashes_are_stable" "workspace::tests::planning::plan_fails_closed_for_unavailable_partial_or_placeholder_adapters" "family:workspace",
  family "security-policy" "crates/animus-adapters/src/effect" "policy resolution is pure and denies by default; concrete enforcement authority is injected or shell-owned" "permission, authorization, confirmation, and sandbox receipts are redacted and leak-scanned" "policy DTOs are no_std contracts; external policy stores remain outside adapter core" "animus_adapters::{CapabilityPolicyContract,RuntimePolicyService,PermissionPolicy,SandboxPolicy}; animus_runtime::policy_prelude" "cargo test -p animus-adapters effect::tests --locked" "effect::permit::tests::positive::trusted_allow_policy_executes_after_authority_passes" "effect::permit::tests::negative::deny_policy_and_missing_authority_fail_before_dispatch" "family:security-policy",
  family "safe-receipt" "crates/animus-adapters/src/effect/audit.rs" "safe receipts are reusable DTOs and validators with no provider, filesystem, process, network, credential, or plugin authority" "safe receipts store bounded safe text, BLAKE3-like hashes, redaction labels, and leak classes only" "receipt validators are text contracts; bridge crates may encode them after revalidation" "animus_adapters::{Receipt,ReceiptDraft,SafeHash,SAFE_RECEIPT_CONTRACT_IDENTITY}; receipt validation functions" "cargo test -p animus-adapters effect::tests --locked; cargo test -p animus-adapters contract::tests --locked" "effect::tests::completed_service_outcome_carries_completed_receipt" "effect::tests::safe_hash_rejects_wrong_length" "family:safe-receipt",
]
in
let case_entries = [
  covered "model-router" "happy_path" "test_support::positive::deterministic_selection_covers_recipe_families",
  na "model-router" "missing_service" "model-router selection is pure data validation and does not dispatch services",
  covered "model-router" "denied_capability" "test_support::negative::deterministic_selection_denies_unsafe_eligibility",
  na "model-router" "unavailable_service" "model-router core owns no service availability state",
  na "model-router" "timeout" "model-router core owns no clock or deadline authority beyond labels",
  na "model-router" "cancellation" "model-router core owns no cancellation handle authority",
  covered "model-router" "malformed_output" "test_support::positive::output_contract_validation_fails_closed_or_falls_back",
  na "model-router" "oversized_output" "model-router receipts use bounded typed fields and safe hashes rather than raw output bodies",
  covered "model-router" "unsafe_reference" "test_support::negative::signal_projection_rejects_unsupported_labels",
  covered "model-router" "schema_mismatch" "test_support::negative::recipe_contracts_reject_invalid_shapes",
  na "model-router" "duplicate_field" "model-router constructors use typed scalar fields and arrays rather than open keyed maps",
  na "model-router" "unknown_field" "model-router constructors do not parse unbounded unknown fields",
  covered "model-router" "leak_rejection" "test_support::negative::router_receipt_leak_fixtures_fail_closed",

  covered "model" "happy_path" "model::canonical_tests::canonical_request_accepts_structured_conversation",
  covered "model" "missing_service" "runtime::shell::tests::model_absence_fails_closed_without_tool_call",
  covered "model" "denied_capability" "runtime::shell::tests::unrelated_model_capability_fails_before_adapter_call",
  covered "model" "unavailable_service" "runtime::shell::tests::typed_adapter_outcome_classes_are_preserved_in_failure_events",
  covered "model" "timeout" "runtime::shell::tests::typed_adapter_outcome_classes_are_preserved_in_failure_events",
  covered "model" "cancellation" "runtime::shell::tests::typed_adapter_outcome_classes_are_preserved_in_failure_events",
  covered "model" "malformed_output" "model::canonical_tests::canonical_response_rejects_missing_finish_and_malformed_usage",
  covered "model" "oversized_output" "model::canonical_tests::canonical_request_rejects_oversized_metadata_and_leaks",
  covered "model" "unsafe_reference" "model::canonical_tests::canonical_request_rejects_oversized_metadata_and_leaks",
  covered "model" "schema_mismatch" "model::canonical_tests::canonical_request_rejects_unsupported_content_and_missing_required_fields",
  na "model" "duplicate_field" "model request, response, and canonical DTO constructors use typed fields rather than open keyed maps",
  na "model" "unknown_field" "model request, response, and canonical constructors do not accept open key-value maps",
  covered "model" "leak_rejection" "model::canonical_tests::canonical_response_rejects_impossible_combinations_and_leaks",

  covered "tool" "happy_path" "tool::tests::positive::request_accepts_valid_input",
  covered "tool" "missing_service" "tool::tests::negative::missing_executor_and_denied_capability_fail_without_fallback",
  covered "tool" "denied_capability" "tool::tests::negative::missing_executor_and_denied_capability_fail_without_fallback",
  covered "tool" "unavailable_service" "runtime::shell::tests::typed_adapter_outcome_classes_are_preserved_in_failure_events",
  covered "tool" "timeout" "runtime::shell::tests::typed_adapter_outcome_classes_are_preserved_in_failure_events",
  covered "tool" "cancellation" "runtime::shell::tests::typed_adapter_outcome_classes_are_preserved_in_failure_events",
  covered "tool" "malformed_output" "tool::tests::negative::malformed_or_unknown_arguments_fail_after_preparation",
  covered "tool" "oversized_output" "tool::tests::positive::oversized_output_is_bounded_and_hashed",
  covered "tool" "unsafe_reference" "tool::tests::negative::unsafe_output_reference_is_rejected",
  covered "tool" "schema_mismatch" "contract::tests::parser_rejects_missing_or_unsupported_contract_versions",
  covered "tool" "duplicate_field" "tool::tests::negative::spans::hashed_edit_rejects_duplicate_and_unknown_tags",
  covered "tool" "unknown_field" "tool::tests::negative::malformed_or_unknown_arguments_fail_after_preparation",
  covered "tool" "leak_rejection" "tool::tests::negative::unsafe_output_reference_is_rejected",

  covered "host-status" "happy_path" "status::tests::host_adapter_uses_typed_observation_contract",
  covered "host-status" "missing_service" "runtime::status::tests::selection_falls_back_without_authoritative_observation",
  na "host-status" "denied_capability" "status observation DTOs carry reported state only; policy denial is covered by security-policy",
  na "host-status" "unavailable_service" "status observation DTOs are validated records and do not dispatch unavailable work",
  na "host-status" "timeout" "status observation DTOs do not own clocks or deadlines",
  na "host-status" "cancellation" "status observation DTOs do not own cancellation handles",
  covered "host-status" "malformed_output" "status::tests::lifecycle_report_rejects_malformed_state",
  covered "host-status" "oversized_output" "status::tests::lifecycle_report_rejects_invalid_evidence_hashes",
  covered "host-status" "unsafe_reference" "status::tests::lifecycle_report_rejects_unsupported_session_reference",
  covered "host-status" "schema_mismatch" "status::tests::lifecycle_report_rejects_malformed_source",
  covered "host-status" "duplicate_field" "status::tests::lifecycle_report_rejects_invalid_evidence_hashes",
  na "host-status" "unknown_field" "status constructors accept typed fields and do not parse open maps",
  covered "host-status" "leak_rejection" "status::tests::lifecycle_report_rejects_invalid_evidence_hashes",

  covered "workspace" "happy_path" "workspace::tests::planning::plan_accepts_leaf_commit_and_receipt_hashes_are_stable",
  covered "workspace" "missing_service" "workspace::tests::planning::plan_fails_closed_for_unavailable_partial_or_placeholder_adapters",
  covered "workspace" "denied_capability" "workspace::tests::planning::plan_rejects_non_leaf_mutation_and_capability_gaps",
  covered "workspace" "unavailable_service" "workspace::tests::planning::plan_fails_closed_for_unavailable_partial_or_placeholder_adapters",
  na "workspace" "timeout" "workspace planning DTOs do not own clocks; host shells must map deadlines to adapter outcomes",
  na "workspace" "cancellation" "workspace planning DTOs do not own cancellation handles; host shells must map cancellation to adapter outcomes",
  covered "workspace" "malformed_output" "workspace::tests::planning::plan_rejects_invalid_graphs_before_adapter_dispatch",
  covered "workspace" "oversized_output" "workspace::tests::handoff::handoff_rejects_empty_included_resource_manifest",
  covered "workspace" "unsafe_reference" "workspace::tests::receipt::validation_rejects_raw_evidence",
  covered "workspace" "schema_mismatch" "workspace::tests::isolation::isolated_workspace_rejects_git_only_requirement_when_adapter_lacks_it",
  covered "workspace" "duplicate_field" "workspace::tests::planning::plan_rejects_invalid_graphs_before_adapter_dispatch",
  covered "workspace" "unknown_field" "workspace::tests::automation::automation_contract_rejects_invalid_cadence_missing_target_and_unadmitted_full_access",
  covered "workspace" "leak_rejection" "workspace::tests::receipt::validation_rejects_raw_evidence",

  covered "security-policy" "happy_path" "effect::permit::tests::positive::trusted_allow_policy_executes_after_authority_passes",
  covered "security-policy" "missing_service" "effect::tests::confirmation::request::absent_broker_denies_by_default_without_raw_payload",
  covered "security-policy" "denied_capability" "effect::permit::tests::negative::deny_policy_and_missing_authority_fail_before_dispatch",
  covered "security-policy" "unavailable_service" "effect::authority::tests::negative::malformed_or_unavailable_sandbox_denies_after_grant_checks",
  covered "security-policy" "timeout" "call::tests::failure_classes_map_from_service_outcomes_without_treating_completed_as_failure",
  covered "security-policy" "cancellation" "call::tests::failure_classes_map_from_service_outcomes_without_treating_completed_as_failure",
  covered "security-policy" "malformed_output" "effect::permit::tests::negative::malformed_or_untrusted_policy_descriptors_are_rejected",
  covered "security-policy" "oversized_output" "effect::permit::tests::negative::precedence_and_receipt_redaction_bounds_are_enforced",
  covered "security-policy" "unsafe_reference" "effect::authority::tests::negative::receipt_validation_rejects_raw_material",
  covered "security-policy" "schema_mismatch" "effect::exec_policy::tests::negative::unsupported_decision_and_missing_fields_are_rejected",
  covered "security-policy" "duplicate_field" "effect::exec_policy::tests::negative::rule_examples_must_prove_match_and_not_match_boundaries",
  covered "security-policy" "unknown_field" "effect::permit::tests::negativekernel::pattern_rules_reject_bad_match_kind_and_regex",
  covered "security-policy" "leak_rejection" "effect::permit::tests::negative::permission_receipt_leak_fixtures_are_rejected",

  covered "safe-receipt" "happy_path" "effect::tests::completed_service_outcome_carries_completed_receipt",
  na "safe-receipt" "missing_service" "receipt validation is reusable data validation and does not dispatch services",
  na "safe-receipt" "denied_capability" "receipt validation owns no capability authority; policy denial is covered by security-policy",
  na "safe-receipt" "unavailable_service" "receipt validation has no service availability state",
  na "safe-receipt" "timeout" "receipt validation has no clock or deadline authority",
  na "safe-receipt" "cancellation" "receipt validation has no cancellation handle authority",
  covered "safe-receipt" "malformed_output" "effect::tests::safe_hash_rejects_wrong_length",
  covered "safe-receipt" "oversized_output" "effect::permit::tests::negative::precedence_and_receipt_redaction_bounds_are_enforced",
  covered "safe-receipt" "unsafe_reference" "contract::tests::parser_rejects_missing_or_unsupported_contract_versions",
  covered "safe-receipt" "schema_mismatch" "contract::tests::parser_rejects_missing_or_unsupported_contract_versions",
  na "safe-receipt" "duplicate_field" "safe receipt constructors use typed scalar fields and arrays rather than open keyed maps",
  na "safe-receipt" "unknown_field" "safe receipt constructors do not accept unbounded unknown fields",
  covered "safe-receipt" "leak_rejection" "effect::permit::tests::negative::permission_receipt_leak_fixtures_are_rejected",
]
in
{
  families | Array FamilyRecord = family_entries,
  cases | Array CaseRecord = case_entries,
  family_lines = std.string.join "\n" (std.array.map family_line family_entries),
  case_lines = std.string.join "\n" (std.array.map case_line case_entries),
}
