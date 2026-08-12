# agent-harness-runtime Change Spec: Memory adapter conformance rail

## Requirements

### Requirement: Memory conformance matrix

r[agent_harness_runtime.memory_conformance.matrix] Animus MUST maintain a memory adapter conformance matrix that records backend family, adapter role, declared capabilities, owner, applicability, positive case, negative case, and safe evidence label for null/no-op, local file or markdown, SQL or document, vector or hybrid search, MCP or remote, extractor, and admission adapter families.

#### Scenario: Matrix row without failure coverage is rejected

r[agent_harness_runtime.memory_conformance.matrix.coverage]
- GIVEN a memory conformance matrix row omits owner, capability declaration, positive case, negative case, safe evidence label, or not-applicable reason
- WHEN matrix validation runs
- THEN validation rejects the row with a typed coverage diagnostic
- AND the row cannot count as release evidence.

### Requirement: Null memory backend is first-class

r[agent_harness_runtime.memory_conformance.null_backend] A null or no-op memory backend MUST be a valid adapter family that reports disabled, unavailable, or no-influence outcomes without storing records, retrieving records, injecting memory, or generating new memory candidates.

#### Scenario: Disabled memory performs no side effects

r[agent_harness_runtime.memory_conformance.null_backend.disabled]
- GIVEN memory use and generation are disabled or backed by the null adapter
- WHEN runtime prepares a turn and closes the turn
- THEN no storage, retrieval, prompt injection, background extraction, update, or deletion operation is planned
- AND required guidance and policy remain active without memory state.

### Requirement: File-backed memory remains host-owned

r[agent_harness_runtime.memory_conformance.filesystem_backend] Local file or markdown memory adapters MUST remain host-owned shells that convert already-admitted file contents into normalized records; reusable core logic MUST NOT read directories, canonicalize paths, parse markdown files from disk, write memory files, or rely on a host path layout.

#### Scenario: File path is redacted from reusable evidence

r[agent_harness_runtime.memory_conformance.filesystem_backend.path_redaction]
- GIVEN a file-backed host adapter imports a memory record from a local markdown file
- WHEN reusable receipt or influence-ledger validation runs
- THEN evidence uses a safe source label and BLAKE3 body hash
- AND full paths and raw memory bodies are absent from reusable receipts and model-visible ledger metadata.

### Requirement: Semantic memory backends expose normalized results

r[agent_harness_runtime.memory_conformance.semantic_backend] SQL, document, vector, and hybrid search backends MUST expose normalized record envelopes plus safe backend-declared match reasons or scores; reusable planning MUST NOT depend on backend-specific query languages, vector dimensions, embedding provider clients, collection names, or index internals.

#### Scenario: Semantic backend unavailable has no ambient fallback

r[agent_harness_runtime.memory_conformance.semantic_backend.unavailable]
- GIVEN a vector or hybrid search adapter is configured but returns unavailable, timed-out, or unsupported
- WHEN reusable memory retrieval planning receives that status
- THEN the plan records a typed unavailable outcome
- AND no keyword search, provider call, network request, local index scan, or alternate backend is attempted unless an explicit alternate adapter was injected and admitted.

### Requirement: Remote memory backend receipts are scoped and redacted

r[agent_harness_runtime.memory_conformance.remote_backend] MCP or remote memory adapters MUST represent remote service access through scoped descriptors, typed request/response DTOs, timeout and denial statuses, redacted remote IDs, and safe receipts that exclude credentials, raw provider payloads, transport envelopes, and unbounded remote output.

#### Scenario: Remote memory failure is safe

r[agent_harness_runtime.memory_conformance.remote_backend.failures]
- GIVEN an MCP or remote memory adapter returns denied, timed-out, malformed, unavailable, or unsafe output
- WHEN reusable memory planning evaluates the response
- THEN the operation fails with the corresponding typed status
- AND no receipt claims successful storage, retrieval, update, deletion, or prompt influence.

### Requirement: Shared memory failure taxonomy

r[agent_harness_runtime.memory_conformance.failure_taxonomy] Memory adapter conformance MUST use a shared failure taxonomy that covers unavailable, denied, timed-out, malformed-record, stale, over-budget, redacted, conflicting, unsafe-receipt, and leak diagnostics.

#### Scenario: Failure taxonomy is stable across backends

r[agent_harness_runtime.memory_conformance.failure_taxonomy.stable]
- GIVEN two backend families encounter equivalent unavailable, denied, malformed, stale, over-budget, conflict, or leak conditions
- WHEN conformance cases map their results
- THEN reusable diagnostics use the same failure taxonomy
- AND backend-specific details remain safe metadata rather than model-visible raw output.

### Requirement: Memory conformance verification

r[agent_harness_runtime.memory_conformance.verification] The memory conformance rail MUST include focused positive and negative fixtures for matrix validation, null memory behavior, file-backed boundaries, semantic backend normalization, remote backend failures, failure taxonomy stability, no ambient fallback, and receipt leak rejection.

#### Scenario: Closeout evidence covers memory conformance fixtures

r[agent_harness_runtime.memory_conformance.verification.closeout]
- GIVEN maintainers intend to archive the memory adapter conformance rail
- WHEN closeout validation runs
- THEN evidence names focused matrix, null-backend, filesystem-boundary, semantic-backend, remote-backend, failure-taxonomy, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
