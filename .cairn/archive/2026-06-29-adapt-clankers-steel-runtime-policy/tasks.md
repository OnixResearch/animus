# Tasks

- [x] [serial] Add `policy/steel-runtime/` Nickel contracts for profile identity, script bindings, BLAKE3 script hashes, budgets, allowed host actions, fallback mode, and receipt redaction. r[agent_harness_runtime.clankers_steel_policy.nickel]
- [x] [depends:clankers_steel_policy.nickel] Add exported policy data or typed fixtures consumed by Rust tests without runtime Nickel evaluation. r[agent_harness_runtime.clankers_steel_policy.export]
- [x] [depends:clankers_steel_policy.export] Add invalid-policy fixtures for missing scripts, bad names, non-positive budgets, unknown actions, hash mismatches, path escapes, and unsupported fallback modes. r[agent_harness_runtime.clankers_steel_policy.invalid_fixtures]
- [x] [depends:clankers_steel_policy.export] Enforce selected profile, script hash, budget, action allowlist, fallback mode, and receipt redaction before Steel evaluation. r[agent_harness_runtime.clankers_steel_policy.enforced]
- [x] [depends:clankers_steel_policy.enforced] Add redacted activation receipts that include safe hashes/status only. r[agent_harness_runtime.clankers_steel_policy.receipts]
- [x] [depends:clankers_steel_policy.invalid_fixtures,clankers_steel_policy.receipts] Add focused policy drift and activation rails with reviewable evidence. r[agent_harness_runtime.clankers_steel_policy.rails]
- [x] [depends:clankers_steel_policy.rails] Run focused rails, Steel tests, Nickel/config drift checks, workspace tests if touched, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_steel_policy.closeout]
