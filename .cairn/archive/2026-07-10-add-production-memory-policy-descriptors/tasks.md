# Tasks

- [x] [serial] Define production memory policy descriptors for tier labels, retention posture, TTL posture, promotion/demotion reasons, backup posture, declared surfaces, and maintenance status. r[agent_harness_runtime.memory_production_policy.tiers]
- [x] [depends:tiers] Define erasure planning and receipt DTOs that enumerate declared memory surfaces and preserve complete, partial, denied, stale, unavailable, and unsupported outcomes. r[agent_harness_runtime.memory_production_policy.erasure]
- [x] [depends:tiers] Define safe cost, latency, quota, redaction, and operation-count descriptors for memory operations without owning clocks, exporters, dashboards, or billing systems. r[agent_harness_runtime.memory_production_policy.budgets]
- [x] [depends:tiers] Add tenant-isolation descriptors and validation for multi-tenant memory surfaces. r[agent_harness_runtime.memory_production_policy.tenant_isolation]
- [x] [depends:erasure,budgets,tenant_isolation] Add positive production-policy tests and negative partial-erasure, tenant-leak, unsafe-label, stale-receipt, unavailable-backend, and raw-body fixtures. r[agent_harness_runtime.memory_production_policy.verification]
- [x] [depends:verification] Update docs and run focused production-memory policy tests, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_production_policy.verification]
