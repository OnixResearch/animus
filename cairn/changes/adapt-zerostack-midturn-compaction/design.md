# Design: Adapt Zerostack mid-turn compaction for Animus

## Context

Zerostack's compaction logic accounts for provider-reported token usage and recognizes that a single tool-heavy turn can exceed context before between-turn compaction runs. Animus should adapt this into explicit ledger operations and injected service contracts.

## Decisions

### 1. Prompt pressure is measured separately from estimates

**Choice:** Track estimated replay tokens and provider-reported prompt pressure as distinct inputs, with provider route metadata for cached-token interpretation.

**Rationale:** Provider APIs report cached tokens differently. The runtime must avoid both undercounting and double-counting.

### 2. Mid-turn compaction creates a continuation plan

**Choice:** When pressure crosses policy threshold, runtime returns a stop/compact/resume plan that requires summarizer and ledger services.

**Rationale:** Continuation should be auditable and fail closed when required services are missing.

### 3. Summaries are ledger entries

**Choice:** Compaction appends explicit summary entries with bounded text and safe metadata instead of destructively rewriting replay history in memory.

**Rationale:** Ledger replay, provenance, and dataset export need durable context transitions.

### 4. Long tool outputs become artifacts

**Choice:** Tool results over policy limits are represented by bounded visible windows plus artifact descriptors and BLAKE3 hashes.

**Rationale:** Models need useful context, while receipts and ledgers must avoid unbounded or sensitive raw output.

## Risks

- Mid-turn compaction can interrupt work too often. Keep thresholds policy-controlled and record decisions in receipts.
- Provider usage metadata can be malformed. Treat malformed pressure data as unavailable and fail according to policy rather than guessing silently.
