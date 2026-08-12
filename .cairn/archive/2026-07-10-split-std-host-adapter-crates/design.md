# Design: Split std host adapters into authority-scoped crates

## Context

The repo already distinguishes reusable crates from the std shell. The next Lego step is to split the std shell itself by authority so embedders can choose local context, provider metadata, observability sinks, document ingest, model-evidence ingest, or demo smoke behavior independently.

## Decisions

### 1. Authority labels drive the split

**Choice:** Split host adapters by required authority rather than by current file layout. Each block declares whether it may read environment pairs, read files, execute processes, open network transports, inspect credentials, persist data, or access terminal state.

**Rationale:** Host effects are the main risk boundary. Authority labels make it clear which block grants which shell capability.

### 2. `animus-std` remains the facade

**Choice:** Keep `animus-std` public exports stable and re-export moved host adapters during migration.

**Rationale:** Existing users should not break while new embedders get smaller host crates and feature gates.

### 3. Pure planning stays testable

**Choice:** Deterministic config parsing, context-resolution planning, provider selection planning, document/model-evidence ingest planning, and observability projection remain pure functions over explicit inputs. Shell code only performs I/O or calls injected services after those plans are admitted.

**Rationale:** This preserves the functional-core / imperative-shell contract while making each host adapter independently testable.

### 4. No ambient fallback

**Choice:** Missing host adapter crates or disabled features produce typed unavailable/denied outcomes instead of falling back to filesystem scans, env discovery, provider probing, network defaults, or storage creation.

**Rationale:** Lego blocks should be explicit. Absence of a block is absence of authority.

## Risks / Trade-offs

- More crates can complicate examples. Mitigate with catalog-generated import guidance and facade re-exports.
- Existing std tests may rely on facade paths. Mitigate with compatibility aliases and focused migration tests.
- Authority labels may be incomplete at first. Mitigate with negative fixtures for common ambient fallback attempts.
