# Production memory policy descriptors

Production memory posture is metadata, not backend authority. Descriptors name declared surfaces, tier labels, retention posture, TTL posture, promotion/demotion reasons, backup posture, maintenance state, isolation scope, budgets, and erasure receipt outcomes.

Reusable logic validates tier descriptors, per-surface erasure outcomes, safe budget descriptors, and tenant isolation. Partial, denied, stale, unavailable, and unsupported erasure outcomes cannot be promoted to complete deletion claims.

Non-claims: Redis, SQL, vector stores, graph stores, file archives, backup systems, billing, clocks, metrics exporters, dashboards, alerting, prompt influence, deletion execution, search execution, and export execution remain host-owned.
