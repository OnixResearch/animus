# Proposal: Add a concrete child-session service

## Why

Animus exposes typed subagent drafts, admission, lifecycle, lineage, fan-out, and a `SubagentService` port. Current runtime paths use injected fakes.

A Prime Agent-style `rlm(...)` call needs a host service that creates a normal child coding session, retains its handle, and returns a bounded parent result.

The service must inherit only admitted capabilities and budgets. It must not become an ambient agent factory.

## What Changes

- Add a non-default child-session host that implements enqueue, status, cancel, and collect over real coding-session instances. r[agent_harness_runtime.child_session_service.lifecycle]
- Add an explicit session factory with injected provider, tool, persistence, context, and event services. r[agent_harness_runtime.child_session_service.factory]
- Enforce parent, profile, context, capability, workspace, model, tool, depth, fan-out, time, token, and result limits before creation. r[agent_harness_runtime.child_session_service.authority]
- Persist safe child lifecycle and lineage facts while keeping private child content in the child session's private channel. r[agent_harness_runtime.child_session_service.persistence]
- Return bounded typed child results to the parent and preserve cancellation and partial outcomes. r[agent_harness_runtime.child_session_service.results]
- Add positive and negative lifecycle, restart, isolation, and leak tests. r[agent_harness_runtime.child_session_service.verification]

## Prerequisites

- `agent_harness_runtime.subagent_coordinator.service`
- `agent_harness_runtime.subagent_typed_capabilities.intersection`
- `agent_harness_runtime.subagent_context_packets.identity`
- `agent_harness_runtime.subagent_lineage_ledger.record`
- `agent_harness_runtime.coding_session_host.facade`
- `agent_harness_runtime.durable_session_store.boundary`
- `agent_harness_runtime.rlm_kernel.children`

## Impact

- **Crates**: a separate non-default child-session host plus runtime adapter bindings.
- **Composition**: callers inject a session factory and all external services.
- **Persistence**: each child has an explicit session scope and durable lifecycle records.
- **Authority**: the child receives intersections, never ambient inheritance.

## Out of Scope

- Automatic delegation, unbounded recursion, global child registries, cross-tenant discovery, remote workers, or child workspace mutation without separate authority.
- Provider quality, child correctness, task success, exactly-once cancellation, or production readiness.

## Affected Specs

- `agent-harness-runtime`: child lifecycle, factory, authority, persistence, results, and verification.
