# agent-harness-runtime Change Spec: Subagent lineage ledger

## Requirements

### Requirement: Ledger records parent/child subagent lineage

r[agent_harness_runtime.subagent_lineage_ledger.record] Animus MUST record subagent lineage using safe parent id, child id, child-run kind, delegation source, admission identity, effective capability identity, context packet identity, output hash, handoff mode, terminal status, and failure class fields.

#### Scenario: Completed child run has lineage record

- GIVEN a child run completes and contributes a summary to the parent
- WHEN the lineage record is built
- THEN the record links the child id to its parent id, kind, delegation source, capability identity, context packet identity, output hash, and handoff mode
- AND the record contains no raw child transcript.

### Requirement: Lifecycle events project into lineage records

r[agent_harness_runtime.subagent_lineage_ledger.lifecycle_projection] Coordinator lifecycle events SHOULD project into validated lineage records that preserve safe status, failure, budget, evidence, and receipt references while rejecting raw transcripts, provider payloads, credentials, and unbounded tool output.

#### Scenario: Failed child run produces safe lineage

- GIVEN a child run times out before returning a valid summary
- WHEN lifecycle projection runs
- THEN the lineage record stores the timeout failure class and safe evidence reference
- AND no raw partial output is copied into the ledger record.

### Requirement: Replay validates child influence

r[agent_harness_runtime.subagent_lineage_ledger.replay] Session replay MUST be able to validate parent-visible child influence against lineage records, accepted handoff mode, context packet identity, output hash, and safe receipt identity without requiring raw child transcript storage.

#### Scenario: Parent imports child summary without lineage

- GIVEN a parent session contains a child summary import
- WHEN replay cannot find a matching lineage record and output hash
- THEN replay marks the import as invalid or unaudited
- AND the child summary is not treated as verified influence.

### Requirement: Lineage validation fails closed

r[agent_harness_runtime.subagent_lineage_ledger.fail_closed] Subagent lineage validation MUST reject missing lineage for imported child output, mismatched parent or child ids, invalid BLAKE3 hashes, unsupported handoff modes, malformed receipts, raw transcript markers, unsafe summaries, and unbounded evidence references.

#### Scenario: Raw transcript marker is rejected

- GIVEN a lineage record attempts to include raw child log text or an unsafe raw transcript marker
- WHEN lineage validation runs
- THEN validation returns a leak diagnostic
- AND the record is not accepted for replay or durable evidence.

### Requirement: Lineage closeout is evidenced

r[agent_harness_runtime.subagent_lineage_ledger.verification] Subagent lineage adoption MUST include positive lineage/replay tests, negative fail-closed leak and mismatch tests, receipt tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Lineage evidence is complete

- GIVEN the subagent lineage change is ready to archive
- WHEN closeout validation runs
- THEN evidence names lineage validation tests, replay tests, receipt tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
