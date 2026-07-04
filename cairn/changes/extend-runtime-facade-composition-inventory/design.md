# Design: Extend runtime facade inventory for composition metadata

## Context

`docs/runtime-facade-inventory.md` and `cairn/runtime-facade-inventory.ncl` already provide an architectural safety rail. The current data answers "what class is this export?" Composability also needs answers to "which feature exposes it?", "what authority does it require?", "what does it depend on?", and "which surfaces is it intended to compose with?"

## Decisions

### 1. Extend records with typed composition fields

**Choice:** Add inventory fields for feature gate, required authority, dependency links, and expected composition links.

**Rationale:** These fields make architecture review explicit and checkable.

### 2. Keep authority values constrained

**Choice:** Use a small controlled vocabulary for authority requirements such as `none`, `injected-service`, `shell-owned`, `terminal-owned`, `process-owned`, `persistence-owned`, or `network-owned`.

**Rationale:** Free-form authority text would not support reliable drift checks.

### 3. Validate dependency direction

**Choice:** Inventory validation rejects reusable contracts that depend on shell adapters and rejects feature-gated exports whose feature label is missing or inconsistent.

**Rationale:** The main composability risk is hidden upward dependency from reusable surfaces into host authority.

### 4. Composition links are advisory but checked

**Choice:** `composes_with` links must point to known export groups or domains, but they do not force a dependency unless also listed in `depends_on`.

**Rationale:** Embedders need discovery guidance without creating artificial coupling.

### 5. Evidence remains bounded

**Choice:** Inventory checks print counts, class summaries, feature summaries, and BLAKE3 content hashes, not full source paths or raw generated data dumps.

**Rationale:** Inventory evidence should remain safe for Cairn closeout and release review.

## Risks / Trade-offs

- Extending every record can be tedious. Allow default values for obvious stable reusable contracts, but require explicit values for new or changed exports.
- Composition links can become stale. Validate link targets and include docs guidance for maintainers.
- Too much metadata can block routine changes. Start with fields that catch real drift and make stricter checks incremental.
