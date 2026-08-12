# Tasks

- [x] [serial] Define context-reference DTOs for raw label, normalized target, kind, range, render mode, and source trust metadata. r[agent_harness_runtime.clankers_context_references.typed_inputs]
- [x] [depends:typed_inputs] Add parser tests for local files, line ranges, directories, diffs, images, URLs, malformed ranges, and unsupported reference kinds. r[agent_harness_runtime.clankers_context_references.typed_inputs.file]
- [x] [serial] Define host-injected resolver service traits and missing/denied/unavailable resolver failure classes. r[agent_harness_runtime.clankers_context_references.resolvers]
- [x] [depends:resolvers] Add negative URL-denied and missing-resolver tests proving no filesystem, VCS, image, or network fallback occurs. r[agent_harness_runtime.clankers_context_references.resolvers.url_denied]
- [x] [serial] Implement deterministic context-reference budget decisions for include, truncate, and reject outcomes. r[agent_harness_runtime.clankers_context_references.bounds]
- [x] [depends:bounds] Add oversized-reference tests with byte/line counts, truncation metadata, and BLAKE3 content identity. r[agent_harness_runtime.clankers_context_references.bounds.oversized]
- [x] [serial] Define safe provenance receipts and ledger metadata for context-reference outcomes. r[agent_harness_runtime.clankers_context_references.provenance]
- [x] [depends:provenance] Add leak fixtures for raw content, URL credentials, absolute path dumps, secret-looking values, and unbounded output. r[agent_harness_runtime.clankers_context_references.provenance.rejects_leaks]
- [x] [depends:typed_inputs.file,resolvers.url_denied,bounds.oversized,provenance.rejects_leaks] Update docs and run focused context-reference tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_context_references.verification]
