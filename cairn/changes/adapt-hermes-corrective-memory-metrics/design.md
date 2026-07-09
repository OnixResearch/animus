# Design: Adapt Hermes corrective memory and review-metrics contracts

## Context

Hermes' napkin and review-metrics workflow is valuable because it turns recurring review findings into durable prevention candidates. Animus already has memory controls, review reports, and Qualifier-style quality record envelopes. This change connects those surfaces with explicit corrective-memory contracts.

## Decisions

### 1. Corrective memory is not always active

**Choice:** Corrective records are host-provided descriptors with trust, scope, source, freshness, tags, and body identity. Prompt influence requires explicit admission.

**Rationale:** Animus is a framework/SDK. It must not require a specific per-repo file, always-read policy, or hidden memory layer.

### 2. Metric records stay safe and append-only by request

**Choice:** Parsing and aggregation remain pure and read-only. Durable append/promote operations are explicit host action requests with safe receipts.

**Rationale:** Metrics can be operational data. Reusable crates should validate records and summarize supplied data, not choose storage or write files.

### 3. Promotion is advisory until admitted

**Choice:** Repeated-finding grouping produces promotion candidates with target classes such as deterministic check, policy profile, spec rule, memory guidance, or human checkpoint. It never directly edits tasks, specs, policy, or memory stores.

**Rationale:** Promotion changes governance and must be owned by host/product policy.

### 4. Corrective records compose with memory/quality contracts

**Choice:** Reuse safe envelope, lifecycle, freshness, and memory-control semantics where possible, while keeping this loop as an optional adapter family.

**Rationale:** This avoids a parallel memory system and keeps records portable across hosts.

## Risks / Trade-offs

- Advisory promotions may be ignored. Receipts should preserve candidate counts and skipped reasons so hosts can surface backlog health.
- Excessive corrective records can bloat prompts. Admission must budget and summarize.
- User preferences and self-corrections have different authority. Trust/scope fields must keep them distinct.
