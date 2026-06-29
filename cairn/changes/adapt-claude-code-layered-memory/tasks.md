# Tasks

- [ ] [serial] Define typed instruction and learned-memory descriptors with ID, source, trust scope, precedence, freshness, inclusion policy, and redaction metadata. r[agent_harness_runtime.claude_code_memory.descriptors]
- [ ] [depends:descriptors] Add negative descriptor validation tests for missing source, missing trust scope, unsupported inclusion policy, stale freshness states, and unsafe body metadata. r[agent_harness_runtime.claude_code_memory.descriptors.invalid]
- [ ] [serial] Implement pure deterministic prompt assembly over in-memory descriptors and selected policy. r[agent_harness_runtime.claude_code_memory.prompt_assembly]
- [ ] [depends:prompt_assembly] Add positive layer-composition tests and negative conflict diagnostics that preserve descriptor IDs without leaking redacted bodies. r[agent_harness_runtime.claude_code_memory.prompt_assembly.conflict]
- [ ] [serial] Add host-admitted learned-memory candidate flow with typed admitted, denied, and unavailable-admission statuses. r[agent_harness_runtime.claude_code_memory.admission]
- [ ] [depends:admission] Add tests proving denied or missing admission prevents future prompt influence and persistence. r[agent_harness_runtime.claude_code_memory.admission.denied]
- [ ] [serial] Define swappable memory source, store, importer, extraction, and admission adapter boundaries outside reusable core contracts. r[agent_harness_runtime.claude_code_memory.adapter_boundary]
- [ ] [depends:adapter_boundary] Add backend-swap and missing-adapter tests proving reusable logic consumes injected descriptor/admission DTOs instead of concrete memory stores. r[agent_harness_runtime.claude_code_memory.adapter_boundary.swappable]
- [ ] [serial] Define safe memory influence receipts with source labels, trust/freshness/inclusion status, redaction classes, and BLAKE3 body hashes. r[agent_harness_runtime.claude_code_memory.receipts]
- [ ] [depends:receipts] Add leak-rejection tests for raw credentials, provider payloads, absolute paths, unbounded prompt text, and redacted memory bodies. r[agent_harness_runtime.claude_code_memory.receipts.redacted]
- [ ] [serial] Document Claude Code memory as a bounded reference and keep any `CLAUDE.md` importer outside canonical core contracts. r[agent_harness_runtime.claude_code_memory.scope]
- [ ] [depends:scope,descriptors.invalid,prompt_assembly.conflict,admission.denied,adapter_boundary.swappable,receipts.redacted] Run focused memory tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.claude_code_memory.scope.compat_import]
