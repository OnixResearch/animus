# Tasks

- [x] [serial] Add an `animus-runtime::subagents` facade with common child-run draft, kind, admission, diagnostic, and receipt DTOs. r[agent_harness_runtime.subagent_facade.boundary]
- [x] [depends:boundary] Implement lossless typed projections for recipe-backed subagents, fresh-context investigations, read-only side questions, and worker-team tasks. r[agent_harness_runtime.subagent_facade.lossless_projection]
- [x] [depends:lossless_projection] Add positive tests proving every existing child-run kind validates through the facade without execution side effects. r[agent_harness_runtime.subagent_facade.all_kinds]
- [x] [depends:lossless_projection] Add negative tests for missing kind-specific fields, unsupported child-run kinds, lossy diagnostics, and accidental execution attempts. r[agent_harness_runtime.subagent_facade.fail_closed]
- [x] [depends:all_kinds,fail_closed] Update docs and public exports to identify facade APIs and lower-level compatibility APIs. r[agent_harness_runtime.subagent_facade.verification]
