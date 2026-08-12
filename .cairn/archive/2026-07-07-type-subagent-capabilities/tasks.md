# Tasks

- [x] [serial] Add typed subagent capability variants and catalog-backed string conversion helpers. r[agent_harness_runtime.subagent_typed_capabilities.types]
- [x] [depends:types] Update subagent capability intersection to operate over typed values and return per-ceiling denial diagnostics. r[agent_harness_runtime.subagent_typed_capabilities.intersection]
- [x] [depends:types] Add compatibility shims that preserve stable adapter string labels and receipt labels at boundaries. r[agent_harness_runtime.subagent_typed_capabilities.compatibility]
- [x] [depends:types,intersection] Add negative tests for unknown labels, duplicate normalized labels, wildcard authority, malformed catalog entries, unauthorized tools, and label collisions. r[agent_harness_runtime.subagent_typed_capabilities.fail_closed]
- [x] [depends:compatibility,fail_closed] Update docs and focused typed-capability verification evidence. r[agent_harness_runtime.subagent_typed_capabilities.verification]
