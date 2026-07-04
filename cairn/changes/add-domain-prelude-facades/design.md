# Design: Add domain prelude facades for composable APIs

## Context

The current facades are intentionally broad compatibility layers. They help consumers who want one import point, but they obscure which contracts belong together and which authority boundaries are involved. Domain preludes should be additive, documented, and validated against dependency leakage.

## Decisions

### 1. Preludes are additive curated modules

**Choice:** Add `*_prelude` modules for stable composition domains while preserving the existing flat exports.

**Rationale:** Additive preludes improve ergonomics without breaking compatibility.

### 2. Each prelude names its authority posture

**Choice:** Document every prelude as `reusable-contract`, `host-injection-surface`, or `shell-adapter` and mirror that classification in the facade inventory.

**Rationale:** A prelude should not hide authority. Embedders need to know whether importing it only brings DTOs/helpers or also host-owned services.

### 3. Preludes map to real workflow seams

**Choice:** Initial preludes cover loop execution, tool contracts, headless protocol, provenance, worker-team delegation, policy/middleware, status selection, and optional TUI bridge models.

**Rationale:** These are the places embedders are most likely to compose Animus with host runtimes.

### 4. Compile tests prove independence

**Choice:** Add small compile or integration tests that import each prelude and construct representative DTOs without bringing unrelated modules into scope.

**Rationale:** The value of a prelude is not just naming; it must maintain a small dependency and authority footprint.

### 5. Compatibility remains explicit

**Choice:** Existing root exports remain available and documented as compatibility facades. New docs recommend preludes for new embedder code.

**Rationale:** This avoids churn while guiding consumers toward clearer composition.

## Risks / Trade-offs

- Too many preludes can become another confusing taxonomy. Start with high-value domains and document selection guidance.
- Preludes can drift into dumping grounds. Facade inventory and dependency checks should prevent unrelated additions.
- Public API duplication can confuse docs. Cross-link compatibility exports to domain preludes rather than duplicating behavior descriptions.
