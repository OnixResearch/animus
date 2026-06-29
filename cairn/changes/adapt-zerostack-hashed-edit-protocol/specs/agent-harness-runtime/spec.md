# agent-harness-runtime Change Spec: Zerostack hashed edit protocol adaptation

## Requirements

### Requirement: BLAKE3 snapshot identity

r[agent_harness_runtime.zerostack_hashed_edit.snapshot] Animus hashed edit snapshots MUST identify the edited resource with a full-content BLAKE3 snapshot hash, text normalization mode, line count, and line tag generation metadata before edits can be validated.

#### Scenario: Stale snapshot is rejected

r[agent_harness_runtime.zerostack_hashed_edit.snapshot.stale]
- GIVEN an edit request references an earlier snapshot hash
- AND the host reports a different current snapshot hash for the resource
- WHEN edit validation runs
- THEN validation fails with a stale-snapshot diagnostic
- AND no write or partial mutation is performed.

### Requirement: Tagged span edit requests

r[agent_harness_runtime.zerostack_hashed_edit.tagged_spans] Hashed edit requests MUST reference validated tagged lines or contiguous tagged spans plus bounded replacement text rather than relying on fuzzy old-text matching.

#### Scenario: Unknown tag cannot select text

r[agent_harness_runtime.zerostack_hashed_edit.tagged_spans.unknown]
- GIVEN an edit request contains a tag not present in the supplied snapshot
- WHEN edit-plan validation runs
- THEN validation fails with an unknown-tag diagnostic
- AND the request cannot select or mutate any text.

### Requirement: Pure edit-plan validation

r[agent_harness_runtime.zerostack_hashed_edit.plan_validation] Edit-plan validation MUST be pure deterministic logic over in-memory snapshots and requests, detecting invalid spans, duplicate span selectors, overlapping edits, out-of-order edits, unsupported newline policy, and overlong replacement text.

#### Scenario: Overlapping edits fail closed

r[agent_harness_runtime.zerostack_hashed_edit.plan_validation.overlap]
- GIVEN two requested edits select overlapping tagged spans
- WHEN edit-plan validation runs
- THEN validation fails with an overlap diagnostic
- AND the host receives no executable edit plan.

### Requirement: Host-owned file mutation

r[agent_harness_runtime.zerostack_hashed_edit.host_mutation] Filesystem reads, writes, locks, atomic replacement, permission checks, and path resolution MUST remain host/tool-adapter responsibilities behind injected tool contracts.

#### Scenario: Core validation performs no ambient I/O

r[agent_harness_runtime.zerostack_hashed_edit.host_mutation.no_io]
- GIVEN edit-plan validation receives a snapshot and edit request in memory
- WHEN validation runs
- THEN it does not read files, inspect environment variables, open locks, spawn processes, or write output files
- AND mutation remains unavailable until a host adapter applies the validated plan.

### Requirement: Safe edit receipts

r[agent_harness_runtime.zerostack_hashed_edit.receipts] Hashed edit receipts MUST record resource class, redacted path or BLAKE3 resource hash, before and after snapshot hashes, selected span metadata, replacement size metadata, and diagnostics without embedding raw private file bodies by default.

#### Scenario: Receipt omits full private path

r[agent_harness_runtime.zerostack_hashed_edit.receipts.redacted_path]
- GIVEN an edit targets a private host path
- WHEN an edit receipt is emitted
- THEN the path is omitted, relativized, or represented by a BLAKE3 resource hash according to policy
- AND before/after snapshot hashes remain available for audit.
