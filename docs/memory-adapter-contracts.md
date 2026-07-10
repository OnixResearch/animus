# Memory adapter contracts

Animus memory is an optional, host-injected adapter family. Reusable code consumes typed DTOs, normalized records, pure retrieval plans, safe control requests, and redacted receipts. It does not discover local files, open databases, call MCP servers, invoke providers, scan vector indexes, or persist generated memories by itself.

## Bounded reference

Agent Memory Techniques is a bounded reference for memory taxonomy, retrieval-quality patterns, evaluation rails, and production-memory policy vocabulary. Animus-owned DTOs, conformance manifests, and receipts remain canonical. The repo does not adopt the reference project's Python notebooks, framework dependencies, managed services, provider calls, benchmark datasets, prompt text, or concrete storage implementations.

## Generic adapter family

The generic memory family is split by role: source/import, durable store, retrieval/search, extraction, embedding/search support, admission, update/delete, and instruction-source behavior. `evaluate_memory_adapter_operation` checks that each required role is explicitly injected, available, and capability-compatible before any dispatch is allowed. Missing, disabled, unavailable, or unsupported adapters fail closed without ambient fallback.

## Cognitive taxonomy

`MemoryRecordCategory` classifies safe memory metadata as episodic, semantic, procedural, reflection, pitfall, instruction, fact, preference, summary, candidate, or unsupported. Existing `GenericMemoryRecordKind` values map deterministically into the category taxonomy so older hosts remain compatible. Hosts that require explicit category admission can require a category label; missing or unknown labels fail before retrieval, prompt influence, or receipts.

Categories are non-authoritative metadata. They may help routing, retrieval ordering, compaction, evaluation, and explanations, but admission, scope, freshness, redaction class, prompt budget, and receipt leak checks still decide whether a record can influence a model.

## Memory routing

`MemoryRouteCandidate`, `MemoryRoutePolicy`, `MemoryRoutePlan`, and `MemoryRouteReceipt` describe adapter selection for read/search, source/import, extraction, durable store, update/delete, and prompt-influence intents. Route planning is pure over safe route IDs, adapter labels, category hints, scopes, adapter roles, availability, capability support, confidence, freshness, budgets, and policy hashes. It does not call classifiers, stores, embedding services, MCP servers, providers, filesystems, clocks, or networks.

Routing fails closed for denied policy, disallowed scope, stale candidates when stale routes are disabled, unavailable adapters, unsupported capabilities, over-budget plans, unsafe labels, and receipt leaks. A selected route only names where a host shell may dispatch after normal adapter admission; it never grants storage, retrieval, prompt visibility, or mutation authority by itself.

## Evaluation rail

`MemoryEvaluationFixture` records safe query labels, expected record IDs, scope, category, abstention expectations, answer-use labels, and BLAKE3 evidence hashes. `reduce_memory_evaluation_metrics` computes deterministic Precision@K, Recall@K, stale-hit count, contradiction-hit count, answer-use count, and abstention correctness over safe observed ledgers.

Memory-quality claims require `MemoryEvaluationBaseline` evidence for no-memory or previous-policy behavior before `MemoryQualityClaim` validates. `MemoryExternalBenchmarkImport` can represent host-owned LoCoMO-like, LongMemEval-like, or private benchmark summaries with safe metric labels, counts, hashes, and policy identity while rejecting raw conversations, judge prompts, provider payloads, credentials, and datasets.

Normalized memory crosses reusable boundaries as `MemoryRecordEnvelope`. Envelopes require safe identity, kind, scope, source label, evidence handle, admission status, freshness, confidence, redaction class, BLAKE3 body hash, prompt budget, and optional expiration, supersession, and conflict metadata. Raw bodies, hidden bodies, malformed hashes, missing evidence, invalid confidence, zero budgets, and expired records are rejected before retrieval or prompt influence.

`plan_memory_retrieval` is a pure deterministic planner over already-loaded envelopes, adapter-declared match reasons, policy scopes, freshness, conflicts, and prompt budgets. Backend labels are validated as safe metadata but are not used for model-visible ordering.

`MemoryRetrievalQualityPolicy` adds declarative quality knobs for manual-pin priority, category priority, and diversity group limits without giving backend identity ranking authority. `MemoryRerankEvidence` can carry host-owned bounded rerank scores and safe reason labels when its policy hash matches the quality policy. `plan_memory_retrieval_with_quality` remains pure and deterministic over normalized records, safe match reasons, category metadata, rerank evidence, diversity groups, conflicts, and budgets.

## Control surface

Memory controls are adapter-neutral DTOs for list, show, search, why, approve, reject, forget, update, supersede, and safe export summaries. `MemoryControlRequest::validate` checks scope, capability, remote admission, target freshness, required evidence, and safe output budget before any adapter mutation, remote call, or prompt-visible use.

Candidate review uses explicit states: pending, approved, rejected, expired, superseded, unavailable, and persisted. Approval and persistence require scope, evidence, confidence, and admitted status; rejection records a safe reason and does not authorize storage or prompt injection.

Forget, update, tombstone, and supersession receipts use typed complete, partial, denied, stale, unavailable, and unsupported statuses. Unavailable or partial outcomes do not claim deletion. `explain_memory_influence` derives `why` output from retrieval ledgers using only safe IDs, ranks, match reasons, scope, confidence, consumed budget, and BLAKE3 hashes.

## Conformance rail

The conformance matrix tracks backend family, adapter role, capabilities, owner, applicability, positive case, negative case, and safe evidence label. Applicable rows must include both positive and negative coverage; not-applicable rows need a safe reason.

Null/no-op memory is first-class and plans no storage, retrieval, prompt injection, extraction, update, or deletion. File-backed memory remains a host-owned shell that passes only safe labels and BLAKE3 hashes into reusable evidence. Semantic backends expose normalized records and match reasons; unavailable vector or hybrid services do not trigger keyword, provider, network, or ambient-index fallback unless an explicit alternate adapter is injected and admitted. MCP/remote failure receipts use scoped service descriptors and redacted IDs without credentials, provider payloads, transport envelopes, or unbounded remote output.

The shared memory failure taxonomy covers unavailable, denied, timed-out, malformed-record, stale, over-budget, redacted, conflicting, unsafe-receipt, and leak diagnostics.
