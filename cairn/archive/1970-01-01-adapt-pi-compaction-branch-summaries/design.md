# Design: Adapt Pi compaction and branch summaries for Animus

## Context

Pi compaction selects a safe cut point, summarizes older messages, appends a compaction entry, and rebuilds context from summary plus kept messages. Pi branch summarization preserves abandoned branch context when navigating a session tree. Animus should adapt these mechanics over its neutral ledger and injected summarizer services.

## Decisions

### 1. Compaction preparation is pure

**Choice:** Select candidate spans, cut points, kept ranges, split-turn prefixes, previous summaries, and summary metadata from an in-memory ledger branch and policy input.

**Rationale:** The hard logic is deterministic and should be testable without providers, files, clocks, or UI.

### 2. Cut points preserve tool-call/result semantics

**Choice:** Valid cut points must not leave tool results without their corresponding tool calls or produce model-visible contexts that violate adapter message ordering.

**Rationale:** Broken transcript shape can confuse models and make replay invalid.

### 3. Split turns are summarized explicitly

**Choice:** If one turn exceeds the kept-recent budget, the early prefix of that turn is summarized separately from prior history and represented in the resulting summary entry.

**Rationale:** Large tool-heavy turns are common. The runtime needs a controlled way to compact them without pretending a clean user-turn boundary exists.

### 4. Summaries carry safe cumulative metadata

**Choice:** Summary entries include bounded summary text, safe counts, BLAKE3 hashes for sensitive content identity, and cumulative read/modified resource metadata that is redacted according to policy.

**Rationale:** Future receipts and operator tools need continuity without raw transcript leakage.

### 5. Summarization is an injected service

**Choice:** Runtime prepares summarization requests, then calls an injected summarizer adapter/service. Core code must not construct providers or read credentials.

**Rationale:** Summarizer choice is a host concern and may use a model, deterministic summarizer, or human-authored summary.

## Risks / Trade-offs

- Compaction is lossy. Documentation and report fields must make that explicit.
- Token estimates vary by provider. Use policy-level budget units and allow provider-specific shells to supply estimates.
- Summary metadata can leak paths or tool bodies. Reuse receipt redaction policy and safe resource identifiers.
