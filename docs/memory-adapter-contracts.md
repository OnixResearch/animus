# Memory adapter contracts

Animus memory is an optional, host-injected adapter family. Reusable code consumes typed DTOs, normalized records, pure retrieval plans, safe control requests, and redacted receipts. It does not discover local files, open databases, call MCP servers, invoke providers, scan vector indexes, or persist generated memories by itself.

## Generic adapter family

The generic memory family is split by role: source/import, durable store, retrieval/search, extraction, embedding/search support, admission, update/delete, and instruction-source behavior. `evaluate_memory_adapter_operation` checks that each required role is explicitly injected, available, and capability-compatible before any dispatch is allowed. Missing, disabled, unavailable, or unsupported adapters fail closed without ambient fallback.

Normalized memory crosses reusable boundaries as `MemoryRecordEnvelope`. Envelopes require safe identity, kind, scope, source label, evidence handle, admission status, freshness, confidence, redaction class, BLAKE3 body hash, prompt budget, and optional expiration, supersession, and conflict metadata. Raw bodies, hidden bodies, malformed hashes, missing evidence, invalid confidence, zero budgets, and expired records are rejected before retrieval or prompt influence.

`plan_memory_retrieval` is a pure deterministic planner over already-loaded envelopes, adapter-declared match reasons, policy scopes, freshness, conflicts, and prompt budgets. Backend labels are validated as safe metadata but are not used for model-visible ordering.

## Control surface

Memory controls are adapter-neutral DTOs for list, show, search, why, approve, reject, forget, update, supersede, and safe export summaries. `MemoryControlRequest::validate` checks scope, capability, remote admission, target freshness, required evidence, and safe output budget before any adapter mutation, remote call, or prompt-visible use.

Candidate review uses explicit states: pending, approved, rejected, expired, superseded, unavailable, and persisted. Approval and persistence require scope, evidence, confidence, and admitted status; rejection records a safe reason and does not authorize storage or prompt injection.

Forget, update, tombstone, and supersession receipts use typed complete, partial, denied, stale, unavailable, and unsupported statuses. Unavailable or partial outcomes do not claim deletion. `explain_memory_influence` derives `why` output from retrieval ledgers using only safe IDs, ranks, match reasons, scope, confidence, consumed budget, and BLAKE3 hashes.

## Conformance rail

The conformance matrix tracks backend family, adapter role, capabilities, owner, applicability, positive case, negative case, and safe evidence label. Applicable rows must include both positive and negative coverage; not-applicable rows need a safe reason.

Null/no-op memory is first-class and plans no storage, retrieval, prompt injection, extraction, update, or deletion. File-backed memory remains a host-owned shell that passes only safe labels and BLAKE3 hashes into reusable evidence. Semantic backends expose normalized records and match reasons; unavailable vector or hybrid services do not trigger keyword, provider, network, or ambient-index fallback unless an explicit alternate adapter is injected and admitted. MCP/remote failure receipts use scoped service descriptors and redacted IDs without credentials, provider payloads, transport envelopes, or unbounded remote output.

The shared memory failure taxonomy covers unavailable, denied, timed-out, malformed-record, stale, over-budget, redacted, conflicting, unsafe-receipt, and leak diagnostics.
