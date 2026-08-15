## Phase 1: Trace-export contract and pure core

- [ ] [serial] Add versioned trace-export DTOs for provider-declared calls, assistant prose, reasoning, directional messages, activations, and typed causal relationships. r[agent_harness_runtime.trace_export.wire]
- [ ] [serial] Add canonical native JSONL and TOON text projections that retain per-item authoritative journal sequence and qualified absolute append time. r[agent_harness_runtime.trace_export.wire] r[agent_harness_runtime.trace_export.sequence]
- [ ] [serial] Add lite and full content modes with a fixed non-zero lite ceiling and exact complete-content metrics. r[agent_harness_runtime.trace_export.bounds]
- [ ] [serial] Add the content-free performance accounting projection with fixed-point cache ratios and missing-evidence coverage markers, one summary per included agent. r[agent_harness_runtime.trace_export.accounting]

## Phase 2: Fail-closed validation and receipts

- [ ] [serial] Add fail-closed validation for unknown kinds, malformed records, non-monotonic sequences, unbounded items, duplicate summaries, and unsupported export shapes. r[agent_harness_runtime.trace_export.fail_closed]
- [ ] [depends:contract] Add metadata-only receipts with schema version, source ledger identity, counts, content-mode label, and BLAKE3 projection identity, rejecting raw prompts, provider payloads, credentials, and private paths. r[agent_harness_runtime.trace_export.receipts]

## Phase 3: Evidence

- [ ] [parallel] Add positive fixtures for projection shape, lite and full modes, sequence ordering, accounting summaries, and receipt identity. r[agent_harness_runtime.trace_export.verification]
- [ ] [parallel] Add negative fixtures for malformed records, non-monotonic sequences, unbounded items, duplicate summaries, unsupported shapes, and receipt leak rejection. r[agent_harness_runtime.trace_export.fail_closed] r[agent_harness_runtime.trace_export.verification]
- [ ] [serial] Verify default builds remain independent of the export family and no live provider or daemon dependency enters reusable crates. r[agent_harness_runtime.trace_export.verification]

## Phase 4: Documentation and closeout

- [ ] [parallel] Document the export surface, content modes, sequence authority, cross-agent non-causality, accounting report shape, and non-claims. r[agent_harness_runtime.trace_export.wire] r[agent_harness_runtime.trace_export.sequence]
- [ ] [serial] Run focused ledger, adapter, workspace, formatting, clippy, component-catalog, Cairn validation, and Cairn gate rails, then sync and archive only after all deterministic gates pass. r[agent_harness_runtime.trace_export.verification]
