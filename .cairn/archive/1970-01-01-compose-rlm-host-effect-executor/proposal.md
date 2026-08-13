# Proposal: Compose an RLM host-effect executor

## Why

Animus defines typed RLM host requests and an injected `RlmKernelHostExecutor`. Only test fakes implement the complete executor.

Concrete read-only queries, one-file edits, bounded processes, MCP calls, context resolution, and subagent contracts exist as separate units. No product host routes RLM requests through them.

A dedicated non-default composition must execute each admitted operation without moving authority into Steel or reusable runtime crates.

## What Changes

- Add a non-default authority-scoped host component that implements the complete RLM executor port. r[agent_harness_runtime.rlm_host_executor.boundary]
- Route each operation class to a narrow explicit capability pack after current admission. r[agent_harness_runtime.rlm_host_executor.dispatch]
- Add capability-relative file reads and reuse the revision-gated workspace edit host for mutation. r[agent_harness_runtime.rlm_host_executor.workspace]
- Execute shell requests through explicit command profiles and bounded process mechanics. r[agent_harness_runtime.rlm_host_executor.process]
- Route named tools, context reads, child requests, and kernel clears through their existing owners. r[agent_harness_runtime.rlm_host_executor.services]
- Add safe observations, failure translation, positive and negative conformance, and license isolation. r[agent_harness_runtime.rlm_host_executor.verification]

## Prerequisites

- `add-resumable-steel-host-bridge`
- `agent_harness_runtime.rlm_kernel.host_requests.admitted`
- `agent_harness_runtime.tool_executor_kit.admission`
- `agent_harness_runtime.hashed_edit_host.snapshot`
- `agent_harness_runtime.clankers_process_job_profiles.resolution`
- `agent_harness_runtime.clankers_context_references.resolvers`
- `agent_harness_runtime.subagent_coordinator.service`

## Impact

- **Crates**: a separate non-default std host crate and narrow adapter bindings.
- **License**: the host must use `AGPL-3.0-or-later` if it links existing AGPL process or workspace components.
- **Authority**: reusable and default crates gain no file, process, network, credential, persistence, or child authority.
- **Composition**: the host receives explicit capability roots, command profiles, tool bindings, context services, and child services.

## Out of Scope

- Ambient current-directory access, arbitrary shell strings, automatic tool discovery, hidden network access, or global service registries.
- Security sandboxing, command correctness, tool correctness, reversible mutation, task success, or production readiness.

## Affected Specs

- `agent-harness-runtime`: executor boundary, dispatch, workspace, process, service routing, and verification.
