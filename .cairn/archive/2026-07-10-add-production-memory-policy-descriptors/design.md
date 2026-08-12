# Design: Add production memory policy descriptors

## Context

Production memory systems often combine hot caches, warm searchable stores, cold archives, graph layers, TTL policies, sharding, redaction, erasure, backup, observability, and cost controls. Animus should not own those concrete systems. It should own typed descriptors and safe receipts that let hosts prove what they configured or attempted without claiming more than the adapter evidence supports.

## Decisions

### 1. Tiering is descriptive

**Choice:** Add descriptors for memory tier labels, retention posture, TTL posture, promotion and demotion reasons, maintenance status, and declared surfaces.

**Rationale:** Hosts can expose Redis-like, SQL-like, vector-like, graph-like, file-like, or archive-like surfaces without making core depend on any one backend.

### 2. Erasure evidence is complete or explicitly partial

**Choice:** Right-to-forget planning and receipts must enumerate declared memory surfaces and return complete, partial, denied, stale, unavailable, or unsupported outcomes per surface.

**Rationale:** Unavailable caches, vector indexes, graph edges, or archives must not produce false deletion claims.

### 3. Costs and latency are bounded metadata

**Choice:** Memory operation evidence may record safe cost and latency budget labels, quota status, operation counts, and redaction classes, but concrete clocks, metrics exporters, dashboards, and billing systems stay host-owned.

**Rationale:** Product hosts need budget posture while reusable logic remains deterministic and pure.

### 4. Tenant isolation is policy-visible

**Choice:** Memory policy descriptors include tenant, user, project, thread, or host scope labels and isolation posture; validation rejects missing or unsafe isolation metadata for multi-tenant surfaces.

**Rationale:** Similar embeddings or shared graph nodes must never substitute for explicit scope filtering.

## Risks / Trade-offs

- Production descriptors can look like implementation guarantees. Receipts must distinguish declared posture from observed operation outcomes.
- Erasure across many surfaces can be hard to prove. Partial and unavailable states should be first-class.
- Cost and latency labels can drift from host reality. Hosts own measurement; Animus validates only safe evidence shape.
