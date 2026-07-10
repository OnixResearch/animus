let ScriptBinding = {
  label | String,
  hash | String,
} in
let RuntimeBudget = {
  max_turns | Number,
} in
let ReceiptRedaction = {
  mode | String,
} in
let SteelRuntimeProfile = {
  name | String,
  enabled | Bool,
  default | Bool,
  script | ScriptBinding,
  budget | RuntimeBudget,
  allowed_host_actions | Array String,
  fallback_mode | String,
  receipt_redaction | ReceiptRedaction,
} in
let SteelRuntimePolicy = {
  schema | String,
  profiles | Array SteelRuntimeProfile,
} in
let policy_schema = "animus.steel-runtime-policy.v1" in
let default_profile_name = "default-loop" in
let default_script_label = "scripts/default-loop.scm" in
let default_script_hash = "79d887ab1ffd350d191ad7aea6a10fe3de28f108d34271cc99e4a59e537e9d0e" in
let default_max_turns = 2 in
let ralph_profile_name = "ralph-loop" in
let ralph_script_label = "scripts/ralph-loop.scm" in
let ralph_script_hash = "0225dfc88a7cbfe9b1639a199dd961cecee66ecd311a2ff13929867f60b6ca77" in
let ralph_max_turns = 10 in
let runtime_host_actions = ["model", "tool", "respond", "stop"] in
let fallback_block = "block" in
let receipt_hash_only = "hash-only" in
{
  steel_runtime_policy | SteelRuntimePolicy = {
    schema = policy_schema,
    profiles = [
      {
        name = default_profile_name,
        enabled = true,
        default = true,
        script = {
          label = default_script_label,
          hash = default_script_hash,
        },
        budget = {
          max_turns = default_max_turns,
        },
        allowed_host_actions = runtime_host_actions,
        fallback_mode = fallback_block,
        receipt_redaction = {
          mode = receipt_hash_only,
        },
      },
      {
        name = ralph_profile_name,
        enabled = true,
        default = false,
        script = {
          label = ralph_script_label,
          hash = ralph_script_hash,
        },
        budget = {
          max_turns = ralph_max_turns,
        },
        allowed_host_actions = runtime_host_actions,
        fallback_mode = fallback_block,
        receipt_redaction = {
          mode = receipt_hash_only,
        },
      },
    ],
  },
}
