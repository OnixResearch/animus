# Tasks

- [ ] [serial] Define context file descriptor DTOs with class, source identity, trust scope, precedence, freshness, size, redaction class, and inclusion policy. r[agent_harness_runtime.zerostack_context_layering.classes]
- [ ] [depends:classes] Add negative unknown-class and missing-source tests. r[agent_harness_runtime.zerostack_context_layering.classes.unknown]
- [ ] [serial] Define host-owned discovery plan and reusable in-memory descriptor boundary. r[agent_harness_runtime.zerostack_context_layering.discovery]
- [ ] [depends:discovery] Add tests or static checks proving prompt assembly performs no ancestor walks, file reads, environment reads, or ambient path canonicalization. r[agent_harness_runtime.zerostack_context_layering.discovery.no_io]
- [ ] [serial] Define admission policy for trust, size, freshness, redaction, and prompt inclusion. r[agent_harness_runtime.zerostack_context_layering.admission]
- [ ] [depends:admission] Add secret-looking body, oversized body, stale file, disabled discovery, and untrusted source negative tests. r[agent_harness_runtime.zerostack_context_layering.admission.secret]
- [ ] [serial] Define deterministic precedence and conflict diagnostics for overlapping admitted context descriptors. r[agent_harness_runtime.zerostack_context_layering.precedence]
- [ ] [depends:precedence] Add conflict tests proving deterministic winner selection and omitted-descriptor diagnostics. r[agent_harness_runtime.zerostack_context_layering.precedence.conflict]
- [ ] [depends:classes,discovery,admission,precedence] Add context layering receipts with BLAKE3 body hashes and redacted source identities. r[agent_harness_runtime.zerostack_context_layering.receipts]
- [ ] [depends:receipts] Update docs and run focused context/prompt assembly tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_context_layering.receipts.omitted]
