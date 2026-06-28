# Tasks

- [ ] [serial] Define lifecycle hook event DTOs for session, prompt, tool, permission, compaction, subagent, and stop boundaries. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry]
- [ ] [depends:event_registry] Add positive event-routing tests and negative unknown-event tests. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry.routing]
- [ ] [serial] Add hook definition identity, BLAKE3 content hashes, trust scopes, managed status, and review decisions. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.trust_identity]
- [ ] [depends:trust_identity] Add negative tests proving changed hook definitions require a fresh trust decision unless managed policy covers the new hash. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.trust_identity.changed]
- [ ] [serial] Define event-scoped hook input and output contracts, including additional context, warnings, deny decisions, and continuation requests where supported. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.io_contract]
- [ ] [depends:io_contract] Add malformed-output and unsupported-effect negative tests for each hook effect class. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.io_contract.unsupported]
- [ ] [serial] Bind hook results to runtime policy without allowing direct adapter calls or sandbox weakening. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.policy_binding]
- [ ] [depends:policy_binding] Add policy-bypass negative tests for hook attempts to execute tools, rewrite denied actions, or suppress required approvals. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.policy_binding.bypass]
- [ ] [serial] Add hook outcome events and failure classes for skipped, untrusted, failed, denied, malformed, and unsupported hook outcomes. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.outcome_reporting]
- [ ] [depends:outcome_reporting] Add report tests proving hook inputs are redacted and hook identities remain auditable. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.outcome_reporting.redacted]
- [ ] [depends:event_registry.routing,trust_identity.changed,io_contract.unsupported,policy_binding.bypass,outcome_reporting.redacted] Update docs and run focused hook tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.openai_codex_hook_trust_lifecycle.event_registry]
