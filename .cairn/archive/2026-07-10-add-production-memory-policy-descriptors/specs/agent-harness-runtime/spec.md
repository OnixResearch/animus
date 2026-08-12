# agent-harness-runtime Change Spec: Production memory policy descriptors

## Requirements

### Requirement: Production memory tier descriptors

r[agent_harness_runtime.memory_production_policy.tiers] Production memory posture MUST be represented as typed descriptors for declared memory surfaces, tier labels, retention posture, TTL posture, promotion and demotion reasons, backup posture, and maintenance status, and reusable Animus logic MUST NOT construct or operate concrete cache, database, vector, graph, file, archive, or backup systems.

#### Scenario: Tier descriptor names posture without backend authority

r[agent_harness_runtime.memory_production_policy.tiers.descriptive]
- GIVEN a host declares hot cache, warm search, and cold archive memory surfaces
- WHEN production memory policy validation runs
- THEN Animus records safe tier labels, retention posture, TTL posture, and maintenance status
- AND no Redis, SQL, vector, graph, file, archive, or backup operation is dispatched by reusable logic.

### Requirement: Erasure evidence distinguishes complete and partial outcomes

r[agent_harness_runtime.memory_production_policy.erasure] Memory erasure planning and receipts MUST enumerate declared memory surfaces and record complete, partial, denied, stale, unavailable, or unsupported outcomes per surface without claiming deletion for unavailable or unverified surfaces.

#### Scenario: Unavailable vector surface prevents complete erasure claim

r[agent_harness_runtime.memory_production_policy.erasure.partial]
- GIVEN a user erasure request covers raw records, summaries, embeddings, graph relationships, cache entries, and archives
- AND the embedding surface is unavailable
- WHEN erasure receipt validation runs
- THEN the receipt records a partial or unavailable outcome for that surface
- AND no report claims complete right-to-forget completion.

### Requirement: Cost and latency budget descriptors

r[agent_harness_runtime.memory_production_policy.budgets] Memory operation evidence MAY include safe cost budget, latency budget, quota, redaction, and operation-count descriptors, but clocks, billing systems, metrics exporters, dashboards, and alerting remain host-owned and MUST NOT be required by reusable core logic.

#### Scenario: Budget evidence remains safe metadata

r[agent_harness_runtime.memory_production_policy.budgets.safe]
- GIVEN a host reports memory retrieval and extraction budget posture
- WHEN budget descriptor validation runs
- THEN safe budget labels, quota status, operation counts, and redaction classes are accepted
- AND raw billing data, credentials, provider payloads, unbounded logs, and full paths are rejected.

### Requirement: Tenant isolation descriptors

r[agent_harness_runtime.memory_production_policy.tenant_isolation] Multi-tenant memory surfaces MUST declare scope and isolation posture for user, project, thread, tenant, or host boundaries, and retrieval or erasure planning MUST fail closed when required isolation metadata is missing or unsafe.

#### Scenario: Missing tenant scope blocks shared-memory operation

r[agent_harness_runtime.memory_production_policy.tenant_isolation.missing]
- GIVEN a shared memory surface is marked multi-tenant
- AND a retrieval or erasure operation lacks required tenant or user scope metadata
- WHEN production memory policy validation runs
- THEN validation rejects the operation with a typed isolation diagnostic
- AND no prompt influence, deletion, search, or export operation is planned.

### Requirement: Production memory policy verification

r[agent_harness_runtime.memory_production_policy.verification] The production memory policy adaptation MUST include focused positive and negative tests for tier descriptors, erasure completeness, partial and unavailable deletion outcomes, budget descriptors, tenant isolation, unsafe observability labels, stale receipts, no backend authority, and leak rejection.

#### Scenario: Closeout evidence covers production policy fixtures

r[agent_harness_runtime.memory_production_policy.verification.closeout]
- GIVEN maintainers intend to archive the production memory policy adaptation
- WHEN closeout validation runs
- THEN evidence names focused tier, erasure, budget, tenant-isolation, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
