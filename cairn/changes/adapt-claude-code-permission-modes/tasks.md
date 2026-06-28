# Tasks

- [ ] [serial] Define permission policy descriptors for guarded action classes, resource labels, allow/ask/deny outcomes, source identity, trust scope, precedence, and prompt-summary policy. r[agent_harness_runtime.claude_code_permissions.policy]
- [ ] [depends:policy] Add negative validation tests for unknown action classes, missing resource labels, unsupported outcomes, missing source identity, and untrusted policy descriptors. r[agent_harness_runtime.claude_code_permissions.policy.invalid]
- [ ] [serial] Implement pure permission resolution over action, authority, selected policy, and prompt-decision inputs. r[agent_harness_runtime.claude_code_permissions.resolution]
- [ ] [depends:resolution] Add tests proving ask decisions return prompt-required before any model, tool, shell, filesystem, or network dispatch. r[agent_harness_runtime.claude_code_permissions.resolution.ask]
- [ ] [serial] Add host prompt request/response DTOs with redacted action summaries and scoped prompt-decision tokens. r[agent_harness_runtime.claude_code_permissions.prompt]
- [ ] [depends:prompt] Add negative tests for stale, mismatched, missing, malformed, and denied prompt-decision tokens. r[agent_harness_runtime.claude_code_permissions.prompt.stale]
- [ ] [serial] Define safe permission receipts with authority status, local outcome, prompt status, policy hash, reasons, and BLAKE3 resource or receipt hashes. r[agent_harness_runtime.claude_code_permissions.receipts]
- [ ] [depends:receipts] Add leak-rejection tests for raw prompts, credentials, provider payloads, raw tool bodies, full paths, and unbounded outputs. r[agent_harness_runtime.claude_code_permissions.receipts.denied]
- [ ] [serial] Document the bounded Claude Code permission adaptation and prove local allow cannot expand authority. r[agent_harness_runtime.claude_code_permissions.scope]
- [ ] [depends:scope.local_allow_denied,policy.invalid,resolution.ask,prompt.stale,receipts.denied] Run focused permission tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_permissions.scope]
