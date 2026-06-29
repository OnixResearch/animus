# Tasks

- [ ] [serial] Define permission mode, guarded action, resource label, pattern rule, and local outcome DTOs. r[agent_harness_runtime.zerostack_permission_kernel.modes]
- [ ] [depends:modes] Implement pure mode-default resolution that remains subordinate to supplied authority and sandbox inputs. r[agent_harness_runtime.zerostack_permission_kernel.modes.authority]
- [ ] [serial] Add validated glob-like and regex-like rule descriptors with source identity, trust scope, precedence, and typed diagnostics. r[agent_harness_runtime.zerostack_permission_kernel.pattern_rules]
- [ ] [depends:pattern_rules] Add negative tests for unknown match kinds, invalid regex, missing source identity, unsupported outcomes, and malformed action classes. r[agent_harness_runtime.zerostack_permission_kernel.pattern_rules.invalid]
- [ ] [serial] Define scoped session allowlist entries and token validation over action, resource, policy, and session identity. r[agent_harness_runtime.zerostack_permission_kernel.session_allowlist]
- [ ] [depends:session_allowlist] Add negative stale-token and mismatched-resource tests. r[agent_harness_runtime.zerostack_permission_kernel.session_allowlist.stale]
- [ ] [serial] Add pure repeated-action detection with policy-selected coach, ask, deny, and ignore outcomes. r[agent_harness_runtime.zerostack_permission_kernel.repeated_action]
- [ ] [depends:repeated_action] Add destructive repeated-action denial tests proving no adapter dispatch occurs. r[agent_harness_runtime.zerostack_permission_kernel.repeated_action.deny]
- [ ] [depends:modes,pattern_rules,session_allowlist,repeated_action] Add safe permission receipt DTOs and redaction tests for prompts, credentials, full paths, provider payloads, and tool bodies. r[agent_harness_runtime.zerostack_permission_kernel.receipts]
- [ ] [depends:receipts] Update docs and run focused permission tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_permission_kernel.receipts.redacted]
