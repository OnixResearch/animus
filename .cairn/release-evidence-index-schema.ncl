let HeadRecord = {
  vcs | String,
  commit | String,
  branch | String,
  dirty | Bool,
}
in
let CheckRecord = {
  mode | String,
  class | String,
  status | String,
  head_match | Bool,
}
in
let MissingModeRecord = {
  mode | String,
  reason | String,
}
in
let RealCheckReceiptRecord = {
  schema | String,
  rail | String,
  mode | String,
  status | String,
  head | String,
  head_dirty | Bool,
  source_blake3 | String,
  selected_check_ids | String,
  cairn_binding | String,
}
in
let mk_head = fun vcs_value commit_value branch_value dirty_value =>
  {
    vcs = vcs_value,
    commit = commit_value,
    branch = branch_value,
    dirty = dirty_value,
  }
in
let mk_check = fun mode_value class_value status_value head_match_value =>
  {
    mode = mode_value,
    class = class_value,
    status = status_value,
    head_match = head_match_value,
  }
in
let mk_missing_mode = fun mode_value reason_value =>
  {
    mode = mode_value,
    reason = reason_value,
  }
in
let mk_real_check_receipt = fun schema_value rail_value mode_value status_value head_value head_dirty_value source_blake3_value selected_check_ids_value cairn_binding_value =>
  {
    schema = schema_value,
    rail = rail_value,
    mode = mode_value,
    status = status_value,
    head = head_value,
    head_dirty = head_dirty_value,
    source_blake3 = source_blake3_value,
    selected_check_ids = selected_check_ids_value,
    cairn_binding = cairn_binding_value,
  }
in
let render_bool_text = fun value => if value then "true" else "false" in
let render_head_line = fun record => "head|%{record.vcs}|%{record.commit}|%{record.branch}|%{render_bool_text record.dirty}" in
let render_check_line = fun record => "check|%{record.mode}|%{record.class}|%{record.status}|%{render_bool_text record.head_match}" in
let render_claim_line = fun surface => "claim|%{surface}" in
let render_non_claim_line = fun surface => "non_claim|%{surface}" in
let render_missing_mode_line = fun record => "missing_mode|%{record.mode}|%{record.reason}" in
let render_real_check_receipt_line = fun record => "real_check_receipt|%{record.schema}|%{record.rail}|%{record.mode}|%{record.status}|%{record.head}|%{render_bool_text record.head_dirty}|%{record.source_blake3}|%{record.selected_check_ids}|%{record.cairn_binding}" in
let pure_class = "pure" in
let optional_class_value = "optional" in
let pass_status = "pass" in
let missing_status = "missing" in
let mode_packaged_headless_dogfood_smoke = "packaged-headless-dogfood-smoke" in
let surface_current_head_pure_evidence = "current-head-pure-evidence" in
let surface_packaged_headless_dogfood = "packaged-headless-dogfood" in
let surface_bounded_real_check_completion = "bounded-real-check-completion" in
let real_check_receipt_schema_value = "animus.real-check-completion-host-smoke-receipt.v1" in
let real_check_rail_value = "real-check-completion-host-smoke" in
let real_check_mode_value = "live" in
let real_check_selected_ids_value = "nickel-config,cairn-validate" in
let real_check_cairn_binding_value = "pinned-cairn-core" in
let surface_live_providers = "live-providers" in
let surface_tui_daemon_dogfood = "tui-daemon-dogfood" in
let surface_vm_checks = "vm-checks" in
let surface_remote_services = "remote-services" in
let surface_plugin_runtime = "plugin-runtime" in
let surface_production_readiness = "production-readiness" in
let sample_head = mk_head "git" "0123456789abcdef" "main" false in

{
  HeadRecord = HeadRecord,
  CheckRecord = CheckRecord,
  MissingModeRecord = MissingModeRecord,
  RealCheckReceiptRecord = RealCheckReceiptRecord,
  head = mk_head,
  check = mk_check,
  missing_mode = mk_missing_mode,
  real_check_receipt = mk_real_check_receipt,
  head_line = render_head_line,
  check_line = render_check_line,
  claim_line = render_claim_line,
  non_claim_line = render_non_claim_line,
  missing_mode_line = render_missing_mode_line,
  real_check_receipt_line = render_real_check_receipt_line,
  pure = pure_class,
  optional_class = optional_class_value,
  pass = pass_status,
  missing = missing_status,
  packaged_headless_dogfood_smoke = mode_packaged_headless_dogfood_smoke,
  current_head_pure_evidence = surface_current_head_pure_evidence,
  packaged_headless_dogfood = surface_packaged_headless_dogfood,
  bounded_real_check_completion = surface_bounded_real_check_completion,
  real_check_receipt_schema = real_check_receipt_schema_value,
  real_check_rail = real_check_rail_value,
  real_check_mode = real_check_mode_value,
  real_check_selected_ids = real_check_selected_ids_value,
  real_check_cairn_binding = real_check_cairn_binding_value,
  live_providers = surface_live_providers,
  tui_daemon_dogfood = surface_tui_daemon_dogfood,
  vm_checks = surface_vm_checks,
  remote_services = surface_remote_services,
  plugin_runtime = surface_plugin_runtime,
  production_readiness = surface_production_readiness,
  sample_index_lines = std.string.join "\n" [
    render_head_line sample_head,
    render_check_line (mk_check "focused-rails" pure_class pass_status true),
    render_check_line (mk_check "workspace-tests" pure_class pass_status true),
    render_check_line (mk_check "clippy" pure_class pass_status true),
    render_check_line (mk_check "flake-check" pure_class pass_status true),
    render_check_line (mk_check "nickel-drift" pure_class pass_status true),
    render_check_line (mk_check "cairn-validate" pure_class pass_status true),
    render_check_line (mk_check "cairn-gates" pure_class pass_status true),
    render_check_line (mk_check mode_packaged_headless_dogfood_smoke pure_class pass_status true),
    render_claim_line surface_current_head_pure_evidence,
    render_non_claim_line surface_live_providers,
    render_non_claim_line surface_tui_daemon_dogfood,
    render_non_claim_line surface_vm_checks,
    render_non_claim_line surface_remote_services,
    render_non_claim_line surface_plugin_runtime,
    render_non_claim_line surface_production_readiness,
  ],
}
