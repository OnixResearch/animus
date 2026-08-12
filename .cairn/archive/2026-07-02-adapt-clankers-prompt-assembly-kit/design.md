# Design: Adapt Clankers prompt assembly kit for Animus

## Context

Prompt assembly is the seam where many independent features meet. Clankers' embedding boundary treats prompt assembly as host-owned inputs plus reusable deterministic composition. Animus should turn that into a central contract so policy profiles, skills, memory, context references, tools, and agent definitions remain attributable and bounded.

## Decisions

### 1. Prompt sources are typed in-memory inputs

**Choice:** Assembly consumes typed `PromptSource` values with source kind, scope, trust/admission status, priority, budget class, redaction class, and safe identity.

**Rationale:** Filesystem discovery and prompt text parsing are host concerns. The reusable core should compose already-authorized source descriptors.

### 2. Precedence is policy-owned

**Choice:** Nickel-authored or checked policy defines section order, source precedence, conflict behavior, required/optional sources, budget classes, and disabled-source behavior.

**Rationale:** Prompt ordering affects behavior and must be reviewable, deterministic, and testable.

### 3. Ambient discovery is off by default

**Choice:** Embeddable/default assembly uses host-context-only mode unless a host explicitly injects swappable discovery adapters and admitted sources. Memory sources are just one source kind; assembly never chooses a concrete memory store, vector index, importer, or provider extractor.

**Rationale:** Animus should not silently read project files, URLs, memories, or prompt snippets in embedded applications, and embedders should be able to replace memory backends without changing prompt assembly.

### 4. Assembly produces a ledger

**Choice:** Assembly returns model-visible sections plus a ledger of included, skipped, truncated, redacted, denied, conflicting, and unsupported sources.

**Rationale:** Operators need to understand why a model saw particular guidance without storing raw hidden context in receipts.

### 5. Receipts stay safe

**Choice:** Ledger/receipt metadata records source kind, safe label/hash, trust status, section, budget use, truncation status, conflict reason, policy hash, and BLAKE3 source body hash when needed. Raw prompt bodies, credentials, full paths, provider payloads, and hidden memory bodies are excluded.

**Rationale:** Prompt provenance should not become a transcript or secret store.

## Risks / Trade-offs

- An umbrella prompt assembly contract overlaps existing policy/guidance/memory changes. This change should unify those ledgers, not replace their domain-specific validation.
- Conflict behavior can become too complex. Start with deterministic precedence and explicit diagnostics.
- Redaction can make debugging harder. Preserve stable hashes and safe source labels.
