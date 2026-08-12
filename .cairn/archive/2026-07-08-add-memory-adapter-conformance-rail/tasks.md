# Tasks

- [x] [serial] Define the memory adapter conformance matrix schema with backend family, adapter role, capabilities, owner, positive case, negative case, and safe evidence label. r[agent_harness_runtime.memory_conformance.matrix]
- [x] [depends:matrix] Add matrix validation fixtures that reject rows without negative coverage, owner, capability declaration, or safe evidence reason. r[agent_harness_runtime.memory_conformance.matrix.coverage]
- [x] [serial] Add null/no-op memory conformance cases for disabled memory use and generation. r[agent_harness_runtime.memory_conformance.null_backend]
- [x] [depends:null_backend] Add disabled-memory tests proving no storage, retrieval, prompt injection, background extraction, or ambient fallback occurs. r[agent_harness_runtime.memory_conformance.null_backend.disabled]
- [x] [serial] Add local file/markdown backend boundary cases that validate host-owned import/export descriptors and safe path redaction. r[agent_harness_runtime.memory_conformance.filesystem_backend]
- [x] [depends:filesystem_backend] Add leak tests proving file paths and raw memory bodies are absent from reusable receipts and ledgers. r[agent_harness_runtime.memory_conformance.filesystem_backend.path_redaction]
- [x] [serial] Add SQL/document and vector/hybrid backend cases using normalized record fixtures and backend-declared match reasons. r[agent_harness_runtime.memory_conformance.semantic_backend]
- [x] [depends:semantic_backend] Add unavailable vector/search service tests proving no keyword, provider, network, or ambient index fallback occurs unless an explicit alternate adapter is injected. r[agent_harness_runtime.memory_conformance.semantic_backend.unavailable]
- [x] [serial] Add MCP/remote memory adapter cases with scoped remote descriptors, redacted service IDs, timeout/unavailable statuses, and no credential leakage. r[agent_harness_runtime.memory_conformance.remote_backend]
- [x] [depends:remote_backend] Add remote failure fixtures for denied, timed-out, malformed, and unsafe-receipt outcomes. r[agent_harness_runtime.memory_conformance.remote_backend.failures]
- [x] [serial] Define shared memory failure taxonomy and map conformance fixtures to unavailable, denied, timed-out, malformed-record, stale, over-budget, redacted, conflicting, and leak diagnostics. r[agent_harness_runtime.memory_conformance.failure_taxonomy]
- [x] [depends:matrix.coverage,null_backend.disabled,filesystem_backend.path_redaction,semantic_backend.unavailable,remote_backend.failures,failure_taxonomy] Update docs and run focused conformance tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_conformance.verification]
