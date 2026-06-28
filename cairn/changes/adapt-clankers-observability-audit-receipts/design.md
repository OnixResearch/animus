# Design: Adapt Clankers observability audit receipts for Animus

## Context

Safe receipts capture individual runtime decisions. Audit receipts summarize runtime health over a bounded event window: how many tool/model calls are pending, completed, failed, or over limits. Clankers' audit pattern is intentionally conservative, exposing counts and booleans rather than raw maps of sensitive state. Animus should adopt that as a reusable receipt brick.

## Decisions

### 1. Audit snapshots are bounded projections

**Choice:** An audit snapshot projects internal runtime state into schema-versioned counts, status classes, timing buckets, over-limit flags, and error classes.

**Rationale:** Operators need observability, but public receipts must not enumerate private call IDs, prompts, outputs, or arguments.

### 2. Pending state is clamped

**Choice:** When pending model/tool/confirmation/background/delivery state exceeds configured public limits, the receipt reports the limit, a clamped count, and an over-limit diagnostic.

**Rationale:** Large pending maps can be sensitive and unbounded. Clamping preserves evidence without leaks.

### 3. Hosts own sinks and clocks

**Choice:** Reusable audit logic accepts an already-defined event window and timing summaries. Hosts own clocks, metrics sinks, telemetry exporters, storage, and UI rendering.

**Rationale:** Audit projection should stay pure and deterministic in tests.

### 4. Receipts compose with runtime events

**Choice:** Audit snapshots reference event window identity, policy hash, and safe status categories from runtime event lifecycle, tool dispatch, confirmation, background work, and delivery contracts.

**Rationale:** Audit receipts should be an index over safe receipts/events, not a separate hidden telemetry schema.

### 5. Sensitive fields are forbidden

**Choice:** Audit receipts reject raw prompts, tool names/arguments/outputs unless explicitly safe labels, provider payloads, credentials, authorization headers, OAuth tokens, environment values, full paths, and unbounded logs.

**Rationale:** Observability must be safe by construction.

## Risks / Trade-offs

- Redacted audit receipts can feel too sparse. Include stable counts, booleans, buckets, and receipt hashes for correlation.
- Without clocks in core, duration data depends on host summaries. Validate bucket shapes rather than requiring wall-clock reads.
- Audit windows can overlap. Use opaque window IDs and policy hashes for traceability.
