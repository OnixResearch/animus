# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Trace-export DTO family over the admitted ledger

r[agent_harness_runtime.trace_export.wire] Animus MUST provide a versioned trace-export DTO family that projects validated provider-declared calls, assistant prose, reasoning, explicit directional agent messages, activations, and typed causal relationships from the admitted session ledger, with one canonical native JSONL projection and one TOON text projection.

#### Scenario: Projection preserves item kind and per-agent order

- GIVEN the admitted session ledger contains provider-declared calls, assistant prose, reasoning, directional messages, and activations for one agent
- WHEN an export projection runs for that agent
- THEN every exported item retains its authoritative journal sequence, qualified absolute append time, and valid item kind
- AND no provider payload, raw prompt, credential, or private path leaks into the projection.

### Requirement: Lite content bounds with exact metrics

r[agent_harness_runtime.trace_export.bounds] The trace-export projection MUST support a lite mode that bounds each semantic text or output item to a fixed non-zero byte ceiling while retaining exact complete-content metrics, and a full mode that retains complete content.

#### Scenario: Lite export bounds its items

- GIVEN a lite export contains a semantic text or output item longer than the fixed ceiling
- WHEN the projection emits that item
- THEN the emitted item content is bounded to the ceiling
- AND the export still retains the exact complete-content metrics for that item.

### Requirement: Journal sequence is the only in-agent causality authority

r[agent_harness_runtime.trace_export.sequence] Within one agent, the journal sequence MUST be the authoritative causality and ordering basis for exported items, while cross-agent wall-clock time MUST be treated as a readable overview only and never as causality.

#### Scenario: Cross-agent wall time is not causality

- GIVEN an export summarizes items from more than one agent
- WHEN a consumer inspects ordering between those agents
- THEN the export marks cross-agent time as an overview
- AND no cross-agent ordering claim is derived from wall-clock comparison.

### Requirement: Content-free performance accounting projection

r[agent_harness_runtime.trace_export.accounting] Animus MUST provide a content-free performance accounting projection that records provider-prompt accounting, fixed-point cache ratios, and explicit missing-evidence coverage markers, with one summary per included agent and no semantic text or output bodies.

#### Scenario: Performance projection emits one summary per agent

- GIVEN a performance accounting projection over included agents
- WHEN the projection runs
- THEN it emits exactly one summary per included agent
- AND the summary contains no assistant prose, reasoning text, tool output, or provider payload bodies.

### Requirement: Trace-export validation fails closed

r[agent_harness_runtime.trace_export.fail_closed] Trace-export validation MUST reject unknown item kinds, malformed records, non-monotonic journal sequences, unbounded items, duplicate summaries, and unsupported export shapes with bounded safe error classes before any projection is accepted.

#### Scenario: Non-monotonic sequence is rejected

- GIVEN an export projection contains an item whose journal sequence is lower than the previous item
- WHEN trace-export validation runs
- THEN validation rejects the projection with a sequence diagnostic
- AND no partially bounded export is promoted as valid.

### Requirement: Metadata-only trace receipts

r[agent_harness_runtime.trace_export.receipts] Trace-export receipts MUST include schema version, source ledger identity, item counts, content-mode label, coverage markers, and BLAKE3 projection identity, and MUST NOT include raw prompts, provider payloads, credentials, private paths, or full item bodies.

#### Scenario: Receipt rejects provider body

- GIVEN a trace receipt fixture includes a provider payload or raw prompt body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Trace-export verification evidence

r[agent_harness_runtime.trace_export.verification] The adaptation MUST include focused positive and negative tests for projection shape, lite bounds, sequence authority, content-free accounting, fail-closed validation, receipt leak rejection, and feature isolation.

#### Scenario: Closeout evidence covers trace fixtures

- GIVEN maintainers intend to archive the trace-export adaptation
- WHEN closeout validation runs
- THEN evidence names focused ledger, projection, bounds, accounting, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
