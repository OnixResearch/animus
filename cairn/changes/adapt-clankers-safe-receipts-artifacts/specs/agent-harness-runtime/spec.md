# agent-harness-runtime Change Spec: Clankers safe receipts and artifacts

## Requirements

### Requirement: Safe receipt DTOs

r[agent_harness_runtime.clankers_safe_receipts.dto] Animus runtime and provenance receipts MUST expose safe metadata only, including schema, status, issue code, labels, bounded counts, BLAKE3 hashes, effect or mutation kind, backend/interchange labels, and explicit redaction markers.

#### Scenario: Positive receipts expose only safe evidence

r[agent_harness_runtime.clankers_safe_receipts.positive]
- GIVEN model, tool, authorization, runtime block, final response, adapter error, or provenance observations contain raw sensitive inputs and outputs
- WHEN safe receipts are constructed
- THEN receipts expose only safe labels, bounded counts, status, issue class, redaction markers, and BLAKE3 hashes
- AND the raw sensitive material is omitted.

### Requirement: Receipt leak rejection

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks] Receipt validation MUST reject or redact raw prompt text, provider JSON, credential-looking values, absolute path dumps, raw tool input, raw tool output, raw model output, and unbounded output before evidence is accepted.

#### Scenario: Leak fixtures fail closed

r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks.fixture]
- GIVEN a receipt fixture includes raw prompt text, provider JSON, token-like values, absolute paths, raw tool/model output, or an oversized body
- WHEN receipt validation runs
- THEN validation returns a typed redaction error or redacted output
- AND the unsafe fixture is not accepted as review or release evidence.

### Requirement: BLAKE3 canonical artifact envelopes

r[agent_harness_runtime.clankers_artifact_identity.envelope] Animus-owned artifact identity MUST use BLAKE3 over versioned canonical envelopes containing artifact kind, envelope version, redaction class, normalized payload, and sorted/deduplicated dependency hashes.

#### Scenario: Artifact identity is deterministic

r[agent_harness_runtime.clankers_artifact_identity.deterministic]
- GIVEN equivalent artifact envelopes with dependencies provided in different orders and with duplicates
- WHEN artifact identity is computed
- THEN dependencies are sorted and deduplicated before hashing
- AND the resulting BLAKE3 hash is stable and round-trips through display/parse validation.

### Requirement: Redaction classes govern review output

r[agent_harness_runtime.clankers_artifact_identity.redaction] Artifact redaction classes MUST determine whether payload bodies may appear in receipts, CLI, or review output.

#### Scenario: Non-public artifacts hide payloads

r[agent_harness_runtime.clankers_artifact_identity.redaction.display]
- GIVEN an artifact is metadata-only, redacted-payload, or secret
- WHEN a receipt or review output is rendered
- THEN payload bodies are omitted or replaced with explicit redaction markers
- AND only public artifacts may expose payload bodies by default.

### Requirement: Focused receipt and artifact rails

r[agent_harness_runtime.clankers_safe_receipts.rails] Animus MUST provide focused verification rails for safe receipt validation, leak rejection, artifact hash determinism, and redaction-class display behavior before claiming the Clankers receipt/artifact pattern adopted.

#### Scenario: Receipt and artifact closeout is evidenced

r[agent_harness_runtime.clankers_safe_receipts.closeout]
- GIVEN the safe receipt and artifact identity change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused receipt/artifact rails, positive and negative tests, dependency checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
