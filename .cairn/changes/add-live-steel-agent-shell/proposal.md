# Proposal: Add a live Steel agent shell

## Why

Animus has fake-backed coding sessions, a text-only live provider host, a persistent Steel kernel, separate tool hosts, durable sessions, and runtime events.

No composition root connects a tool-capable provider to `steel_repl`, resumable host effects, child sessions, persistence, and provider continuation.

This missing shell is the smallest unit that can run a Prime Agent-style Steel agent end to end.

## What Changes

- Add a non-default headless application that selects all concrete adapters at one visible composition root. r[agent_harness_runtime.live_steel_agent_shell.composition]
- Run the provider, `steel_repl`, host-effect, child-session, and provider-continuation loop through normal coding-session semantics. r[agent_harness_runtime.live_steel_agent_shell.tool_loop]
- Require typed Nickel configuration and explicit capability roots, profiles, services, credentials, bounds, and non-claims. r[agent_harness_runtime.live_steel_agent_shell.configuration]
- Preserve durable message, tool, kernel, queue, event, cancellation, and recovery ordering. r[agent_harness_runtime.live_steel_agent_shell.session]
- Add bounded startup, shutdown, cancellation, and degraded recovery behavior. r[agent_harness_runtime.live_steel_agent_shell.lifecycle]
- Add a complete deterministic end-to-end fixture and separately scoped optional live evidence. r[agent_harness_runtime.live_steel_agent_shell.verification]

## Prerequisites

- `add-resumable-steel-host-bridge`
- `add-durable-steel-kernel-restore`
- `add-tool-capable-openai-provider-host`
- `compose-rlm-host-effect-executor`
- `add-concrete-child-session-service`
- `add-steel-native-skill-modules`
- `add-coding-session-queue-cas`
- `agent_harness_runtime.coding_session_host.conformance`
- `agent_harness_runtime.runtime_authority_seal.monotonic`

## Impact

- **Product surface**: one explicit non-default headless Steel agent binary or package.
- **Licensing**: the shell must retain the license requirements of its selected process and workspace host components.
- **Configuration**: Nickel is the human-authored source. Exported runtime configuration is validated before startup.
- **Evidence**: deterministic fixture evidence and optional live-provider evidence remain separate.

## Out of Scope

- Daemon supervision, remote attach, live terminal ownership, provider discovery, OAuth, global session stores, automatic skill installation, or release automation.
- General provider parity, sandboxing, task correctness, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: composition, tool loop, configuration, session ordering, lifecycle, and verification.
