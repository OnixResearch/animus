# Proposal: Add a resumable Steel host bridge

## Why

The persistent Steel kernel can return one admitted host-request marker. The normal tool adapter converts that marker to model-visible text.

This behavior does not execute the admitted request. It also cannot return a host result to the same Steel program.

Animus needs a resumable bridge before Steel can replace Prime Agent's callable IPython environment. The bridge must preserve host authority and cell-state atomicity.

## What Changes

- Add a host-injected Steel bridge that can process bounded host calls and return correlated results to the active Steel cell. r[agent_harness_runtime.steel_host_bridge.continuation]
- Separate pure request admission from host execution and continuation delivery. r[agent_harness_runtime.steel_host_bridge.authority]
- Commit persistent kernel state only after the Steel cell reaches a successful final value. r[agent_harness_runtime.steel_host_bridge.commit_boundary]
- Record prior external effects honestly when a later continuation or Steel expression fails. r[agent_harness_runtime.steel_host_bridge.partial_effects]
- Add finite call, result, fuel, time, cancellation, and nesting bounds. r[agent_harness_runtime.steel_host_bridge.bounds]
- Add positive and negative conformance tests through the production bridge entrypoint. r[agent_harness_runtime.steel_host_bridge.verification]

## Prerequisites

- `agent_harness_runtime.rlm_kernel.environment`
- `agent_harness_runtime.rlm_kernel.host_requests`
- `agent_harness_runtime.rlm_kernel.host_requests.admitted`
- `agent_harness_runtime.tool_executor_kit.admission`
- `agent_harness_runtime.coding_session_host.durable_boundaries`

## Impact

- **Crates**: `animus-steel`, `animus-runtime`, and the RLM adapter contract.
- **Architecture**: Steel remains effect-free without an injected bridge. Rust retains admission and execution authority.
- **Compatibility**: The current marker-only adapter remains available until callers migrate.
- **Evidence**: Receipts contain safe correlation, class, count, status, and BLAKE3 identity fields only.

## Out of Scope

- Durable restore of a kernel after process exit.
- Concrete file, process, tool, context, or child services.
- Security sandboxing, effect rollback, exactly-once external effects, task correctness, or production readiness.

## Affected Specs

- `agent-harness-runtime`: resumable Steel host calls, authority, commit boundaries, partial effects, bounds, and verification.
