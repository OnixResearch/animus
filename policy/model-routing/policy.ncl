let Role = {
  role_label | String,
  minimum_score | Number,
  downgrade_role_label | String,
} in
let Thresholds = {
  simple_max_score | Number,
  medium_max_score | Number,
} in
let ComplexityWeights = {
  token_bucket_weight | Number,
  file_bucket_weight | Number,
  tool_bucket_weight | Number,
  risk_bucket_weight | Number,
} in
let RoutingPolicy = {
  schema_version | Number,
  roles | Array Role,
  thresholds | Thresholds,
  weights | ComplexityWeights,
  missing_price_behavior | String,
  budget_behavior | String,
  policy_hash | String,
} in
let schema_version_value = 1 in
let simple_max_score_value = 4 in
let medium_max_score_value = 10 in
let economy_minimum_score_value = 0 in
let balanced_minimum_score_value = 5 in
let reasoning_minimum_score_value = 11 in
let token_bucket_weight_value = 2 in
let file_bucket_weight_value = 1 in
let tool_bucket_weight_value = 2 in
let risk_bucket_weight_value = 3 in
let no_downgrade_role_value = "" in
let deny_missing_price_value = "deny" in
let prefer_cheaper_then_deny_value = "prefer-cheaper-then-deny" in
let policy_hash_value = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc" in
{
  model_routing_policy | RoutingPolicy = {
    schema_version = schema_version_value,
    roles = [
      {
        role_label = "economy",
        minimum_score = economy_minimum_score_value,
        downgrade_role_label = no_downgrade_role_value,
      },
      {
        role_label = "balanced",
        minimum_score = balanced_minimum_score_value,
        downgrade_role_label = "economy",
      },
      {
        role_label = "deep-reasoning",
        minimum_score = reasoning_minimum_score_value,
        downgrade_role_label = "balanced",
      },
    ],
    thresholds = {
      simple_max_score = simple_max_score_value,
      medium_max_score = medium_max_score_value,
    },
    weights = {
      token_bucket_weight = token_bucket_weight_value,
      file_bucket_weight = file_bucket_weight_value,
      tool_bucket_weight = tool_bucket_weight_value,
      risk_bucket_weight = risk_bucket_weight_value,
    },
    missing_price_behavior = deny_missing_price_value,
    budget_behavior = prefer_cheaper_then_deny_value,
    policy_hash = policy_hash_value,
  },
}
