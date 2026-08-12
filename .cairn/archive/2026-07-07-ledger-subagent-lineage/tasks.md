# Tasks

- [x] [serial] Add child-run neutral subagent lineage ledger DTOs and receipt validation helpers. r[agent_harness_runtime.subagent_lineage_ledger.record]
- [x] [depends:record] Project coordinator lifecycle events into safe lineage records without raw child transcripts. r[agent_harness_runtime.subagent_lineage_ledger.lifecycle_projection]
- [x] [depends:record] Add replay validation for parent-visible child influence, handoff modes, context packet identities, and output hashes. r[agent_harness_runtime.subagent_lineage_ledger.replay]
- [x] [depends:record,replay] Add negative tests for missing lineage, mismatched parent/child ids, invalid hashes, raw transcript leakage, malformed receipts, and unsafe summaries. r[agent_harness_runtime.subagent_lineage_ledger.fail_closed]
- [x] [depends:lifecycle_projection,fail_closed] Update docs and focused lineage ledger verification evidence. r[agent_harness_runtime.subagent_lineage_ledger.verification]
