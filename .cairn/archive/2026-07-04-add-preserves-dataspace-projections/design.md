# Design: Add Preserves dataspace projections

## Context

`animus-adapters` owns the implementation-free dataspace DTOs and fake backend. `animus-preserves` owns portable text/binary boundary encoding. The existing Preserves catalog covers adapter messages, runtime events, reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows, but not the dataspace assertion/subscription/orchestration shapes that a future live coordination layer will need.

## Decisions

### 1. Dataspace Preserves coverage lives in `animus-preserves`

**Choice:** Add dataspace schema identities and conversion helpers in `animus-preserves`, not in `animus-adapters`.

**Rationale:** `animus-adapters` must remain implementation-free and `#![no_std]`; Preserves is an optional bridge crate. Keeping encoding there preserves dependency isolation and lets hosts opt into the wire format only when needed.

### 2. Records decode through validated typed DTO constructors

**Choice:** Preserves decoding produces `animus_adapters::dataspace` DTOs through existing constructors and validation helpers before behavior decisions run.

**Rationale:** Raw Preserves syntax trees should never become policy or runtime state. The typed DTO boundary is where scope, redaction, unsupported value, and malformed subscription checks already belong.

### 3. Safe payloads are hash-and-label oriented

**Choice:** Dataspace Preserves payloads carry only stable identifiers, normalized labels, status classes, assertion kinds, redaction markers, counts, BLAKE3 hashes, lease/expiry fields, and ownership-transfer labels.

**Rationale:** Dataspaces invite fan-out. The serialization layer must make it hard to accidentally distribute prompts, provider payloads, credentials, full paths, file bodies, tool input/output, model output, or unbounded logs.

### 4. Dataspace coverage starts as stable external wire contract for DTOs, not live transport

**Choice:** Treat the versioned dataspace DTO records as stable external wire contracts once fixtures pass. Do not claim live Syndicate transport compatibility in this change.

**Rationale:** Stable DTO encoding is the prerequisite for cross-process and cross-language coordination. Live transport brings separate host configuration, daemon/network, credentials, and smoke-test concerns.

## Risks / Trade-offs

- Adding dataspace families expands the Preserves catalog. Keep variants small and fixture-driven.
- The dataspace DTOs are richer than current adapter request/response records. Prefer conversion helpers with explicit validation over generic maps.
- Unknown future fields are useful for compatibility but risky for broadcast surfaces. Preserve only bounded safe metadata and fail closed on unsafe names or values.
