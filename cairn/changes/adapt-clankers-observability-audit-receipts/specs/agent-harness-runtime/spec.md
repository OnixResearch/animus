# agent-harness-runtime Change Spec: Clankers observability audit receipt adaptation

## Requirements

### Requirement: Bounded audit snapshots

r[agent_harness_runtime.clankers_audit_receipts.snapshots] Animus audit snapshots MUST project runtime state into schema-versioned bounded counts, status classes, timing buckets, over-limit flags, event window identity, and error classes without exposing private pending maps.

#### Scenario: Empty audit snapshot is valid

r[agent_harness_runtime.clankers_audit_receipts.snapshots.empty]
- GIVEN no model calls, tool calls, confirmations, background work, or delivery attempts are pending or completed in the window
- WHEN audit projection runs
- THEN it returns a valid empty snapshot with zero counts and safe schema metadata.

### Requirement: Pending state clamping

r[agent_harness_runtime.clankers_audit_receipts.clamping] Audit projection MUST clamp public pending counts to configured limits and record over-limit diagnostics instead of serializing full pending state.

#### Scenario: Pending tool calls exceed public limit

r[agent_harness_runtime.clankers_audit_receipts.clamping.pending_tools]
- GIVEN internal pending tool-call state exceeds the configured public pending limit
- WHEN audit projection runs
- THEN the receipt reports the configured limit, clamped public count, and over-limit status
- AND it does not serialize raw pending call IDs, arguments, outputs, or private metadata.

### Requirement: Host-owned audit sinks

r[agent_harness_runtime.clankers_audit_receipts.host_owned] Metrics sinks, telemetry exporters, clocks, persistent storage, and UI rendering MUST remain host-owned behavior outside reusable audit projection logic.

#### Scenario: Sink unavailable does not alter projection

r[agent_harness_runtime.clankers_audit_receipts.host_owned.sink_unavailable]
- GIVEN audit projection produces a valid snapshot
- AND a host telemetry sink is unavailable
- WHEN the host attempts export
- THEN the reusable projection remains unchanged
- AND sink failure is recorded as a host adapter outcome rather than a projection mutation.

### Requirement: Audit receipt composition

r[agent_harness_runtime.clankers_audit_receipts.composition] Audit receipts MUST reference safe runtime event, tool dispatch, confirmation, background work, delivery, and receipt status categories rather than raw source payloads.

#### Scenario: Completed failure classes summarize safely

r[agent_harness_runtime.clankers_audit_receipts.composition.failures]
- GIVEN completed work includes invalid input, denied capability, timeout, cancellation, malformed output, and unavailable service failures
- WHEN audit projection runs
- THEN the receipt summarizes counts by stable failure class
- AND no raw tool arguments, provider payloads, prompts, or output bodies are included.

### Requirement: Safe audit receipts

r[agent_harness_runtime.clankers_audit_receipts.receipts] Audit receipts MUST record safe metadata such as schema version, policy hash, event window identity, bounded counts, status classes, timing buckets, over-limit diagnostics, failure class counts, and BLAKE3 receipt hash, and MUST NOT include raw prompts, tool arguments, tool outputs, provider payloads, credentials, authorization headers, environment values, full paths, or unbounded logs.

#### Scenario: Audit receipt leak rejected

r[agent_harness_runtime.clankers_audit_receipts.receipts.rejects_leaks]
- GIVEN an audit receipt fixture contains raw prompt text, tool arguments, tool output, provider JSON, credentials, headers, environment values, full paths, or unbounded logs
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Audit receipt verification

r[agent_harness_runtime.clankers_audit_receipts.verification] The adaptation MUST include focused positive and negative tests for empty snapshots, pending-state clamping, completed summaries, host-owned sink boundaries, failure class composition, over-limit diagnostics, and receipt redaction.

#### Scenario: Closeout evidence covers audit fixtures

r[agent_harness_runtime.clankers_audit_receipts.verification.closeout]
- GIVEN maintainers intend to archive the observability audit receipt adaptation
- WHEN closeout validation runs
- THEN evidence names focused snapshot, clamping, host-boundary, composition, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
