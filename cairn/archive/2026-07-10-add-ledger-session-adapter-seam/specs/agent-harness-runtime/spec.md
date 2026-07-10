# agent-harness-runtime Change Spec: Add a ledger and session adapter seam

## Requirements

### Requirement: Ledger and session service seam is host-injected

r[agent_harness_runtime.ledger_session_adapter.service] Animus MUST define host-injected ledger and session service contracts, store descriptors, session identities, branch metadata, replay requests, restore requests, mutation records, and safe receipt drafts without requiring or discovering a default global session store.

#### Scenario: Missing session service fails closed

r[agent_harness_runtime.ledger_session_adapter.service.missing]
- GIVEN runtime or host-shell code requests session replay, restore, append, branch, or mutation recording
- WHEN no ledger/session service has been injected
- THEN Animus returns a typed unavailable-service outcome
- AND it does not scan global directories, open databases, read files, create storage, contact networks, or resume conversations by ambient fallback.

### Requirement: Replay and restore planning are pure

r[agent_harness_runtime.ledger_session_adapter.replay] Ledger replay and session restore planning MUST be pure over already-loaded ledger entries, descriptors, branch metadata, policies, budgets, freshness inputs, and redaction rules, producing bounded context projections, omitted-entry reasons, and receipt drafts without filesystem, database, network, clock, process, credential, terminal, or persistence effects.

#### Scenario: Bounded replay plan omits unsafe entries

r[agent_harness_runtime.ledger_session_adapter.replay.bounded]
- GIVEN loaded ledger entries include safe messages, tool summaries, omitted raw outputs, stale branch records, and private host references
- WHEN replay planning runs with explicit budget and redaction policy
- THEN the plan includes only bounded safe context projections
- AND records deterministic omitted-entry reasons for stale, over-budget, private, or unsafe entries.

### Requirement: Ledger/session receipts are safe

r[agent_harness_runtime.ledger_session_adapter.receipts] Ledger and session receipts MUST include only bounded safe labels, session and branch identifiers, sequence ranges, counts, redaction status, freshness class, safe hashes, and explicit non-claims, and MUST reject raw transcripts, raw tool output, credentials, provider payloads, private paths, unbounded terminal snapshots, and unbounded model-visible context.

#### Scenario: Unsafe restore receipt is rejected

r[agent_harness_runtime.ledger_session_adapter.receipts.rejects_leaks]
- GIVEN a restore receipt attempts to embed a raw transcript, credential, provider payload, private path, unbounded terminal snapshot, or raw tool output
- WHEN receipt validation runs
- THEN validation rejects the receipt with a deterministic leak diagnostic
- AND no restore-success evidence is emitted from that unsafe receipt.

### Requirement: Ledger/session seam verification

r[agent_harness_runtime.ledger_session_adapter.verification] The ledger/session adapter seam MUST include focused positive and negative tests for injected service availability, missing services, replay planning, restore planning, stale snapshots, conflicting branches, malformed sequence ranges, ambient fallback rejection, safe receipts, and leak rejection.

#### Scenario: Closeout evidence covers persistence non-claims

r[agent_harness_runtime.ledger_session_adapter.verification.closeout]
- GIVEN maintainers intend to archive the ledger/session adapter seam
- WHEN closeout validation runs
- THEN evidence names replay tests, restore tests, missing-service tests, receipt leak checks, catalog checks, request-lifecycle checks, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
