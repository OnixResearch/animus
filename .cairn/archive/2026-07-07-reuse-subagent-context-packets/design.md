# Design: Reuse context packets across subagent kinds

Animus should make context minimization a shared primitive. The existing worker-team context packet is the best starting point because it already distinguishes safe-by-default items from unsafe classes.

## Decisions

### 1. Shared packet owns minimization rules

**Choice:** A reusable packet type represents selected parent message excerpts, summaries, file observations, memory descriptors, bounded tool-result summaries, and admitted context references.

**Rationale:** Every child-run kind needs the same safe context vocabulary.

### 2. Unsafe context requires explicit admission

**Choice:** Full transcripts, raw hidden memory, credentials, provider-native payloads, and unbounded tool output are denied unless policy explicitly admits the class and text leak validation passes.

**Rationale:** Shared packets should improve safety, not create a bypass around existing worker-team checks.

### 3. Packet identity is deterministic

**Choice:** Context packets carry BLAKE3 identities over stable, redacted packet content and budget metadata.

**Rationale:** Parent/child ledger entries and receipts need replayable references without storing raw prompt or output bodies.

### 4. Existing child-run kinds adapt through shims first

**Choice:** Existing validators initially project their context references into the shared packet shape rather than changing all call sites at once.

**Rationale:** This keeps the migration incremental and testable.
