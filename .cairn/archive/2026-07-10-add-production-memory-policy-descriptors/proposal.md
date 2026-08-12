# Proposal: Add production memory policy descriptors

## Why

The current memory contracts focus on safe adapter boundaries, prompt influence, controls, and conformance. Production memory references add operational concerns that should be visible before Animus hosts scale memory: tiering, TTL and retention, right-to-forget completeness, tenant isolation, cost budgets, latency budgets, redaction gates, backups, and partial-deletion receipts.

Animus should model these as typed policy and evidence descriptors while leaving Redis, SQL, vector stores, graph databases, archives, PII scanners, backups, and observability backends to host adapters.

## What Changes

- Define production memory policy descriptors for hot/warm/cold tier labels, retention posture, TTL posture, promotion/demotion reasons, backup posture, and maintenance status.
- Define privacy and erasure evidence descriptors that track complete, partial, denied, stale, unavailable, and unsupported deletion outcomes across declared memory surfaces.
- Define cost, latency, quota, redaction, and tenant-isolation descriptors for memory operations without owning concrete metrics exporters or storage systems.
- Add focused positive and negative fixtures for policy validation, erasure completeness, partial deletion, tenant leaks, unsafe observability labels, and no concrete backend authority.

## Impact

- **Files**: future policy DTOs, erasure/evidence reducers, docs, tests, conformance fixtures, and Cairn artifacts.
- **Testing**: positive production-policy fixtures and negative partial-erasure, tenant-leak, unsafe-label, stale-receipt, unavailable-backend, and raw-body fixtures.
- **Non-claims**: this does not add Redis, pgvector, graph databases, archives, backup jobs, dashboards, PII services, or production scaling claims.
