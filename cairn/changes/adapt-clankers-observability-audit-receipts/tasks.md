# Tasks

- [ ] [serial] Define audit snapshot DTOs with schema version, bounded counts, status classes, timing buckets, over-limit flags, event window identity, and error classes. r[agent_harness_runtime.clankers_audit_receipts.snapshots]
- [ ] [depends:snapshots] Add empty-window audit snapshot tests with zero model/tool/confirmation/background/delivery counts. r[agent_harness_runtime.clankers_audit_receipts.snapshots.empty]
- [ ] [serial] Implement pending-state clamping for model calls, tool calls, confirmations, background work, delivery attempts, and retries. r[agent_harness_runtime.clankers_audit_receipts.clamping]
- [ ] [depends:clamping] Add over-limit pending-tool tests proving raw call IDs, arguments, outputs, and private metadata are omitted. r[agent_harness_runtime.clankers_audit_receipts.clamping.pending_tools]
- [ ] [serial] Enforce host-owned boundaries for metrics sinks, telemetry exporters, clocks, persistent storage, and UI rendering. r[agent_harness_runtime.clankers_audit_receipts.host_owned]
- [ ] [depends:host_owned] Add sink-unavailable tests proving projection remains pure and unchanged. r[agent_harness_runtime.clankers_audit_receipts.host_owned.sink_unavailable]
- [ ] [serial] Compose audit summaries over stable safe status/failure categories from runtime events, tools, confirmations, background work, delivery, and receipts. r[agent_harness_runtime.clankers_audit_receipts.composition]
- [ ] [depends:composition] Add completed failure-class summary tests for invalid input, denial, timeout, cancellation, malformed output, and unavailable service. r[agent_harness_runtime.clankers_audit_receipts.composition.failures]
- [ ] [serial] Define safe audit receipts with policy/window/count/status/timing/over-limit/failure metadata and BLAKE3 receipt hash. r[agent_harness_runtime.clankers_audit_receipts.receipts]
- [ ] [depends:receipts] Add leak fixtures for raw prompts, tool args/output, provider JSON, credentials, headers, env values, full paths, and unbounded logs. r[agent_harness_runtime.clankers_audit_receipts.receipts.rejects_leaks]
- [ ] [depends:snapshots.empty,clamping.pending_tools,host_owned.sink_unavailable,composition.failures,receipts.rejects_leaks] Update docs and run focused audit receipt tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_audit_receipts.verification]
