# Tasks

- [x] [serial] Define hashed edit snapshot DTOs with BLAKE3 full-content hash, line count, normalization mode, and tag generation metadata. r[agent_harness_runtime.zerostack_hashed_edit.snapshot]
- [x] [depends:snapshot] Add stale snapshot positive/negative tests using in-memory current snapshot hashes. r[agent_harness_runtime.zerostack_hashed_edit.snapshot.stale]
- [x] [serial] Define tagged line and contiguous span selectors with bounded replacement text. r[agent_harness_runtime.zerostack_hashed_edit.tagged_spans]
- [x] [depends:tagged_spans] Add negative tests for unknown, malformed, duplicate, and ambiguous tag selectors. r[agent_harness_runtime.zerostack_hashed_edit.tagged_spans.unknown]
- [x] [serial] Implement pure edit-plan validation for span order, overlap, newline policy, and replacement size. r[agent_harness_runtime.zerostack_hashed_edit.plan_validation]
- [x] [depends:plan_validation] Add overlapping and out-of-order edit rejection tests. r[agent_harness_runtime.zerostack_hashed_edit.plan_validation.overlap]
- [x] [serial] Define host adapter contract for applying validated edit plans without moving filesystem authority into reusable core. r[agent_harness_runtime.zerostack_hashed_edit.host_mutation]
- [x] [depends:host_mutation] Add tests or static checks proving pure validation performs no ambient filesystem, process, clock, or environment I/O. r[agent_harness_runtime.zerostack_hashed_edit.host_mutation.no_io]
- [x] [depends:snapshot,tagged_spans,plan_validation,host_mutation] Add safe edit receipts and redaction tests for full private paths and raw file bodies. r[agent_harness_runtime.zerostack_hashed_edit.receipts]
- [x] [depends:receipts] Update docs and run focused edit protocol tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_hashed_edit.receipts.redacted_path]
