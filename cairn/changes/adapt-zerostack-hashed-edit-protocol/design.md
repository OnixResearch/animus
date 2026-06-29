# Design: Adapt Zerostack hashed edit protocol for Animus

## Context

Zerostack's hashed edit mode trades verbose search/replace blocks for tagged read output and file-level CAS. Animus should adopt the contract shape but use BLAKE3 for content identity and keep the host boundary explicit.

## Decisions

### 1. Snapshot identity uses BLAKE3

**Choice:** Represent a readable text snapshot with a file/resource identity, full-content BLAKE3 hash, normalization mode, and line-tag generation parameters.

**Rationale:** BLAKE3 is the Animus default for content hashing and provides deterministic stale-file detection.

### 2. Line tags are opaque capabilities

**Choice:** Edit requests reference line tags or contiguous tagged spans, not raw old text.

**Rationale:** This reduces token usage and avoids fuzzy matching while still proving the model saw the intended snapshot.

### 3. Plan validation is pure

**Choice:** Validate tag existence, span order, overlap, replacement text limits, newline policy, and expected snapshot hash in deterministic core logic over in-memory snapshots.

**Rationale:** The functional core can be tested without filesystem access, locks, editors, or mocks.

### 4. Host adapter owns mutation

**Choice:** The tool host reads files, produces snapshots, applies validated plans atomically where possible, and records receipts.

**Rationale:** Reusable contracts should not gain filesystem authority or OS-specific locking behavior.

## Risks

- Hash tags in prompts can be copied incorrectly. Validation must reject unknown or malformed tags with actionable diagnostics.
- Line-based tags can be ambiguous when lines repeat. Include line ordinal or span context in tag identity rather than raw line hash alone.
