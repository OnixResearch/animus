let GuardedCapability = {
  action | String,
  contract | String,
  resource_prefix | String,
  ability | String,
} in
let CapabilityPolicy = {
  schema | String,
  contracts | Array GuardedCapability,
} in
let policy_schema = "animus.capability-policy.v1" in
let contract_id = "animus.runtime" in
{
  capability_policy | CapabilityPolicy = {
    schema = policy_schema,
    contracts = [
      {
        action = "model",
        contract = contract_id,
        resource_prefix = "model:",
        ability = "call",
      },
      {
        action = "tool",
        contract = contract_id,
        resource_prefix = "tool:",
        ability = "invoke",
      },
      {
        action = "filesystem",
        contract = contract_id,
        resource_prefix = "fs:workspace/",
        ability = "read",
      },
      {
        action = "exec",
        contract = contract_id,
        resource_prefix = "exec:",
        ability = "run",
      },
      {
        action = "network",
        contract = contract_id,
        resource_prefix = "net:",
        ability = "connect",
      },
      {
        action = "steel",
        contract = contract_id,
        resource_prefix = "steel:",
        ability = "evaluate",
      },
    ],
  },
}
